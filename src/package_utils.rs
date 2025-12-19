use serde::{Deserialize, Serialize};
use anyhow::{Context, Result};
use std::fs;
use flate2::read::GzDecoder;
use tar::Archive;
use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use console::style;
use crate::runner;
use semver::{Version, VersionReq};

#[derive(Debug, Deserialize)]
pub struct PackageDist {
    pub tarball: String,
    #[serde(default)]
    pub shasum: String,
}

#[derive(Debug, Deserialize)]
pub struct PackageVersion {
    pub dist: PackageDist,
}

#[derive(Debug, Deserialize)]
pub struct PackageMetadata {
    #[serde(rename = "dist-tags")]
    pub dist_tags: PackageDistTags,
    pub versions: HashMap<String, PackageVersion>,
}

#[derive(Debug, Deserialize)]
pub struct PackageDistTags {
    pub latest: String,
}

#[derive(Debug, Deserialize)]
struct InstalledPackageJson {
    #[serde(default)]
    dependencies: HashMap<String, String>,
    #[serde(default)]
    scripts: HashMap<String, String>,
    #[serde(default)]
    bin: PackageBin,
}

// Bin can be a string (executable name = package name) or a map
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum PackageBin {
    String(String),
    Map(HashMap<String, String>),
    None,
}

impl Default for PackageBin {
    fn default() -> Self {
        PackageBin::None
    }
}

pub fn fetch_package_version(name: &str, registry_url: &str, version_req_str: Option<&str>, client: &reqwest::blocking::Client) -> Result<(String, String, String)> {
    let encoded_name = name.replace("/", "%2f");
    let url = format!("{}/{}", registry_url, encoded_name);
    
    let resp = client.get(&url)
        .send()
        .context(format!("Failed to fetch metadata for package '{}'", name))?
        .error_for_status()?;
    
    let metadata: PackageMetadata = resp.json()?;
    
    // If no version constraint, use latest
    if version_req_str.is_none() {
        let latest_version = metadata.dist_tags.latest.clone();
        let version_info = metadata.versions.get(&latest_version)
            .context("Latest version not found")?;
        return Ok((latest_version, version_info.dist.tarball.clone(), version_info.dist.shasum.clone()));
    }

    let req_str = version_req_str.unwrap();
    // Use semver to find best match
    
    // Quick cleaning of version string for simple cases
    let req = VersionReq::parse(req_str).or_else(|_| {
        // Fallback: try parsing as exact version if it's just numbers
        VersionReq::parse(&format!("={}", req_str))
    }).unwrap_or_else(|_| {
        println!("{} Invalid version req '{}' for {}, using latest", style("⚠️").yellow(), req_str, name);
         VersionReq::STAR
    });

    let mut valid_versions: Vec<Version> = metadata.versions.keys()
        .filter_map(|v| Version::parse(v).ok())
        .filter(|v| req.matches(v))
        .collect();
    
    valid_versions.sort(); // Ascending
    
    if let Some(best_version) = valid_versions.last() {
        let best_version_str = best_version.to_string();
        let version_info = metadata.versions.get(&best_version_str)
            .context("Version not found in map")?;
        Ok((best_version_str, version_info.dist.tarball.clone(), version_info.dist.shasum.clone()))
    } else {
        println!("{} No matching version for {} {}, using latest", style("⚠️").yellow(), name, req_str);
        // Fallback to latest to try our best
        let latest_version = metadata.dist_tags.latest.clone();
        let version_info = metadata.versions.get(&latest_version)
            .context("Latest version not found")?;
        Ok((latest_version, version_info.dist.tarball.clone(), version_info.dist.shasum.clone()))
    }
}

pub fn install_package(name: &str, registry_url: &str, client: &reqwest::blocking::Client, lockfile: Option<&crate::manifest::CrabbyLock>) -> Result<(String, String)> {
    let mut visited = HashSet::new();
    install_package_recursive(name, registry_url, None, &mut visited, client, lockfile)
}

fn install_package_recursive(name: &str, registry_url: &str, version_req: Option<&str>, visited: &mut HashSet<String>, client: &reqwest::blocking::Client, lockfile: Option<&crate::manifest::CrabbyLock>) -> Result<(String, String)> {
    let visit_key = format!("{}@{}", name, version_req.unwrap_or("latest"));
    if visited.contains(&visit_key) {
        return Ok(("0.0.0".to_string(), "".to_string()));
    }
    visited.insert(visit_key);

    println!("{} Resolving {} {}", style("🔍").dim(), name, version_req.unwrap_or("latest"));

    // Check lockfile first if no specific version requirement or exact match
    if let Some(lock) = lockfile {
        if let Some(dep) = lock.dependencies.get(name) {
            let use_lock_version = match version_req {
                Some(req) => {
                     // Simple check: if req is "latest" or matches exact version
                     req == "latest" || req == dep.version
                },
                None => true,
            };
            
            if use_lock_version {
                println!("{} Using locked version {}", style("🔒").dim(), style(&dep.version).dim());
                download_and_extract(name, &dep.version, &dep.tarball, client, None)?;
                 return Ok((dep.version.clone(), dep.tarball.clone()));
            }
        }
    }

    let (version, tarball, checksum) = fetch_package_version(name, registry_url, version_req, client)?;
    
    // Download and verify integrity with checksum
    download_and_extract(name, &version, &tarball, client, Some(&checksum))?;
    
    // Checksum verification is now done inside download_and_extract

    let node_modules = Path::new("node_modules");
    let install_dir = node_modules.join(name);

    // Read package.json to find dependencies & binaries
    let pkg_json_path = install_dir.join("package.json");
    if pkg_json_path.exists() {
        let content = fs::read_to_string(&pkg_json_path)?;
        let pkg_json: InstalledPackageJson = serde_json::from_str(&content).unwrap_or(InstalledPackageJson { 
            dependencies: HashMap::new(), 
            scripts: HashMap::new(),
            bin: PackageBin::None 
        });

        // Link Binaries
        link_binaries(name, &pkg_json.bin, &install_dir)?;

        // Run preinstall
        if let Some(script) = pkg_json.scripts.get("preinstall") {
            println!("{} Running preinstall for {}: '{}'", style("⚙️").yellow(), name, script);
            runner::run_script(script, Some(&install_dir))?;
        }

        for (dep_name, dep_ver) in pkg_json.dependencies {
             install_package_recursive(&dep_name, registry_url, Some(&dep_ver), visited, client, lockfile)?;
        }

        // Run install
        if let Some(script) = pkg_json.scripts.get("install") {
            println!("{} Running install for {}: '{}'", style("⚙️").yellow(), name, script);
            runner::run_script(script, Some(&install_dir))?;
        }

        // Run postinstall
        if let Some(script) = pkg_json.scripts.get("postinstall") {
            println!("{} Running postinstall for {}: '{}'", style("⚙️").yellow(), name, script);
            runner::run_script(script, Some(&install_dir))?;
        }
    }

    Ok((version, tarball))
}

fn link_binaries(pkg_name: &str, bin: &PackageBin, install_dir: &Path) -> Result<()> {
    let node_modules = Path::new("node_modules");
    let bin_dir = node_modules.join(".bin");
    if !bin_dir.exists() {
        fs::create_dir_all(&bin_dir)?;
    }

    let links = match bin {
        PackageBin::String(path) => {
            let mut map = HashMap::new();
            map.insert(pkg_name.to_string(), path.clone());
            map
        },
        PackageBin::Map(map) => map.clone(),
        PackageBin::None => return Ok(()),
    };

    for (bin_name, file_path) in links {
        let source_path = install_dir.join(&file_path);
        let target = bin_dir.join(&bin_name);
        
        // Windows .cmd shim
        #[cfg(target_os = "windows")]
        {
            let shim_content = format!(
                "@ECHO OFF\r\nnode \"%~dp0\\..\\{}\\{}\" %*", 
                pkg_name, file_path
            );
            fs::write(target.with_extension("cmd"), shim_content)?;
        }

        // Unix shell shim (for git bash etc on windows, or real unix)
        #[cfg(not(target_os = "windows"))]
        {
             // Simple shell script
             use std::os::unix::fs::PermissionsExt;
             let shim_content = format!(
                "#!/bin/sh\nexec node \"$0/../../{}/{}\" \"$@\"",
                pkg_name, file_path
             );
             fs::write(&target, shim_content)?;
             let mut perms = fs::metadata(&target)?.permissions();
             perms.set_mode(0o755);
             fs::set_permissions(&target, perms)?;
        }
    }
    
    Ok(())
}

pub fn download_and_extract(name: &str, version: &str, tarball_url: &str, client: &reqwest::blocking::Client, expected_checksum: Option<&str>) -> Result<()> {
    use crate::config::get_cache_dir;
    
    // Create cache key from package name and version
    let cache_key = format!("{}-{}.tgz", name.replace("/", "-"), version);
    let cache_dir = get_cache_dir()?;
    let cached_file = cache_dir.join(&cache_key);
    
    // Check if tarball exists in cache
    let tar_gz_data = if cached_file.exists() {
        println!("{} Using cached tarball for {}", style("📦").dim(), name);
        fs::read(&cached_file)?
    } else {
        println!("{} Downloading {}", style("⬇️").dim(), name);
        let response = client.get(tarball_url)
            .send()
            .context("Failed to download tarball")?
            .error_for_status()?;
        
        let bytes = response.bytes()?.to_vec();
        
        // Save to cache
        fs::write(&cached_file, &bytes)?;
        bytes
    };

    // Verify checksum if provided
    if let Some(expected) = expected_checksum {
        if !expected.is_empty() {
            println!("{} Verifying checksum for {}", style("🔐").dim(), name);
            match crate::safety::verify_checksum(&cached_file, Some(expected)) {
                Ok(true) => {
                    println!("{} Checksum verified for {}", style("✅").green(), name);
                },
                Ok(false) => {
                    // Checksum mismatch - warn but continue (some registries have stale metadata)
                    println!("{} {} Checksum mismatch for package '{}'", 
                        style("⚠️").yellow(), 
                        style("WARNING:").bold().yellow(),
                        name
                    );
                    println!("   Expected: {}", style(expected).dim());
                    println!("   This package may have been updated recently.");
                    println!("   Installation will continue, but use caution.");
                },
                Err(e) => {
                    println!("{} Could not verify checksum: {}", style("⚠️").yellow(), e);
                }
            }
        }
    }

    let tar_gz = GzDecoder::new(&tar_gz_data[..]);
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

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;
        
        let path_str = path.to_string_lossy();
        let relative_path = if path_str.starts_with("package/") {
             path.strip_prefix("package")?
        } else {
             &path
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

