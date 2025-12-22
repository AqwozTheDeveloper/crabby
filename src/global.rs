use anyhow::{Context, Result};
use console::style;
use std::fs;
use std::path::{Path, PathBuf};
use crate::{manifest, package_utils, runner, registry, config, global};

/// Get global installation directory (~/.crabby/global)
pub fn get_global_dir() -> Result<PathBuf> {
    let home = dirs::home_dir()
        .context("Could not determine home directory")?;
    
    let global_dir = home.join(".crabby").join("global");
    if !global_dir.exists() {
        fs::create_dir_all(&global_dir)?;
        
        // Create basic package.json if strictly needed, but we might just use node_modules directly
        let pkg_json = global_dir.join("package.json");
        if !pkg_json.exists() {
             fs::write(&pkg_json, "{\"private\":true,\"dependencies\":{}}")?;
        }
    }
    
    Ok(global_dir)
}

/// Get global bin directory (~/.crabby/bin)
pub fn get_global_bin_dir() -> Result<PathBuf> {
    let home = dirs::home_dir()
        .context("Could not determine home directory")?;
    
    let bin_dir = home.join(".crabby").join("bin");
    if !bin_dir.exists() {
        fs::create_dir_all(&bin_dir)?;
    }
    
    Ok(bin_dir)
}

/// Install a package globally
pub async fn install_global(package: &str) -> Result<()> {
    let global_dir = get_global_dir()?;
    let bin_dir = get_global_bin_dir()?;
    let config = config::load_config()?;
    
    // We treat the global dir like a project with its own node_modules
    let node_modules = global_dir.join("node_modules");
    if !node_modules.exists() {
        fs::create_dir_all(&node_modules)?;
    }

    println!("{} Installing {} globally...", style("🌍").bold().blue(), package);
    println!("   Target: {}", style(global_dir.display()).dim());
    
    // Reuse package_utils::install_package logic but pointing to global dir
    let client = registry::get_client()?;
    
    // Strategy: Change Directory. since CLI is single-process, this is fine.
    let original_cwd = std::env::current_dir()?;
    std::env::set_current_dir(&global_dir)?;
    
    // Install package
    let mut lockfile = manifest::CrabbyLock::load().unwrap_or_default();
    let result = package_utils::install_package(package, &config.registry, &client, lockfile).await;
    
    // Restore CWD
    // We attempt to restore even if install failed, but trigger error if restore fails methods
    let restore_res = std::env::set_current_dir(original_cwd);
    
    match result {
        Ok((version, _tarball, updated_lock)) => {
            restore_res?;
            
            // Save lockfile (conceptually in global dir, but we changed back, so we need to be careful)
            // Wait, we are back in original CWD. 
            // We should save lockfile in global dir.
            // Actually install_package returns the updated lock struct. 
            // We should save it to global_dir/crabby.lock
            
            let lock_path = global_dir.join("crabby.lock");
            let content = serde_json::to_string_pretty(&updated_lock)?;
            fs::write(lock_path, content)?;

            // Link binaries to global bin
            link_global_binaries(package, &global_dir, &bin_dir)?;
            
            println!("{} Installed {} v{}", style("✅").green(), style(package).bold(), style(&version).dim());
            Ok(())
        },
        Err(e) => {
            let _ = restore_res; // Best effort restore
            Err(e)
        }
    }
}

pub async fn update_global(package: &str) -> Result<()> {
    println!("{} Updating global package {}...", style("🌍").bold().blue(), package);
    // Reuse install logic as it fetches latest matching version
    install_global(package).await
}

fn link_global_binaries(pkg_name: &str, global_dir: &Path, global_bin_dir: &Path) -> Result<()> {
    // Read the installed package.json from the global directory
    let pkg_path = global_dir.join("node_modules").join(pkg_name).join("package.json");
    
    if !pkg_path.exists() {
        println!("{} Warning: package.json not found at {}", style("⚠️").yellow(), style(pkg_path.display()).dim());
        return Ok(());
    }
    
    let content = fs::read_to_string(&pkg_path)?;
    let json: serde_json::Value = serde_json::from_str(&content)?;
    
    if let Some(bin) = json.get("bin") {
        if let Some(bin_map) = bin.as_object() {
            for (bin_name, script_path) in bin_map {
                if let Some(path_str) = script_path.as_str() {
                    create_global_shim(bin_name, pkg_name, path_str, global_bin_dir)?;
                }
            }
        } else if let Some(path_str) = bin.as_str() {
            // "bin": "./cli.js" -> name is package name
            create_global_shim(pkg_name, pkg_name, path_str, global_bin_dir)?;
        }
    }
    
    Ok(())
}

fn create_global_shim(bin_name: &str, pkg_name: &str, script_path: &str, global_bin_dir: &Path) -> Result<()> {
    // The target script path relative to the global node_modules
    // absolute path is global_modules / pkg / script
    let target_bin = global_bin_dir.join(bin_name);
    
    println!("   Linking bin: {} -> {}", bin_name, target_bin.display());
    
    // Windows: Create .cmd and shell shim
    #[cfg(target_os = "windows")]
    {
        // %~dp0 is the directory of the cmd file (global/bin)
        // We need to go ../global/node_modules/pkg/script
        // global/bin is sibling to global/global (where node_modules is)
        // structure: ~/.crabby/global/node_modules
        // structure: ~/.crabby/bin
        // So from bin, go ../global/node_modules
        
        // Shim content
        let cmd_content = format!(
            "@ECHO OFF\r\nnode \"%~dp0\\..\\global\\node_modules\\{}\\{}\" %*",
            pkg_name, script_path
        );
        fs::write(target_bin.with_extension("cmd"), cmd_content)?;
        
        // Also create bash shim for git bash
        let sh_content = format!(
            "#!/bin/sh\nexec node \"$0/../../global/node_modules/{}/{}\" \"$@\"",
            pkg_name, script_path
        );
         fs::write(&target_bin, sh_content)?;
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        use std::os::unix::fs::PermissionsExt;
        let sh_content = format!(
            "#!/bin/sh\nexec node \"$0/../../global/node_modules/{}/{}\" \"$@\"",
            pkg_name, script_path
        );
        fs::write(&target_bin, sh_content)?;
        let mut perms = fs::metadata(&target_bin)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&target_bin, perms)?;
    }

    Ok(())
}
