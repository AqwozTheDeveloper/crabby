use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use anyhow::{Context, Result};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PackageJson {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub scripts: HashMap<String, String>,
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
    #[serde(default, rename = "devDependencies")]
    pub dev_dependencies: HashMap<String, String>,
    #[serde(default)]
    pub workspaces: Option<Vec<String>>,
}

impl PackageJson {
    pub fn load() -> Result<Self> {
        if !Path::new("package.json").exists() {
           return Ok(Self::default());
        }
        let mut content = fs::read_to_string("package.json")?;
        
        // Strip UTF-8 BOM if present (fixes PowerShell Out-File issue)
        if content.starts_with('\u{FEFF}') {
            content = content.trim_start_matches('\u{FEFF}').to_string();
        }
        
        let pkg: PackageJson = serde_json::from_str(&content)?;
        Ok(pkg)
    }

    pub fn save(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write("package.json", content)?;
        Ok(())
    }

    pub fn add_dependency(&mut self, name: String, version: String) {
        self.dependencies.insert(name, version);
    }
    
    pub fn add_dev_dependency(&mut self, name: String, version: String) {
        self.dev_dependencies.insert(name, version);
    }
    
    pub fn remove_dependency(&mut self, name: &str) -> Option<String> {
        self.dependencies.remove(name)
    }
    
    pub fn get_all_dependencies(&self) -> HashMap<String, String> {
        let mut all_deps = self.dependencies.clone();
        all_deps.extend(self.dev_dependencies.clone());
        all_deps
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CrabbyLock {
    pub dependencies: HashMap<String, LockDependency>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LockDependency {
    pub version: String,
    pub tarball: String,
}

impl CrabbyLock {
    pub fn load() -> Result<Self> {
        if !Path::new("crabby.lock").exists() {
            return Ok(Self::default());
        }
        let content = fs::read_to_string("crabby.lock")?;
        let lock: CrabbyLock = serde_json::from_str(&content)?;
        Ok(lock)
    }

    pub fn save(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write("crabby.lock", content)?;
        Ok(())
    }

    pub fn add_package(&mut self, name: String, version: String, tarball: String) {
        self.dependencies.insert(name, LockDependency { version, tarball });
    }
}

pub fn ensure_package_files() -> Result<()> {
    if !Path::new("package.json").exists() {
        let pkg = PackageJson {
            name: "my-crabby-project".to_string(),
            version: "1.0.0".to_string(),
            ..Default::default()
        };
        pkg.save().context("Failed to create package.json")?;
    }
    
    // We don't necessarily need to create an empty lockfile on init, but we can if we want to be explicit.
    // For now, let's leave it to be created on first install.
    Ok(())
}
