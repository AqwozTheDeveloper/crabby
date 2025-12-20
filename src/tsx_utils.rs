use anyhow::Result;
use std::path::{Path, PathBuf};
use crate::global;

#[derive(Debug, Clone)]
pub enum TsxCommand {
    NodeMjs(PathBuf),
    Executable(PathBuf),
}

/// Get the path to the tsx entry point
pub fn get_tsx_command() -> Result<TsxCommand> {
    // 1. Check local node_modules dist (new versions)
    let local_mjs = Path::new("node_modules").join("tsx").join("dist").join("cli.mjs");
    if local_mjs.exists() {
        return Ok(TsxCommand::NodeMjs(local_mjs));
    }
    
    // 2. Check local node_modules bin
    let local_bin = Path::new("node_modules").join(".bin").join("tsx");
    #[cfg(target_os = "windows")]
    let local_bin = local_bin.with_extension("cmd");
    if local_bin.exists() {
        return Ok(TsxCommand::Executable(local_bin));
    }

    // 3. Check global .crabby/global
    if let Ok(global_dir) = global::get_global_dir() {
        let global_mjs = global_dir.join("node_modules").join("tsx").join("dist").join("cli.mjs");
        if global_mjs.exists() {
            return Ok(TsxCommand::NodeMjs(global_mjs));
        }
    }

    // 4. Check global .crabby/bin
    if let Ok(bin_dir) = global::get_global_bin_dir() {
        let global_bin = bin_dir.join("tsx");
        #[cfg(target_os = "windows")]
        let global_bin = global_bin.with_extension("cmd");
        if global_bin.exists() {
            return Ok(TsxCommand::Executable(global_bin));
        }
    }

    // 5. Check PATH
    if let Ok(path) = which::which("tsx") {
        return Ok(TsxCommand::Executable(path));
    }

    anyhow::bail!("tsx not found locally or globally")
}

/// Check if tsx is available
pub fn is_tsx_globally_available() -> bool {
    get_tsx_command().is_ok()
}

/// Prompt user to install tsx if not available
pub fn ensure_tsx_available() -> Result<bool> {
    if is_tsx_globally_available() {
        return Ok(true);
    }
    
    println!("\n{} TypeScript execution requires 'tsx'", console::style("⚠️").yellow());
    println!("{} Install it: {}", 
        console::style("💡").cyan(),
        console::style("crabby install tsx").bold()
    );
    
    Ok(false)
}
