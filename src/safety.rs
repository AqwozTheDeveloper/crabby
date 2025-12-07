use anyhow::{Context, Result};
use sha1::{Sha1, Digest};
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Verify file integrity using SHA-1 checksum (npm registry format)
pub fn verify_checksum(file_path: &Path, expected_checksum: Option<&str>) -> Result<bool> {
    if expected_checksum.is_none() {
        // No checksum provided, skip verification
        return Ok(true);
    }
    
    let expected = expected_checksum.unwrap();
    
    let mut file = File::open(file_path)
        .context("Failed to open file for checksum verification")?;
    
    let mut hasher = Sha1::new();
    let mut buffer = [0; 8192];
    
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    
    let result = hasher.finalize();
    let actual = format!("{:x}", result);
    
    Ok(actual == expected)
}

/// Calculate SHA-1 checksum of a file (npm registry format)
pub fn calculate_checksum(file_path: &Path) -> Result<String> {
    let mut file = File::open(file_path)
        .context("Failed to open file for checksum calculation")?;
    
    let mut hasher = Sha1::new();
    let mut buffer = [0; 8192];
    
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

/// Create a backup of a file or directory
pub fn create_backup(path: &Path) -> Result<std::path::PathBuf> {
    use std::time::SystemTime;
    
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();
    
    let backup_name = format!("{}.backup.{}", 
        path.file_name().unwrap().to_string_lossy(),
        timestamp
    );
    
    let backup_path = path.parent()
        .unwrap_or_else(|| Path::new("."))
        .join(backup_name);
    
    if path.is_file() {
        std::fs::copy(path, &backup_path)?;
    } else if path.is_dir() {
        copy_dir_recursive(path, &backup_path)?;
    }
    
    Ok(backup_path)
}

/// Recursively copy a directory
fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    std::fs::create_dir_all(dst)?;
    
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if file_type.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }
    
    Ok(())
}

/// Validate package.json structure
pub fn validate_package_json(content: &str) -> Result<()> {
    let _: serde_json::Value = serde_json::from_str(content)
        .context("Invalid JSON in package.json")?;
    
    // Additional validation can be added here
    Ok(())
}

/// Validate lock file integrity
pub fn validate_lockfile(lockfile: &crate::manifest::CrabbyLock) -> Result<()> {
    // Check for circular dependencies
    // Check for missing packages
    // Validate version strings
    
    for (name, info) in &lockfile.dependencies {
        if name.is_empty() {
            anyhow::bail!("Empty package name in lock file");
        }
        
        if info.version.is_empty() {
            anyhow::bail!("Empty version for package: {}", name);
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    
    #[test]
    fn test_checksum_calculation() {
        let temp_file = std::env::temp_dir().join("test_checksum.txt");
        let mut file = File::create(&temp_file).unwrap();
        file.write_all(b"Hello, World!").unwrap();
        
        let checksum = calculate_checksum(&temp_file).unwrap();
        assert!(!checksum.is_empty());
        
        std::fs::remove_file(temp_file).ok();
    }
    
    #[test]
    fn test_validate_package_json() {
        let valid_json = r#"{"name": "test", "version": "1.0.0"}"#;
        assert!(validate_package_json(valid_json).is_ok());
        
        let invalid_json = r#"{"name": "test", "version": }"#;
        assert!(validate_package_json(invalid_json).is_err());
    }
}
