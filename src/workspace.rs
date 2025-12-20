use anyhow::{Context, Result};
use console::style;
use glob::glob;
use std::fs;
use std::path::{Path, PathBuf};
use crate::manifest::PackageJson;

#[derive(Debug, Clone)]
pub struct Workspace {
    pub name: String,
    pub path: PathBuf,
    pub _package_json: PackageJson,
}

/// Find all workspaces based on the patterns in root package.json
pub fn find_workspaces(root: &Path) -> Result<Vec<Workspace>> {
    let root_pkg_path = root.join("package.json");
    let mut content = fs::read_to_string(&root_pkg_path)
        .context("Failed to read root package.json")?;
        
    if content.starts_with('\u{FEFF}') {
        content = content.trim_start_matches('\u{FEFF}').to_string();
    }
    
    let pkg: PackageJson = serde_json::from_str(&content)?;

    let mut workspaces = Vec::new();

    if let Some(patterns) = pkg.workspaces {
        for pattern in patterns {
            // Pattern e.g. "packages/*"
            // We need to look for package.json inside matches
            let full_pattern = root.join(&pattern).join("package.json");
            let pattern_str = full_pattern.to_string_lossy();

            for entry in glob(&pattern_str).expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => {
                        // path is .../packages/a/package.json
                        let pkg_dir = path.parent().unwrap().to_path_buf();
                        
                        // Load the workspace package.json
                        let mut content = fs::read_to_string(&path)?;
                        
                        // Debug prints
                        // println!("DEBUG: Reading {}", path.display());
                        // println!("DEBUG: First bytes: {:?}", content.as_bytes().iter().take(5).collect::<Vec<_>>());
                        
                        if content.starts_with('\u{FEFF}') {
                            content = content.trim_start_matches('\u{FEFF}').to_string();
                        }
                        
                        let ws_pkg: PackageJson = match serde_json::from_str(&content) {
                            Ok(p) => p,
                            Err(e) => {
                                println!("{} Failed to parse {}: {}", style("❌").red(), path.display(), e);
                                println!("First 10 chars: {:?}", content.chars().take(10).collect::<String>());
                                continue;
                            }
                        };
                        
                        workspaces.push(Workspace {
                            name: ws_pkg.name.clone(),
                            path: pkg_dir,
                            _package_json: ws_pkg,
                        });
                    }
                    Err(e) => println!("{} Error reading workspace glob: {}", style("⚠️").yellow(), e),
                }
            }
        }
    }

    Ok(workspaces)
}

/// Link all workspaces to the root node_modules so they can resolve each other
pub fn link_workspaces(root: &Path, workspaces: &[Workspace]) -> Result<()> {
    let node_modules = root.join("node_modules");
    if !node_modules.exists() {
        fs::create_dir_all(&node_modules)?;
    }

    for ws in workspaces {
        let target_link = node_modules.join(&ws.name);
        
        // Remove existing link/dir if present
        if target_link.exists() {
            // Simple remove, handles symlinks
             #[cfg(target_os = "windows")]
             {
                 if target_link.is_symlink() || target_link.is_dir() {
                     // remove_dir_all works on symlinks to directories in Rust std lib? 
                     // Actually, remove_dir_all follows symlinks sometimes, be careful.
                     // Safe for symlink: fs::remove_dir if it's a directory junction/symlink
                     // But std::fs::remove_dir requires empty directory.
                     // best to try remove_file (if it's a file-like symlink) or remove_dir_all
                     // Using crate::package_utils logic or simple attempt
                      let _ = fs::remove_dir_all(&target_link); 
                      // if unique file (symlink), remove_file
                      let _ = fs::remove_file(&target_link);
                 }
             }
             #[cfg(not(target_os = "windows"))]
             {
                 let _ = fs::remove_dir_all(&target_link);
                 let _ = fs::remove_file(&target_link);
             }
        }
        
        // Ensure parent dir exists (for scoped packages @foo/bar)
        if let Some(parent) = target_link.parent() {
            fs::create_dir_all(parent)?;
        }
        
        println!("   Linking workspace {} -> {}", style(&ws.name).cyan(), ws.path.display());

        #[cfg(target_os = "windows")]
        {
            if let Err(e) = std::os::windows::fs::symlink_dir(&ws.path, &target_link) {
                 // Error 1314: A required privilege is not held by the client.
                 if e.raw_os_error() == Some(1314) {
                     println!("   {} Symlink failed, trying junction...", style("⚠️").yellow());
                     let status = std::process::Command::new("cmd")
                        .args(&["/C", "mklink", "/J", target_link.to_str().unwrap(), ws.path.to_str().unwrap()])
                        .output()?; // Use output to suppress "Junction created for..." message or handle stdout
                     
                     if !status.status.success() {
                         return Err(anyhow::anyhow!("Failed to create junction for {}: {}", ws.name, String::from_utf8_lossy(&status.stderr)));
                     }
                 } else {
                     return Err(anyhow::anyhow!("Failed to link workspace {}: {}", ws.name, e));
                 }
            }
        }

        #[cfg(not(target_os = "windows"))]
        std::os::unix::fs::symlink(&ws.path, &target_link)
             .context(format!("Failed to link workspace {}", ws.name))?;
    }

    Ok(())
}
