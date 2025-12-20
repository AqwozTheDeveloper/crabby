use anyhow::{Context, Result};
use console::style;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use flate2::read::GzDecoder;
use tar::Archive;

use crate::{manifest, runner, registry, tsx_utils};

#[derive(Debug, Deserialize)]
pub struct PackageMetadata {
    pub name: String,
    pub versions: HashMap<String, PackageVersion>,
    #[serde(rename = "dist-tags")]
    pub dist_tags: DistTags,
}

#[derive(Debug, Deserialize)]
pub struct DistTags {
    pub latest: String,
}

#[derive(Debug, Deserialize)]
pub struct PackageVersion {
    pub version: String,
    pub dist: PackageDist,
}

#[derive(Debug, Deserialize)]
pub struct PackageDist {
    pub tarball: String,
    pub shasum: String,
}

#[derive(Debug, Deserialize)]
pub struct InstalledPackageJson {
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
    #[serde(default)]
    pub scripts: HashMap<String, String>,
    #[serde(default)]
    pub bin: PackageBin,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum PackageBin {
    String(String),
    Map(HashMap<String, String>),
    None,
}

impl Default for PackageBin {
    fn default() -> Self {
        PackageBin::None
    }
}

pub fn fetch_package_version(name: &str, registry_url: &str, version_req: Option<&str>, client: &reqwest::blocking::Client) -> Result<(String, String, String)> {
    let url = format!("{}/{}", registry_url.trim_end_matches('/'), name);
    let response = client.get(&url)
        .send()
        .context("Failed to fetch package metadata")?
        .error_for_status()?;

    let metadata: PackageMetadata = response.json()
        .context("Failed to parse package metadata")?;

    let req_str = version_req.unwrap_or("latest");
    
    // Resolve version
    if let Ok(req) = semver::VersionReq::parse(req_str) {
        let mut versions: Vec<semver::Version> = metadata.versions.keys()
            .filter_map(|v| semver::Version::parse(v).ok())
            .collect();
        versions.sort();
        
        let best_version = versions.into_iter()
            .rev()
            .find(|v| req.matches(v))
            .context(format!("No matching version found for {}@{}", name, req_str))?;

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

pub fn install_package(name: &str, registry_url: &str, client: &reqwest::blocking::Client, lockfile: &mut crate::manifest::CrabbyLock) -> Result<(String, String)> {
    let mut visited = HashSet::new();
    install_package_recursive(name, registry_url, None, &mut visited, client, lockfile)
}

fn install_package_recursive(name: &str, registry_url: &str, version_req: Option<&str>, visited: &mut HashSet<String>, client: &reqwest::blocking::Client, lockfile: &mut crate::manifest::CrabbyLock) -> Result<(String, String)> {
    let visit_key = format!("{}@{}", name, version_req.unwrap_or("latest"));
    if visited.contains(&visit_key) {
        if let Some(dep) = lockfile.dependencies.get(name) {
             return Ok((dep.version.clone(), dep.tarball.clone()));
        }
        return Ok(("0.0.0".to_string(), "".to_string()));
    }
    visited.insert(visit_key);

    println!("{} Resolving {} {}", style("🔍").dim(), name, version_req.unwrap_or("latest"));

    if let Some(dep) = lockfile.dependencies.get(name) {
        let use_lock_version = match version_req {
            Some(req) => req == "latest" || req == dep.version,
            None => true,
        };
        
        if use_lock_version {
            println!("{} Using locked version {}", style("🔒").dim(), style(&dep.version).dim());
            download_and_extract(name, &dep.version, &dep.tarball, client, None)?;
             return Ok((dep.version.clone(), dep.tarball.clone()));
        }
    }

    let (version, tarball, checksum) = fetch_package_version(name, registry_url, version_req, client)?;
    download_and_extract(name, &version, &tarball, client, Some(&checksum))?;

    let node_modules = Path::new("node_modules");
    // Normalize name for filesystem (handle scoped packages @types/node)
    let safe_name = name.replace("/", &std::path::MAIN_SEPARATOR.to_string());
    let install_dir = node_modules.join(&safe_name);
    let mut pkg_deps = HashMap::new();

    let pkg_json_path = install_dir.join("package.json");
    if pkg_json_path.exists() {
        let content = fs::read_to_string(&pkg_json_path)?;
        let cleaned = crate::manifest::clean_json_content(content);
        let pkg_json: InstalledPackageJson = match serde_json::from_str(&cleaned) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Warning: Failed to parse package.json for {}: {}", name, e);
                InstalledPackageJson { 
                    dependencies: HashMap::new(), 
                    scripts: HashMap::new(),
                    bin: PackageBin::None 
                }
            }
        };

        link_binaries(name, &pkg_json.bin, &install_dir)?;

        if let Some(script) = pkg_json.scripts.get("preinstall") {
            println!("{} Running preinstall for {}: '{}'", style("⚙️").yellow(), name, script);
            runner::run_script(script, Some(&install_dir))?;
        }

        pkg_deps = pkg_json.dependencies.clone();
        for (dep_name, dep_ver) in &pkg_deps {
             install_package_recursive(dep_name, registry_url, Some(dep_ver), visited, client, lockfile)?;
        }

        if let Some(script) = pkg_json.scripts.get("install") {
            println!("{} Running install for {}: '{}'", style("⚙️").yellow(), name, script);
            runner::run_script(script, Some(&install_dir))?;
        }

        if let Some(script) = pkg_json.scripts.get("postinstall") {
            println!("{} Running postinstall for {}: '{}'", style("⚙️").yellow(), name, script);
            runner::run_script(script, Some(&install_dir))?;
        }
    }

    lockfile.add_package(name.to_string(), version.clone(), tarball.clone(), pkg_deps);
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
        let target = bin_dir.join(&bin_name);
        
        #[cfg(target_os = "windows")]
        {
            let shim_content = format!(
                "@ECHO OFF\r\nnode \"%~dp0\\..\\{}\\{}\" %*", 
                pkg_name, file_path
            );
            fs::write(target.with_extension("cmd"), shim_content)?;
        }

        #[cfg(not(target_os = "windows"))]
        {
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
    
    let cache_key = format!("{}-{}.tgz", name.replace("/", "-"), version);
    let cache_dir = get_cache_dir()?;
    let cached_file = cache_dir.join(&cache_key);
    
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
        fs::write(&cached_file, &bytes)?;
        bytes
    };

    if let Some(expected) = expected_checksum {
        if !expected.is_empty() {
            println!("{} Verifying checksum for {}", style("🔐").dim(), name);
            match crate::safety::verify_checksum(&cached_file, Some(expected)) {
                Ok(true) => {
                    println!("{} Checksum verified for {}", style("✅").green(), name);
                },
                Ok(false) => {
                    println!("{} {} Checksum mismatch for package '{}'", 
                        style("⚠️").yellow(), 
                        style("WARNING:").bold().yellow(),
                        name
                    );
                    println!("   Expected: {}", style(expected).dim());
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
        fs::create_dir_all(node_modules)?;
    }
    
    // Normalize name for filesystem (handle scoped packages @types/node)
    let safe_name = name.replace("/", &std::path::MAIN_SEPARATOR.to_string());
    let target_dir = node_modules.join(&safe_name);
    if target_dir.exists() {
        fs::remove_dir_all(&target_dir)?;
    }
    fs::create_dir_all(&target_dir)?;

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?.to_path_buf();
        
        // Strip the first component (usually "package", but can be anything)
        let mut components = path.components();
        let _root = components.next();
        let relative_path = components.as_path();

        if relative_path.as_os_str().is_empty() {
             continue; 
        }

        let extract_path = target_dir.join(relative_path);
        // println!("   {} Extracting to: {}", style("📄").dim(), extract_path.display());
        if let Some(parent) = extract_path.parent() {
            fs::create_dir_all(parent)?;
        }
        entry.unpack(&extract_path)?;
    }
    Ok(())
}

pub fn install_all_packages(deps: &HashMap<String, String>, registry_url: &str, client: &reqwest::blocking::Client, lockfile: &mut crate::manifest::CrabbyLock) -> Result<()> {
    if deps.is_empty() {
        return Ok(());
    }
    for (name, _version_req) in deps {
        let _ = install_package(name, registry_url, client, lockfile)?;
    }
    Ok(())
}
