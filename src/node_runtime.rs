use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;

/// Get the path to Node.js executable
/// Returns system Node.js if available, otherwise downloads portable version
pub fn get_node_path() -> Result<PathBuf> {
    // First, try to find system Node.js
    if let Ok(path) = find_system_node() {
        return Ok(path);
    }
    
    // If not found, use or download portable Node.js
    get_portable_node()
}

/// Try to find Node.js in system PATH
fn find_system_node() -> Result<PathBuf> {
    let node_cmd = if cfg!(target_os = "windows") {
        "node.exe"
    } else {
        "node"
    };
    
    // Try running node --version to check if it exists
    let output = Command::new(node_cmd)
        .arg("--version")
        .output();
    
    if output.is_ok() {
        Ok(PathBuf::from(node_cmd))
    } else {
        anyhow::bail!("System Node.js not found")
    }
}

/// Get portable Node.js path, download if needed
fn get_portable_node() -> Result<PathBuf> {
    let runtime_dir = get_runtime_dir()?;
    let node_exe = if cfg!(target_os = "windows") {
        runtime_dir.join("node.exe")
    } else {
        runtime_dir.join("bin").join("node")
    };
    
    // Check if already downloaded
    if node_exe.exists() {
        return Ok(node_exe);
    }
    
    // Download Node.js
    println!("📥 Downloading Node.js runtime (one-time setup)...");
    download_node(&runtime_dir)?;
    
    Ok(node_exe)
}

/// Get the runtime directory path
fn get_runtime_dir() -> Result<PathBuf> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .context("Could not determine home directory")?;
    
    let runtime_dir = PathBuf::from(home).join(".crabby").join("runtime");
    
    if !runtime_dir.exists() {
        std::fs::create_dir_all(&runtime_dir)?;
    }
    
    Ok(runtime_dir)
}

/// Download portable Node.js
fn download_node(runtime_dir: &PathBuf) -> Result<()> {
    use std::io::Write;
    
    // Determine Node.js download URL based on platform
    let (url, archive_name) = get_node_download_url()?;
    
    println!("Downloading from: {}", url);
    
    // Download the archive
    let response = reqwest::blocking::get(&url)
        .context("Failed to download Node.js")?
        .bytes()?;
    
    // Save to temp file
    let temp_file = runtime_dir.join(&archive_name);
    let mut file = std::fs::File::create(&temp_file)?;
    file.write_all(&response)?;
    
    // Extract archive
    println!("📦 Extracting Node.js...");
    extract_node_archive(&temp_file, runtime_dir)?;
    
    // Clean up temp file
    std::fs::remove_file(&temp_file)?;
    
    println!("✅ Node.js runtime installed!");
    
    Ok(())
}

/// Get Node.js download URL for current platform
fn get_node_download_url() -> Result<(String, String)> {
    let version = "v20.11.0"; // LTS version
    
    let (os, arch, ext) = if cfg!(target_os = "windows") {
        if cfg!(target_arch = "x86_64") {
            ("win", "x64", "zip")
        } else {
            ("win", "x86", "zip")
        }
    } else if cfg!(target_os = "macos") {
        ("darwin", "x64", "tar.gz")
    } else {
        ("linux", "x64", "tar.xz")
    };
    
    let filename = format!("node-{}-{}-{}", version, os, arch);
    let url = format!(
        "https://nodejs.org/dist/{}/{}.{}",
        version, filename, ext
    );
    
    Ok((url, format!("{}.{}", filename, ext)))
}

/// Extract Node.js archive
fn extract_node_archive(archive_path: &PathBuf, dest_dir: &PathBuf) -> Result<()> {
    if cfg!(target_os = "windows") {
        // Extract ZIP on Windows
        let file = std::fs::File::open(archive_path)?;
        let mut archive = zip::ZipArchive::new(file)?;
        
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = match file.enclosed_name() {
                Some(path) => dest_dir.join(path),
                None => continue,
            };
            
            if file.name().ends_with('/') {
                std::fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(p)?;
                    }
                }
                let mut outfile = std::fs::File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
        }
    } else {
        // Extract tar.gz/tar.xz on Unix
        use flate2::read::GzDecoder;
        use tar::Archive;
        
        let tar_gz = std::fs::File::open(archive_path)?;
        let tar = GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);
        archive.unpack(dest_dir)?;
    }
    
    Ok(())
}
