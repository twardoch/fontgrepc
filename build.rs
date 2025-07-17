// this_file: build.rs

use std::process::Command;
use std::env;

fn main() {
    // Get version from git tag, fallback to Cargo.toml version
    let version = get_version_from_git()
        .or_else(|| get_version_from_cargo())
        .unwrap_or_else(|| "unknown".to_string());
    
    println!("cargo:rustc-env=FONTGREPC_VERSION={}", version);
    
    // Also set the build timestamp
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=FONTGREPC_BUILD_TIMESTAMP={}", timestamp);
    
    // Set git commit hash if available
    if let Some(commit) = get_git_commit() {
        println!("cargo:rustc-env=FONTGREPC_GIT_COMMIT={}", commit);
    }
    
    // Tell Cargo to re-run this script if the git HEAD changes
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/heads/");
    println!("cargo:rerun-if-changed=.git/refs/tags/");
}

fn get_version_from_git() -> Option<String> {
    // Try to get version from git describe
    let output = Command::new("git")
        .args(&["describe", "--tags", "--exact-match", "HEAD"])
        .output()
        .ok()?;
    
    if output.status.success() {
        let tag = String::from_utf8(output.stdout).ok()?;
        let version = tag.trim();
        
        // Remove 'v' prefix if present
        if version.starts_with('v') {
            return Some(version[1..].to_string());
        }
        return Some(version.to_string());
    }
    
    // If exact match fails, try to get the latest tag and add commit info
    let output = Command::new("git")
        .args(&["describe", "--tags", "--always", "--dirty"])
        .output()
        .ok()?;
    
    if output.status.success() {
        let describe = String::from_utf8(output.stdout).ok()?;
        let version = describe.trim();
        
        // Parse git describe output (e.g., "v1.0.5-2-g1234567")
        if let Some(tag_part) = version.split('-').next() {
            if tag_part.starts_with('v') {
                return Some(format!("{}-dev", &tag_part[1..]));
            }
            return Some(format!("{}-dev", tag_part));
        }
        
        // Fallback to commit hash if no tags
        return Some(format!("0.0.0-dev-{}", version));
    }
    
    None
}

fn get_version_from_cargo() -> Option<String> {
    env::var("CARGO_PKG_VERSION").ok()
}

fn get_git_commit() -> Option<String> {
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .ok()?;
    
    if output.status.success() {
        let commit = String::from_utf8(output.stdout).ok()?;
        return Some(commit.trim().to_string());
    }
    
    None
}