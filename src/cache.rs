use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::fs;
use sha1::{Sha1, Digest};

/// Get the cache directory path (~/.crabby/cache)
#[allow(dead_code)]
pub fn get_cache_dir() -> Result<PathBuf> {
    let home = dirs::home_dir()
        .context("Could not determine home directory")?;
    
    let cache_dir = home.join(".crabby").join("cache");
    fs::create_dir_all(&cache_dir)?;
    
    Ok(cache_dir)
}

#[allow(dead_code)]
pub fn get_package_cache_path(name: &str, version: &str) -> Result<PathBuf> {
    let cache_dir = get_cache_dir()?;
    let packages_dir = cache_dir.join("packages");
    fs::create_dir_all(&packages_dir)?;
    
    let safe_name = name.replace("/", "_");
    let filename = format!("{}-{}.tgz", safe_name, version);
    Ok(packages_dir.join(filename))
}

#[allow(dead_code)]
pub fn is_cached(name: &str, version: &str, expected_checksum: Option<&str>) -> Result<bool> {
    let cache_path = get_package_cache_path(name, version)?;
    
    if !cache_path.exists() {
        return Ok(false);
    }
    
    // Verify checksum if provided
    if let Some(checksum) = expected_checksum {
        let cached_checksum = calculate_file_checksum(&cache_path)?;
        return Ok(cached_checksum == checksum);
    }
    
    Ok(true)
}

#[allow(dead_code)]
pub fn save_to_cache(name: &str, version: &str, data: &[u8]) -> Result<PathBuf> {
    let cache_path = get_package_cache_path(name, version)?;
    fs::write(&cache_path, data)
        .context("Failed to write package to cache")?;
    
    println!("{} Cached {}", console::style("💾").dim(), cache_path.display());
    Ok(cache_path)
}

#[allow(dead_code)]
pub fn load_from_cache(name: &str, version: &str) -> Result<Vec<u8>> {
    let cache_path = get_package_cache_path(name, version)?;
    println!("{} Loading from cache", console::style("⚡").cyan());
    
    fs::read(&cache_path)
        .context("Failed to read package from cache")
}

#[allow(dead_code)]
fn calculate_file_checksum(path: &Path) -> Result<String> {
    let data = fs::read(path)?;
    let mut hasher = Sha1::new();
    hasher.update(&data);
    Ok(format!("{:x}", hasher.finalize()))
}

#[allow(dead_code)]
pub fn clear_cache() -> Result<()> {
    let cache_dir = get_cache_dir()?;
    
    if cache_dir.exists() {
        fs::remove_dir_all(&cache_dir)?;
        fs::create_dir_all(&cache_dir)?;
        println!("{} Cache cleared", console::style("🗑️").green());
    }
    
    Ok(())
}

#[allow(dead_code)]
pub fn get_cache_stats() -> Result<(usize, u64)> {
    let cache_dir = get_cache_dir()?;
    let packages_dir = cache_dir.join("packages");
    
    if !packages_dir.exists() {
        return Ok((0, 0));
    }
    
    let mut count = 0;
    let mut total_size = 0u64;
    
    for entry in fs::read_dir(packages_dir)? {
        let entry = entry?;
        if entry.path().is_file() {
            count += 1;
            total_size += entry.metadata()?.len();
        }
    }
    
    Ok((count, total_size))
}
