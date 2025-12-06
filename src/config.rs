use serde::{Deserialize, Serialize};
use std::fs;
use anyhow::{Context, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct CrabbyConfig {
    #[serde(default = "default_registry")]
    pub registry: String,
}

fn default_registry() -> String {
    "https://registry.npmjs.org".to_string()
}

impl Default for CrabbyConfig {
    fn default() -> Self {
        Self {
            registry: default_registry(),
        }
    }
}

impl CrabbyConfig {
    pub fn load() -> Result<Self> {
        // If config doesn't exist, return default without error
        if !std::path::Path::new("crabby.config.json").exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string("crabby.config.json")
            .context("Could not read crabby.config.json")?;
        
        // Try parsing. If it fails (maybe it's the old format with "scripts"), 
        // fallback to default to avoid breaking.
        let config: CrabbyConfig = match serde_json::from_str(&content) {
            Ok(c) => c,
            Err(_) => {
                // Potential future improvement: warn user if format is invalid
                Self::default()
            }
        };
        Ok(config)
    }
}

pub fn get_cache_dir() -> Result<std::path::PathBuf> {
    let cache_dir = if cfg!(target_os = "windows") {
        let local_app_data = std::env::var("LOCALAPPDATA")
            .context("LOCALAPPDATA environment variable not set")?;
        std::path::PathBuf::from(local_app_data).join("crabby").join("cache")
    } else {
        let home = std::env::var("HOME")
            .context("HOME environment variable not set")?;
        std::path::PathBuf::from(home).join(".cache").join("crabby")
    };
    
    // Create cache directory if it doesn't exist
    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir)?;
    }
    
    Ok(cache_dir)
}
