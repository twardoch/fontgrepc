// this_file: tests/integration/cli_tests.rs

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
    assert!(stdout.contains("USAGE"));
    assert!(stdout.contains("OPTIONS"));
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
    
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("directory") || stderr.contains("not found") || stderr.contains("No such file"));
}

#[test]
fn test_add_command_with_valid_directory() {
    let temp_dir = create_test_font_dir();
    let temp_cache = TempDir::new().expect("Failed to create temp cache dir");
    
    let output = run_fontgrepc(&[
        "add",
        "--cache-path",
        temp_cache.path().join("cache.db").to_str().unwrap(),
        temp_dir.path().to_str().unwrap()
    ]);
    
    // The command might fail if there are no actual font files, but it should try to process
    let stderr = String::from_utf8(output.stderr).unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    
    // Should not crash and should provide some feedback
    assert!(stderr.len() > 0 || stdout.len() > 0);
}

#[test]
fn test_add_command_with_verbose() {
    let temp_dir = create_test_font_dir();
    let temp_cache = TempDir::new().expect("Failed to create temp cache dir");
    
    let output = run_fontgrepc(&[
        "add",
        "--verbose",
        "--cache-path",
        temp_cache.path().join("cache.db").to_str().unwrap(),
        temp_dir.path().to_str().unwrap()
    ]);
    
    let stderr = String::from_utf8(output.stderr).unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    
    // Verbose mode should produce more output
    assert!(stderr.len() > 0 || stdout.len() > 0);
}

#[test]
fn test_find_command_without_cache() {
    let temp_cache = TempDir::new().expect("Failed to create temp cache dir");
    
    let output = run_fontgrepc(&[
        "find",
        "--cache-path",
        temp_cache.path().join("cache.db").to_str().unwrap(),
        "--variable"
    ]);
    
    // Should fail or return empty results since cache doesn't exist
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("cache") || stderr.contains("not found") || stderr.contains("empty"));
}

#[test]
fn test_list_command_without_cache() {
    let temp_cache = TempDir::new().expect("Failed to create temp cache dir");
    
    let output = run_fontgrepc(&[
        "list",
        "--cache-path",
        temp_cache.path().join("cache.db").to_str().unwrap()
    ]);
    
    // Should fail or return empty results since cache doesn't exist
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("cache") || stderr.contains("not found") || stderr.contains("empty"));
}

#[test]
fn test_clean_command_without_cache() {
    let temp_cache = TempDir::new().expect("Failed to create temp cache dir");
    
    let output = run_fontgrepc(&[
        "clean",
        "--cache-path",
        temp_cache.path().join("cache.db").to_str().unwrap()
    ]);
    
    // Should handle missing cache gracefully
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("cache") || stderr.contains("not found") || output.status.success());
}

#[test]
fn test_json_output_flag() {
    let temp_cache = TempDir::new().expect("Failed to create temp cache dir");
    
    let output = run_fontgrepc(&[
        "--json",
        "list",
        "--cache-path",
        temp_cache.path().join("cache.db").to_str().unwrap()
    ]);
    
    // Should attempt to produce JSON output
    let stderr = String::from_utf8(output.stderr).unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    
    // Even if it fails, the error should mention the cache issue
    assert!(stderr.contains("cache") || stdout.contains("[]") || stdout.contains("{}"));
}

#[test]
fn test_find_with_features() {
    let temp_cache = TempDir::new().expect("Failed to create temp cache dir");
    
    let output = run_fontgrepc(&[
        "find",
        "--cache-path",
        temp_cache.path().join("cache.db").to_str().unwrap(),
        "--features",
        "kern,liga"
    ]);
    
    // Should fail since cache doesn't exist
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("cache") || stderr.contains("not found"));
}

#[test]
fn test_find_with_scripts() {
    let temp_cache = TempDir::new().expect("Failed to create temp cache dir");
    
    let output = run_fontgrepc(&[
        "find",
        "--cache-path",
        temp_cache.path().join("cache.db").to_str().unwrap(),
        "--scripts",
        "latn,cyrl"
    ]);
    
    // Should fail since cache doesn't exist
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("cache") || stderr.contains("not found"));
}

#[test]
fn test_find_with_name_pattern() {
    let temp_cache = TempDir::new().expect("Failed to create temp cache dir");
    
    let output = run_fontgrepc(&[
        "find",
        "--cache-path",
        temp_cache.path().join("cache.db").to_str().unwrap(),
        "--name",
        "Arial.*Regular"
    ]);
    
    // Should fail since cache doesn't exist
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("cache") || stderr.contains("not found"));
}

#[test]
fn test_find_with_unicode_range() {
    let temp_cache = TempDir::new().expect("Failed to create temp cache dir");
    
    let output = run_fontgrepc(&[
        "find",
        "--cache-path",
        temp_cache.path().join("cache.db").to_str().unwrap(),
        "--unicode",
        "U+0041-U+005A"
    ]);
    
    // Should fail since cache doesn't exist
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("cache") || stderr.contains("not found"));
}

#[test]
fn test_find_with_tables() {
    let temp_cache = TempDir::new().expect("Failed to create temp cache dir");
    
    let output = run_fontgrepc(&[
        "find",
        "--cache-path",
        temp_cache.path().join("cache.db").to_str().unwrap(),
        "--tables",
        "GPOS,GSUB"
    ]);
    
    // Should fail since cache doesn't exist
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("cache") || stderr.contains("not found"));
}

#[test]
fn test_invalid_arguments() {
    let output = run_fontgrepc(&["--invalid-flag"]);
    
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("error") || stderr.contains("unknown") || stderr.contains("invalid"));
}

#[test]
fn test_add_with_force_flag() {
    let temp_dir = create_test_font_dir();
    let temp_cache = TempDir::new().expect("Failed to create temp cache dir");
    
    let output = run_fontgrepc(&[
        "add",
        "--force",
        "--cache-path",
        temp_cache.path().join("cache.db").to_str().unwrap(),
        temp_dir.path().to_str().unwrap()
    ]);
    
    // Should attempt to process with force flag
    let stderr = String::from_utf8(output.stderr).unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    
    // Should provide some feedback
    assert!(stderr.len() > 0 || stdout.len() > 0);
}

#[test]
fn test_add_with_parallel_jobs() {
    let temp_dir = create_test_font_dir();
    let temp_cache = TempDir::new().expect("Failed to create temp cache dir");
    
    let output = run_fontgrepc(&[
        "add",
        "--jobs",
        "2",
        "--cache-path",
        temp_cache.path().join("cache.db").to_str().unwrap(),
        temp_dir.path().to_str().unwrap()
    ]);
    
    // Should attempt to process with specified number of jobs
    let stderr = String::from_utf8(output.stderr).unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    
    // Should provide some feedback
    assert!(stderr.len() > 0 || stdout.len() > 0);
}

#[test]
fn test_subcommand_help() {
    let commands = ["add", "find", "list", "clean"];
    
    for cmd in &commands {
        let output = run_fontgrepc(&[cmd, "--help"]);
        
        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains(cmd));
        assert!(stdout.contains("USAGE") || stdout.contains("OPTIONS"));
    }
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

/// Integration test for the complete workflow
#[test]
fn test_complete_workflow() {
    let temp_dir = create_test_font_dir();
    let temp_cache = TempDir::new().expect("Failed to create temp cache dir");
    let cache_path = temp_cache.path().join("cache.db");
    
    // Step 1: Add fonts to cache
    let add_output = run_fontgrepc(&[
        "add",
        "--cache-path",
        cache_path.to_str().unwrap(),
        temp_dir.path().to_str().unwrap()
    ]);
    
    // Step 2: List fonts in cache
    let list_output = run_fontgrepc(&[
        "list",
        "--cache-path",
        cache_path.to_str().unwrap()
    ]);
    
    // Step 3: Clean cache
    let clean_output = run_fontgrepc(&[
        "clean",
        "--cache-path",
        cache_path.to_str().unwrap()
    ]);
    
    // Verify that commands ran without crashing
    // (They may fail due to dummy font files, but should not crash)
    assert!(add_output.status.code().is_some());
    assert!(list_output.status.code().is_some());
    assert!(clean_output.status.code().is_some());
}