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
    // Determine target location (same as installer)
    let home = dirs::home_dir().context("Could not find home directory")?;
    let crabby_dir = home.join(".crabby");
    let source_dir = crabby_dir.join("src");
    let bin_dir = crabby_dir.join("bin");
    
    std::fs::create_dir_all(&source_dir)?;
    
    println!("{} Preparing source directory at {:?}...", style("📂").bold().blue(), source_dir);

    // If .git doesn't exist in source_dir, clone. Otherwise, pull.
    if !source_dir.join(".git").exists() {
        println!("{} Cloning Crabby repository...", style("📥").bold().blue());
        let status = Command::new("git")
            .args(&["clone", "https://github.com/AqwozTheDeveloper/crabby", "."])
            .current_dir(&source_dir)
            .status()
            .context("Failed to run 'git clone'. Ensure Git is installed.")?;
            
        if !status.success() {
            bail!("Git clone failed. Please check your internet connection.");
        }
    } else {
        println!("{} Updating source code...", style("📂").bold().blue());
        
        // Reset local changes if any, to ensure clean pull
        let _ = Command::new("git")
            .args(&["fetch", "--all"])
            .current_dir(&source_dir)
            .status();
            
        let status = Command::new("git")
            .args(&["reset", "--hard", "origin/main"])
            .current_dir(&source_dir)
            .status()
            .context("Failed to run 'git reset'.")?;
            
        if !status.success() {
            bail!("Failed to update source code via git reset.");
        }
    }
    
    println!("{} Rebuilding Crabby (this may take a minute)...", style("🔨").bold().yellow());
    
    let status = Command::new("cargo")
        .args(&["build", "--release"])
        .current_dir(&source_dir)
        .status()
        .context("Failed to run 'cargo build'. Ensure Rust is installed (https://rustup.rs).")?;
        
    if !status.success() {
        bail!("Build failed. There might be a compilation error in the latest version.");
    }
    
    println!("{} Installing new binary...", style("📦").bold().magenta());
    
    #[cfg(target_os = "windows")]
    let exe_name = "crabby.exe";
    #[cfg(not(target_os = "windows"))]
    let exe_name = "crabby";
    
    let target_path = bin_dir.join(exe_name);
    let source_path = source_dir.join("target").join("release").join(exe_name);
    
    if !source_path.exists() {
        bail!("Source binary not found at {:?}. Build might have skipped the release target.", source_path);
    }
    
    std::fs::create_dir_all(&bin_dir)?;
    
    #[cfg(target_os = "windows")]
    {
        // Try to rename the target first if it exists
        if target_path.exists() {
            let old_path = target_path.with_extension("old");
            if old_path.exists() {
                let _ = std::fs::remove_file(&old_path);
            }
            std::fs::rename(&target_path, &old_path).context("Failed to swap existing binary. Ensure Crabby is not running in another window.")?;
        }
    }
    
    std::fs::copy(&source_path, &target_path).context("Failed to copy new binary to installation directory.")?;
    
    println!("\n{} Crabby upgraded successfully to v{}!", style("🎉").bold().green(), fetch_latest_version().await.unwrap_or_default());
    println!("{} Run {} to verify the new version.", style("💡").dim(), style("crabby --version").cyan());
    
    Ok(())
}
