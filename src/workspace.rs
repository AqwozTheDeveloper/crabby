use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use glob::glob;

#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    pub root: PathBuf,
    pub packages: Vec<WorkspacePackage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspacePackage {
    pub name: String,
    pub path: PathBuf,
    pub dependencies: HashMap<String, String>,
}

impl Workspace {
    /// Discover workspaces from package.json
    pub fn discover() -> Result<Option<Self>> {
        let pkg = crate::manifest::PackageJson::load()?;
        
        // Check if workspaces are defined
        if let Some(workspace_patterns) = pkg.workspaces {
            let mut packages = Vec::new();
            
            for pattern in workspace_patterns {
                for entry in glob(&pattern)? {
                    let path = entry?;
                    if path.is_dir() {
                        let pkg_json_path = path.join("package.json");
                        if pkg_json_path.exists() {
                            let content = std::fs::read_to_string(&pkg_json_path)?;
                            let workspace_pkg: crate::manifest::PackageJson = serde_json::from_str(&content)?;
                            
                            packages.push(WorkspacePackage {
                                name: workspace_pkg.name.clone(),
                                path: path.clone(),
                                dependencies: workspace_pkg.dependencies.clone(),
                            });
                        }
                    }
                }
            }
            
            Ok(Some(Workspace {
                root: std::env::current_dir()?,
                packages,
            }))
        } else {
            Ok(None)
        }
    }
    
    /// Link workspace packages using symlinks
    pub fn link_packages(&self) -> Result<()> {
        let node_modules = self.root.join("node_modules");
        std::fs::create_dir_all(&node_modules)?;
        
        for package in &self.packages {
            let link_path = node_modules.join(&package.name);
            
            // Remove existing link/dir if present
            if link_path.exists() {
                if link_path.is_symlink() {
                    std::fs::remove_file(&link_path)?;
                } else {
                    std::fs::remove_dir_all(&link_path)?;
                }
            }
            
            // Create symlink
            #[cfg(unix)]
            std::os::unix::fs::symlink(&package.path, &link_path)?;
            
            #[cfg(windows)]
            std::os::windows::fs::symlink_dir(&package.path, &link_path)?;
            
            println!("Linked {} -> {}", package.name, package.path.display());
        }
        
        Ok(())
    }
}
