use serde::Deserialize;
use anyhow::{Context, Result};
use std::fs;
use flate2::read::GzDecoder;
use tar::Archive;
use std::path::Path;
use std::time::Duration;
use std::thread;
use console::style;

const REGISTRY_URL: &str = "https://registry.npmjs.org";
const MAX_RETRIES: u32 = 3;
const TIMEOUT_SECS: u64 = 60;

#[derive(Debug, Deserialize)]
pub struct PackageDist {
    pub tarball: String,
}

#[derive(Debug, Deserialize)]
pub struct PackageVersion {
    pub dist: PackageDist,
}

#[derive(Debug, Deserialize)]
pub struct PackageMetadata {
    #[serde(rename = "dist-tags")]
    pub dist_tags: PackageDistTags,
    pub versions: std::collections::HashMap<String, PackageVersion>,
}

#[derive(Debug, Deserialize)]
pub struct PackageDistTags {
    pub latest: String,
}

pub fn get_client() -> Result<reqwest::blocking::Client> {
    // reqwest::blocking::Client::builder() panics if called in async context.
    // Crabby uses tokio::main, so we are in async context.
    // We must spawn a separate thread to create the client.
    
    std::thread::spawn(|| {
        reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(TIMEOUT_SECS))
            .build()
            .context("Failed to create HTTP client")
    }).join().unwrap_or_else(|_| Err(anyhow::anyhow!("Thread panicked creating client")))
}

pub fn fetch_package_version(name: &str) -> Result<(String, String)> {
    let url = format!("{}/{}", REGISTRY_URL, name);
    let client = get_client()?;

    let mut attempt = 0;
    loop {
        attempt += 1;
        match client.get(&url).send() {
            Ok(resp) => {
                let resp = resp.error_for_status()?;
                let metadata: PackageMetadata = resp.json()?;
                let latest_version = metadata.dist_tags.latest.clone();
                
                let version_info = metadata.versions.get(&latest_version)
                    .context("Latest version not found in metadata")?;
                
                return Ok((latest_version, version_info.dist.tarball.clone()));
            }
            Err(e) => {
                if attempt >= MAX_RETRIES {
                    return Err(anyhow::anyhow!("Failed to fetch metadata for package '{}' after {} attempts: {}", name, MAX_RETRIES, e));
                }
                println!("{} Retrying metadata fetch for {} (attempt {}/{}): {}", 
                    style("⚠️").yellow(), 
                    name, 
                    attempt, 
                    MAX_RETRIES, 
                    e
                );
                thread::sleep(Duration::from_secs(2u64.pow(attempt - 1)));
            }
        }
    }
}

pub fn download_and_extract(name: &str, _version: &str, tarball_url: &str) -> Result<()> {
    // Note: This function seems to be legacy or used for simple cases. 
    // package_utils::download_and_extract is the main one used by install command.
    // However, we update this one too for consistency.
    
    let client = get_client()?;
    
    let mut attempt = 0;
    let response = loop {
        attempt += 1;
        match client.get(tarball_url).send() {
            Ok(resp) => break resp.error_for_status()?,
            Err(e) => {
                 if attempt >= MAX_RETRIES {
                    return Err(anyhow::anyhow!("Failed to download tarball for '{}' after {} attempts: {}", name, MAX_RETRIES, e));
                }
                println!("{} Retrying download for {} (attempt {}/{}): {}", 
                    style("⚠️").yellow(), 
                    name, 
                    attempt, 
                    MAX_RETRIES, 
                    e
                );
                thread::sleep(Duration::from_secs(2u64.pow(attempt - 1)));
            }
        }
    };

    let tar_gz = GzDecoder::new(response);
    let mut archive = Archive::new(tar_gz);

    let node_modules = Path::new("node_modules");
    if !node_modules.exists() {
        fs::create_dir(node_modules)?;
    }
    
    let target_dir = node_modules.join(name);
    if target_dir.exists() {
        fs::remove_dir_all(&target_dir)?;
    }
    fs::create_dir_all(&target_dir)?;

    // NPM tarballs usually contain a 'package' root directory. We want to strip that.
    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;
        
        let path_str = path.to_string_lossy();
        let relative_path = if path_str.starts_with("package/") {
             path.strip_prefix("package")?
        } else {
             continue; // Skip files not in 'package/' if any (standard npm packing puts everything in package/)
        };

        if relative_path.as_os_str().is_empty() {
             continue; 
        }

        let extract_path = target_dir.join(relative_path);
        if let Some(parent) = extract_path.parent() {
            fs::create_dir_all(parent)?;
        }
        entry.unpack(&extract_path)?;
    }

    Ok(())
}
