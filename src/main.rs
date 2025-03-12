// this_file: fontgrepc/src/main.rs
//
// Main entry point for the fontgrepc application

use clap::Parser;
use fontgrepc::cli;
use log::{error, info};
use std::process;

/// Main entry point for the fontgrepc application
fn main() {
    // Parse command-line arguments
    let cli = cli::Cli::parse();

    // Log startup information
    info!("fontgrepc v{}", env!("CARGO_PKG_VERSION"));
    info!("A cache-based tool for fast font searching");

    // Run the application and handle errors
    if let Err(e) = cli::execute(cli) {
        error!("Error: {}", e);

        // Provide helpful guidance for common errors
        match e {
            fontgrepc::FontgrepcError::CacheNotInitialized => {
                error!("The cache is empty. Please add fonts first using 'fontgrepc add /path/to/fonts'");
            }
            fontgrepc::FontgrepcError::FontNotInCache(path) => {
                error!(
                    "Font '{}' not found in cache. Make sure it has been added.",
                    path
                );
            }
            fontgrepc::FontgrepcError::Cache(msg) => {
                error!(
                    "Cache error: {}. Try running 'fontgrepc clean' to fix cache issues.",
                    msg
                );
            }
            _ => {}
        }

        process::exit(1);
    }
}
