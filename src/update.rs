use anyhow::{Context, Result};
use colored::Colorize;
use console::style;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct RegistryPackage {
    #[serde(rename = "dist-tags")]
    dist_tags: HashMap<String, String>,
    versions: HashMap<String, VersionInfo>,
}

#[derive(Debug, Deserialize)]
struct VersionInfo {
    version: String,
    description: Option<String>,
}

/// Update a specific package to latest version
pub async fn update_package(name: &str, registry: &str) -> Result<(String, String)> {
    println!("{} Checking for updates to {}...", style("🔍").dim(), name);
    
    let url = format!("{}/{}", registry, name);
    let response = reqwest::get(&url).await?;
    
    if !response.status().is_success() {
        anyhow::bail!("Package {} not found in registry", name);
    }
    
    let pkg: RegistryPackage = response.json().await?;
    let latest = pkg.dist_tags.get("latest")
        .context("No latest version found")?;
    
    println!("{} Latest version: {}", style("📌").dim(), latest);
    
    // Return version and tarball URL
    let version_info = pkg.versions.get(latest)
        .context("Version info not found")?;
    
    let tarball = format!("{}/{}/-/{}-{}.tgz", registry, name, name, latest);
    
    Ok((latest.clone(), tarball))
}

/// Check which packages are outdated
pub async fn check_outdated(registry: &str) -> Result<Vec<(String, String, String)>> {
    let pkg_json = crate::manifest::PackageJson::load()?;
    let mut outdated = Vec::new();
    
    for (name, current_version) in &pkg_json.dependencies {
        let current = current_version.trim_start_matches('^');
        
        match update_package(name, registry).await {
            Ok((latest, _)) => {
                if latest != current {
                    outdated.push((name.clone(), current.to_string(), latest));
                }
            }
            Err(_) => continue,
        }
    }
    
    Ok(outdated)
}

/// Get package information from registry
pub async fn get_package_info(name: &str, registry: &str) -> Result<()> {
    let url = format!("{}/{}", registry, name);
    let response = reqwest::get(&url).await?;
    
    if !response.status().is_success() {
        anyhow::bail!("Package {} not found", name);
    }
    
    let pkg: RegistryPackage = response.json().await?;
    let latest = pkg.dist_tags.get("latest")
        .context("No latest version found")?;
    
    let version_info = pkg.versions.get(latest)
        .context("Version info not found")?;
    
    println!("\n{}", style(format!("📦 {}", name)).bold().cyan());
    println!("{}", "=".repeat(50));
    println!("{}: {}", style("Version").bold(), latest);
    
    if let Some(desc) = &version_info.description {
        println!("{}: {}", style("Description").bold(), desc);
    }
    
    println!("{}: {}/{}", style("Registry").bold(), registry, name);
    println!();
    
    Ok(())
}
