use anyhow::Result;
use std::path::Path;

/// Check if tsx is installed (needed for TypeScript execution)
pub fn is_tsx_installed() -> bool {
    let node_modules = Path::new("node_modules");
    node_modules.join("tsx").exists()
}

/// Check if a global tsx installation exists
pub fn is_tsx_globally_available() -> bool {
    // Check if tsx command is in PATH
    which::which("tsx").is_ok()
}

/// Prompt user to install tsx if not available
pub fn ensure_tsx_available() -> Result<bool> {
    if is_tsx_installed() || is_tsx_globally_available() {
        return Ok(true);
    }
    
    println!("\n{} TypeScript execution requires 'tsx'", console::style("⚠️").yellow());
    println!("{} Install it with: {}", 
        console::style("💡").cyan(),
        console::style("crabby install tsx").bold()
    );
    
    Ok(false)
}
