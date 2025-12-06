use serde::Deserialize;
use anyhow::{Context, Result};
use std::fs;
use std::io::copy;
use flate2::read::GzDecoder;
use tar::Archive;
use std::path::Path;

const REGISTRY_URL: &str = "https://registry.npmjs.org";

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

pub fn fetch_package_version(name: &str) -> Result<(String, String)> {
    let url = format!("{}/{}", REGISTRY_URL, name);
    let resp = reqwest::blocking::get(&url)
        .context(format!("Failed to fetch metadata for package '{}'", name))?
        .error_for_status()?;
    
    let metadata: PackageMetadata = resp.json()?;
    let latest_version = metadata.dist_tags.latest.clone();
    
    let version_info = metadata.versions.get(&latest_version)
        .context("Latest version not found in metadata")?;
        
    Ok((latest_version, version_info.dist.tarball.clone()))
}

pub fn download_and_extract(name: &str, version: &str, tarball_url: &str) -> Result<()> {
    let response = reqwest::blocking::get(tarball_url)
        .context("Failed to download tarball")?
        .error_for_status()?;

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
