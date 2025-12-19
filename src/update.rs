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

pub async fn update_package(name: &str, registry: &str) -> Result<(String, String)> {
    println!("{} Checking for updates to {}...", style("🔍").dim(), name);
    
    let registry_owned = registry.to_string();
    let name_owned = name.to_string();

    let (latest, tarball, _) = tokio::task::spawn_blocking(move || {
        let client = crate::registry::get_client()?;
        crate::package_utils::fetch_package_version(&name_owned, &registry_owned, None, &client)
    }).await??;
    
    println!("{} Latest version: {}", style("📌").dim(), latest);
    
    Ok((latest, tarball))
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
    let registry_owned = registry.to_string();
    let name_owned = name.to_string();

    let pkg = tokio::task::spawn_blocking(move || {
        let client = crate::registry::get_client()?;
        let url = format!("{}/{}", registry_owned, name_owned);
        
        let mut attempt = 0;
        let max_retries = 3;
        
        loop {
            attempt += 1;
            match client.get(&url).send() {
                Ok(resp) => {
                    let resp = resp.error_for_status()?;
                    let pkg: RegistryPackage = resp.json()?;
                    return Ok(pkg);
                },
                Err(e) => {
                    if attempt >= max_retries {
                         return Err(anyhow::anyhow!("Failed to fetch {} info: {}", name_owned, e));
                    }
                    std::thread::sleep(std::time::Duration::from_secs(2u64.pow(attempt - 1)));
                }
            }
        }
    }).await??;

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
