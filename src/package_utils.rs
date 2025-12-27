use anyhow::{Context, Result};
use console::style;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use flate2::read::GzDecoder;
use tar::Archive;
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};

use crate::runner;

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

pub async fn fetch_package_version(name: &str, registry_url: &str, version_req: Option<&str>, client: &reqwest::Client) -> anyhow::Result<(String, String, String)> {
    let url = format!("{}/{}", registry_url.trim_end_matches('/'), name);
    let response = client.get(&url)
        .send()
        .await
        .context("Failed to fetch package metadata")?
        .error_for_status()?;

    let metadata = response.json::<PackageMetadata>()
        .await
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
        crate::ui::print_warning(&format!("No matching version for {} {}, using latest", name, req_str));
        // Fallback to latest to try our best
        let latest_version = metadata.dist_tags.latest.clone();
        let version_info = metadata.versions.get(&latest_version)
            .context("Latest version not found")?;
        Ok((latest_version, version_info.dist.tarball.clone(), version_info.dist.shasum.clone()))
    }
}

// Shared state for recursion
struct InstallState {
    visited: Mutex<HashSet<String>>,
    package_locks: Mutex<HashMap<String, Arc<Mutex<()>>>>,
    lockfile: Mutex<crate::manifest::CrabbyLock>,
    client: reqwest::Client,
    registry_url: String,
    semaphore: Semaphore,
}

pub async fn install_package(name: &str, registry_url: &str, client: &reqwest::Client, lockfile: crate::manifest::CrabbyLock) -> Result<(String, String, crate::manifest::CrabbyLock)> {
    let state = Arc::new(InstallState {
        visited: Mutex::new(HashSet::new()),
        package_locks: Mutex::new(HashMap::new()),
        lockfile: Mutex::new(lockfile),
        client: client.clone(),
        registry_url: registry_url.to_string(),
        semaphore: Semaphore::new(crate::MAX_CONCURRENT_DOWNLOADS),
    });

    install_package_recursive(name.to_string(), None, state.clone()).await?;

    let lockfile = state.lockfile.lock().await.clone();
    Ok(("".to_string(), "".to_string(), lockfile))
}

// Recursive async function using BoxFuture for recursion
fn install_package_recursive(name: String, version_req: Option<String>, state: Arc<InstallState>) 
    -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send>> 
{
    Box::pin(async move {
        let visit_key = format!("{}@{}", name, version_req.as_deref().unwrap_or("latest"));
        
        {
            let mut visited = state.visited.lock().await;
            if visited.contains(&visit_key) {
                return Ok(());
            }
            visited.insert(visit_key);
        }

        // Check lockfile first
        // Check lockfile first
        let lock_data = {
            let lockfile = state.lockfile.lock().await;
            if let Some(dep) = lockfile.dependencies.get(&name) {
                let use_lock_version = match &version_req {
                    Some(req) => req == "latest" || req == &dep.version,
                    None => true,
                };
                
                if use_lock_version {
                    Some((dep.version.clone(), dep.tarball.clone()))
                } else {
                    None
                }
            } else {
                None
            }
        };

        if let Some((ver, tar)) = lock_data {
            println!("{} Using locked version {}", crate::ui::Icons::LOCK, style(&ver).dim());
            download_and_extract(&name, &ver, &tar, &state.client, None).await?;
            return Ok(());
        }

        println!("{} Resolving {} {}", crate::ui::Icons::SEARCH, style(&name).cyan(), style(version_req.as_deref().unwrap_or("latest")).dim());

        // Acquire per-package lock to prevent concurrent extraction of the same package name
        let pkg_lock = {
            let mut locks = state.package_locks.lock().await;
            locks.entry(name.clone()).or_insert_with(|| Arc::new(Mutex::new(()))).clone()
        };
        
        let _lock_guard = pkg_lock.lock().await;

        let (version, tarball, checksum) = fetch_package_version(&name, &state.registry_url, version_req.as_deref(), &state.client).await?;
        
        // Acquire permit for download slots
        let _permit = state.semaphore.acquire().await?;
        download_and_extract(&name, &version, &tarball, &state.client, Some(&checksum)).await?;
        drop(_permit);

        let node_modules = Path::new("node_modules");
        // Normalize name for filesystem (handle scoped packages @types/node)
        #[cfg(target_os = "windows")]
        let safe_name = name.replace("/", "\\");
        #[cfg(not(target_os = "windows"))]
        let safe_name = name.replace("/", "/");
        
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

            link_binaries(&name, &pkg_json.bin)?;

            // Run scripts (sequentially for now within this task, but we should be careful about concurrency here)
            // Ideally scripts run after all installs, but npm runs them post-extract often.
            // For safety in parallel mode, we might want to suppress interactive scripts or lock output.
            // For now, let's keep simplistic runner calls, but node usage might be tricky if parallel.
            
            // To be truly safe, we should probably collect scripts and run them at the end. 
            // But for "speed boost", parallel download is key.
            
            if let Some(script) = pkg_json.scripts.get("preinstall") {
                // println!("{} Running preinstall for {}", style("⚙️").yellow(), name);
                 runner::run_script(script, Some(&install_dir))?;
            }

            pkg_deps = pkg_json.dependencies.clone();
            
            // Spawn parallel tasks for dependencies
            let mut tasks = tokio::task::JoinSet::new();
            
            for (dep_name, dep_ver) in pkg_deps.clone() {
                let state_clone = state.clone();
                tasks.spawn(install_package_recursive(dep_name, Some(dep_ver), state_clone));
            }

            while let Some(res) = tasks.join_next().await {
                res??; // Check for JoinError and Result calls
            }

            if let Some(script) = pkg_json.scripts.get("install") {
                 runner::run_script(script, Some(&install_dir))?;
            }

            if let Some(script) = pkg_json.scripts.get("postinstall") {
                 runner::run_script(script, Some(&install_dir))?;
            }
        }

        {
            let mut lockfile = state.lockfile.lock().await;
            lockfile.add_package(name.clone(), version.clone(), tarball.clone(), pkg_deps);
        }
        
        Ok(())
    })
}

fn link_binaries(pkg_name: &str, bin: &PackageBin) -> Result<()> {
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
             // Permissions need to be set on unix
             if let Ok(mut perms) = fs::metadata(&target).and_then(|m| Ok(m.permissions())) {
                 perms.set_mode(0o755);
                 let _ = fs::set_permissions(&target, perms);
             }
        }
    }
    Ok(())
}

pub async fn download_and_extract(name: &str, version: &str, tarball_url: &str, client: &reqwest::Client, expected_checksum: Option<&str>) -> Result<()> {
    use crate::config::get_cache_dir;
    
    let cache_key = format!("{}-{}.tgz", name.replace("/", "-"), version);
    let cache_dir = get_cache_dir()?;
    let cached_file = cache_dir.join(&cache_key);
    
    let tar_gz_data = if cached_file.exists() {
        // println!("{} Using cached tarball for {}", style("📦").dim(), name);
        fs::read(&cached_file)?
    } else {
        println!("{} Downloading {}", crate::ui::Icons::DOWNLOAD, style(name).cyan());
        let response = client.get(tarball_url)
            .send()
            .await
            .context("Failed to download tarball")?
            .error_for_status()?;
        
        let bytes = response.bytes().await?.to_vec();
        fs::write(&cached_file, &bytes)?;
        bytes
    };

    if let Some(expected) = expected_checksum {
        if !expected.is_empty() {
             match crate::safety::verify_checksum(&cached_file, Some(expected)) {
                Ok(true) => {
                    // Verified
                },
                Ok(false) => {
                    println!("{} {} Checksum mismatch for package '{}'", 
                        style("⚠️").yellow(), 
                        style("WARNING:").bold().yellow(),
                        name
                    );
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
    #[cfg(target_os = "windows")]
    let safe_name = name.replace("/", "\\");
    #[cfg(not(target_os = "windows"))]
    let safe_name = name.replace("/", "/");

    let target_dir = node_modules.join(&safe_name);
    if target_dir.exists() {
        fs::remove_dir_all(&target_dir)?;
    }
    fs::create_dir_all(&target_dir)?;

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?.to_path_buf();
        
        let mut components = path.components();
        let _root = components.next();
        let relative_path = components.as_path();

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

pub async fn install_all_packages(deps: &HashMap<String, String>, registry_url: &str, client: &reqwest::Client, lockfile: crate::manifest::CrabbyLock) -> Result<crate::manifest::CrabbyLock> {
    let state = Arc::new(InstallState {
        visited: Mutex::new(HashSet::new()),
        package_locks: Mutex::new(HashMap::new()),
        lockfile: Mutex::new(lockfile),
        client: client.clone(),
        registry_url: registry_url.to_string(),
        semaphore: Semaphore::new(crate::MAX_CONCURRENT_DOWNLOADS),
    });

    let mut tasks = tokio::task::JoinSet::new();
    
    if deps.is_empty() {
        return Ok(state.lockfile.lock().await.clone());
    }

    for (name, version_req) in deps {
        let state_clone = state.clone();
        let name = name.clone();
        let version_req = version_req.clone();
        tasks.spawn(install_package_recursive(name, Some(version_req), state_clone));
    }

    while let Some(res) = tasks.join_next().await {
        res??;
    }
    
    let lockfile = state.lockfile.lock().await.clone();
    Ok(lockfile)
}
