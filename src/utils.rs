// this_file: fontgrepc/src/utils.rs
//
// Utility functions for fontgrepc

use crate::{FontgrepcError, Result};
use std::{
    path::{Path, PathBuf},
    time::SystemTime,
};

/// Get the modification time of a file
pub fn get_file_mtime(path: &Path) -> Result<i64> {
    let metadata = std::fs::metadata(path)?;
    let mtime = metadata
        .modified()?
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|e| FontgrepcError::Io(format!("Invalid modification time: {}", e)))?
        .as_secs() as i64;
    Ok(mtime)
}

/// Get the size of a file
pub fn get_file_size(path: &Path) -> Result<i64> {
    let metadata = std::fs::metadata(path)?;
    Ok(metadata.len() as i64)
}

/// Determine the cache path
pub fn determine_cache_path(custom_path: Option<&str>) -> Result<PathBuf> {
    if let Some(path) = custom_path {
        return Ok(PathBuf::from(path));
    }

    // Get the user's data directory
    let data_dir = dirs::data_dir()
        .ok_or_else(|| FontgrepcError::Cache("Could not determine data directory".to_string()))?;

    // Create the cache directory
    let cache_dir = data_dir.join("fontgrepc");
    std::fs::create_dir_all(&cache_dir)?;

    // Return the cache file path
    Ok(cache_dir.join("fonts.db"))
}
