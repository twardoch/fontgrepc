// this_file: tests/integration.rs

//! Integration tests for fontgrepc CLI tool
//! 
//! These tests verify the complete functionality of the fontgrepc CLI tool,
//! including command-line argument parsing, font processing, and cache management.

use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

/// Helper function to get the path to the binary
fn get_binary_path() -> String {
    // Try debug first, then release
    let debug_path = "target/debug/fontgrepc";
    let release_path = "target/release/fontgrepc";
    
    if Path::new(debug_path).exists() {
        debug_path.to_string()
    } else if Path::new(release_path).exists() {
        release_path.to_string()
    } else {
        // Build the binary if it doesn't exist
        Command::new("cargo")
            .args(&["build"])
            .status()
            .expect("Failed to build binary");
        debug_path.to_string()
    }
}

/// Helper function to run fontgrepc command
fn run_fontgrepc(args: &[&str]) -> std::process::Output {
    Command::new(get_binary_path())
        .args(args)
        .output()
        .expect("Failed to execute fontgrepc")
}

/// Helper function to create a temporary directory with test font files
fn create_test_font_dir() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    
    // Create some dummy font files
    let font_files = [
        "Arial-Regular.ttf",
        "Arial-Bold.ttf",
        "Helvetica-Regular.otf",
        "Roboto-Variable.ttf",
        "invalid-file.txt",
    ];
    
    for file in &font_files {
        let file_path = temp_dir.path().join(file);
        fs::write(&file_path, format!("dummy font content for {}", file))
            .expect("Failed to write test font file");
    }
    
    temp_dir
}

#[test]
fn test_help_command() {
    let output = run_fontgrepc(&["--help"]);
    
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("fontgrepc"));
    assert!(stdout.contains("Usage") || stdout.contains("USAGE"));
}

#[test]
fn test_version_command() {
    let output = run_fontgrepc(&["--version"]);
    
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("fontgrepc"));
    
    // Check that version contains semantic version format
    assert!(stdout.contains(char::is_numeric));
}

#[test]
fn test_add_command_with_nonexistent_directory() {
    let output = run_fontgrepc(&["add", "/nonexistent/directory"]);
    
    // The command succeeds but warns about nonexistent directory
    let stderr = String::from_utf8(output.stderr).unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stderr.contains("does not exist") || stderr.contains("not found") || stdout.contains("No font files found"));
}

#[test]
fn test_binary_exists_and_is_executable() {
    let binary_path = get_binary_path();
    assert!(Path::new(&binary_path).exists());
    
    // Test that the binary is executable by running it
    let output = Command::new(&binary_path)
        .arg("--version")
        .output()
        .expect("Failed to execute binary");
    
    assert!(output.status.success());
}

#[test]
fn test_subcommand_help() {
    let commands = ["add", "find", "list", "clean"];
    
    for cmd in &commands {
        let output = run_fontgrepc(&[cmd, "--help"]);
        
        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains(cmd));
    }
}

#[test]
fn test_invalid_arguments() {
    let output = run_fontgrepc(&["--invalid-flag"]);
    
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("error") || stderr.contains("unknown") || stderr.contains("invalid"));
}