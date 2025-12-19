use anyhow::{Result, Context, bail};
use console::style;
use std::process::Command;
use std::env;
use std::io::{self, Write};

const GITHUB_CARGO_TOML: &str = "https://raw.githubusercontent.com/AqwozTheDeveloper/crabby/main/Cargo.toml";
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub async fn check_and_upgrade() -> Result<()> {
    println!("{} Checking for updates...", style("🔍").bold().cyan());

    let latest_version = fetch_latest_version().await?;
    
    if is_newer(&latest_version, CURRENT_VERSION) {
        println!("{} New version available: {} (current: {})", 
            style("✨").bold().green(), 
            style(&latest_version).bold().yellow(),
            style(CURRENT_VERSION).dim()
        );
        
        print!("\n{} Would you like to upgrade now? (y/n): ", style("❓").bold().yellow());
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        if input.trim().to_lowercase() == "y" {
            perform_upgrade().await?;
        } else {
            println!("{} Upgrade cancelled.", style("❌").red());
        }
    } else {
        println!("{} Crabby is already up to date! (v{})", style("✅").bold().green(), CURRENT_VERSION);
    }

    Ok(())
}

async fn fetch_latest_version() -> Result<String> {
    let client = reqwest::Client::builder()
        .user_agent("crabby-self-upgrade")
        .build()?;
        
    let content = client.get(GITHUB_CARGO_TOML)
        .send()
        .await?
        .text()
        .await?;
        
    // Simple parsing of version = "x.y.z"
    for line in content.lines() {
        if line.starts_with("version = \"") {
            let version = line.trim_start_matches("version = \"")
                .trim_end_matches("\"");
            return Ok(version.to_string());
        }
    }
    
    bail!("Could not find version in remote Cargo.toml")
}

fn is_newer(latest: &str, current: &str) -> bool {
    // Simple semver comparison or string comparison?
    // Let's use semver crate since it's already a dependency
    let v_latest = semver::Version::parse(latest).unwrap_or(semver::Version::new(0, 0, 0));
    let v_current = semver::Version::parse(current).unwrap_or(semver::Version::new(0, 0, 0));
    
    v_latest > v_current
}

async fn perform_upgrade() -> Result<()> {
    println!("{} Pulling latest changes...", style("📂").bold().blue());
    
    // Check if we are in a git repo
    let status = Command::new("git")
        .args(&["pull"])
        .status()
        .context("Failed to run 'git pull'. Ensure you are in the crabby source directory.")?;
        
    if !status.success() {
        bail!("Git pull failed");
    }
    
    println!("{} Rebuilding Crabby...", style("🔨").bold().yellow());
    
    let status = Command::new("cargo")
        .args(&["build", "--release"])
        .status()
        .context("Failed to run 'cargo build'. Ensure Rust is installed.")?;
        
    if !status.success() {
        bail!("Build failed");
    }
    
    println!("{} Installing new binary...", style("📦").bold().magenta());
    
    // Determine target location (same as installer)
    let home = dirs::home_dir().context("Could not find home directory")?;
    let bin_dir = home.join(".crabby").join("bin");
    
    #[cfg(target_os = "windows")]
    let exe_name = "crabby.exe";
    #[cfg(not(target_os = "windows"))]
    let exe_name = "crabby";
    
    let target_path = bin_dir.join(exe_name);
    let source_path = std::path::Path::new("target").join("release").join(exe_name);
    
    if !source_path.exists() {
        bail!("Source binary not found at {:?}", source_path);
    }
    
    // On Windows, if we are running the binary we are trying to replace, copy will fail.
    // However, the user is likely running the binary from the source dir or from terminal.
    // If they run `crabby upgrade --self` and it's THE one in bin_dir, we might need to overwrite carefully.
    
    std::fs::create_dir_all(&bin_dir)?;
    
    #[cfg(target_os = "windows")]
    {
        // Try to rename the target first if it exists
        if target_path.exists() {
            let old_path = target_path.with_extension("old");
            if old_path.exists() {
                let _ = std::fs::remove_file(&old_path);
            }
            std::fs::rename(&target_path, &old_path).context("Failed to rename existing binary")?;
        }
    }
    
    std::fs::copy(&source_path, &target_path).context("Failed to copy new binary to installation directory")?;
    
    println!("\n{} Crabby upgraded successfully to the latest version!", style("🎉").bold().green());
    println!("{} You may need to restart your terminal.", style("💡").dim());
    
    Ok(())
}
