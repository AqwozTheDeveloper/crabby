use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use crate::global;

/// Check if tsx is installed locally in node_modules
pub fn is_tsx_installed_locally() -> bool {
    let node_modules = Path::new("node_modules");
    node_modules.join("tsx").exists()
}

/// Get the path to the tsx entry point
pub fn get_tsx_path() -> Result<PathBuf> {
    // 1. Check local node_modules
    let local_path = Path::new("node_modules").join("tsx").join("dist").join("cli.mjs");
    if local_path.exists() {
        return Ok(local_path);
    }

    // 2. Check global .crabby/global
    let global_dir = global::get_global_dir()?;
    let global_path = global_dir.join("node_modules").join("tsx").join("dist").join("cli.mjs");
    if global_path.exists() {
        return Ok(global_path);
    }

    // 3. Last resort: try to find it in PATH (not ideal if we want to run via node directly)
    if let Ok(path) = which::which("tsx") {
        return Ok(path);
    }

    anyhow::bail!("tsx not found locally or globally")
}

/// Check if a global tsx installation exists
pub fn is_tsx_globally_available() -> bool {
    let global_dir = global::get_global_dir().map_or(PathBuf::new(), |d| d);
    let global_path = global_dir.join("node_modules").join("tsx").join("dist").join("cli.mjs");
    
    global_path.exists() || which::which("tsx").is_ok()
}

/// Prompt user to install tsx if not available
pub fn ensure_tsx_available() -> Result<bool> {
    if is_tsx_installed_locally() || is_tsx_globally_available() {
        return Ok(true);
    }
    
    println!("\n{} TypeScript execution requires 'tsx'", console::style("⚠️").yellow());
    println!("{} Install it: {}", 
        console::style("💡").cyan(),
        console::style("crabby install tsx").bold()
    );
    
    Ok(false)
}
