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
}

impl PackageJson {
    pub fn load() -> Result<Self> {
        if !Path::new("package.json").exists() {
           return Ok(Self::default());
        }
        let content = fs::read_to_string("package.json")?;
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
    
    pub fn remove_dependency(&mut self, name: &str) -> Option<String> {
        self.dependencies.remove(name)
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
