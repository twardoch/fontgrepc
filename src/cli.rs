// this_file: fontgrepc/src/cli.rs
//
// Command-line interface for fontgrepc

use crate::{
    cache::{CacheStatistics, FontCache},
    font::FontInfo,
    query::{FontQuery, QueryCriteria},
    utils::{get_file_mtime, get_file_size},
    FontgrepcError, Result,
};
use clap::{Args as ClapArgs, Parser, Subcommand};
use jwalk::WalkDir;
use rayon;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    time::Instant,
};

/// Command-line arguments for fontgrepc
#[derive(Parser, Debug)]
#[command(
    author = "fontgrepc authors",
    version,
    about = "find fonts in a cache, based on various criteria",
    long_about = "fontgrepc: CLI tool that finds fonts in a cahce that contain specified features, axes, codepoints, scripts"
)]
pub struct Cli {
    /// Subcommand to execute
    #[command(subcommand)]
    pub command: Commands,

    /// Path to the cache file
    #[arg(
        long,
        value_name = "FILE",
        help = "Path to the cache file",
        long_help = "Path to the SQLite database file used for caching font information. \
                    If not specified, defaults to a file in the user's data directory."
    )]
    pub cache_path: Option<String>,

    /// Enable verbose output
    #[arg(
        short,
        long,
        help = "Enable verbose output",
        long_help = "Enable verbose output mode that shows additional information \
                    about the search process and font properties."
    )]
    pub verbose: bool,

    /// Output as JSON
    #[arg(
        short,
        long,
        help = "Output as JSON",
        long_help = "Output results in JSON format for machine processing. \
                    If not specified, results are output as human-readable text."
    )]
    pub json: bool,
}

/// Subcommands for fontgrepc
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Find fonts in the cache based on various criteria
    Find(SearchArgs),

    /// Add fonts to the cache
    Add(UpdateArgs),

    /// List all fonts in the cache
    List,

    /// Remove missing fonts from the cache
    Clean,
}

/// Arguments for the search command
#[derive(ClapArgs, Debug)]
pub struct SearchArgs {
    /// Directories or font files to search
    #[arg(
        help = "Directories or font files to search (optional)",
        long_help = "One or more directories or font files to search. \
                    If not specified, searches all fonts in the cache. \
                    Directories are used to filter cached fonts by path."
    )]
    pub paths: Vec<PathBuf>,

    /// Variation axes to search for
    #[arg(
        short,
        long,
        value_delimiter = ',',
        help = "Variation axes to search for (e.g., wght,wdth)",
        long_help = "Comma-separated list of OpenType variation axes to search for. \
                    Common axes include:\n\
                    - wght: Weight\n\
                    - wdth: Width\n\
                    - ital: Italic\n\
                    - slnt: Slant\n\
                    - opsz: Optical Size"
    )]
    pub axes: Vec<String>,

    /// OpenType features to search for
    #[arg(
        short,
        long,
        value_delimiter = ',',
        help = "OpenType features to search for (e.g., smcp,onum)",
        long_help = "Comma-separated list of OpenType features to search for. \
                    Common features include:\n\
                    - smcp: Small Capitals\n\
                    - onum: Oldstyle Numerals\n\
                    - liga: Standard Ligatures\n\
                    - kern: Kerning\n\
                    - dlig: Discretionary Ligatures"
    )]
    pub features: Vec<String>,

    /// OpenType scripts to search for
    #[arg(
        short,
        long,
        value_delimiter = ',',
        help = "OpenType scripts to search for (e.g., latn,cyrl)",
        long_help = "Comma-separated list of OpenType script tags to search for. \
                    Common scripts include:\n\
                    - latn: Latin\n\
                    - cyrl: Cyrillic\n\
                    - grek: Greek\n\
                    - arab: Arabic\n\
                    - deva: Devanagari"
    )]
    pub scripts: Vec<String>,

    /// Font tables to search for
    #[arg(
        short = 'T',
        long,
        value_delimiter = ',',
        help = "Font tables to search for (e.g., GPOS,GSUB)",
        long_help = "Comma-separated list of OpenType table tags to search for. \
                    Common tables include:\n\
                    - GPOS: Glyph Positioning\n\
                    - GSUB: Glyph Substitution\n\
                    - GDEF: Glyph Definition\n\
                    - BASE: Baseline\n\
                    - OS/2: OS/2 and Windows Metrics"
    )]
    pub tables: Vec<String>,

    /// Only show variable fonts
    #[arg(
        short,
        long,
        help = "Only show variable fonts",
        long_help = "Only show variable fonts that support OpenType Font Variations."
    )]
    pub variable: bool,

    /// Regular expressions to match against font names
    #[arg(
        short,
        long,
        help = "Regular expressions to match against font names",
        long_help = "One or more regular expressions to match against font names. \
                    The search is case-insensitive and matches anywhere in the name."
    )]
    pub name: Vec<String>,

    /// Unicode codepoints or ranges to search for
    #[arg(
        short = 'u',
        long,
        value_delimiter = ',',
        help = "Unicode codepoints or ranges to search for (e.g., U+0041-U+005A,U+0061)",
        long_help = "Comma-separated list of Unicode codepoints or ranges to search for. \
                    Formats accepted:\n\
                    - Single codepoint: U+0041 or 0041\n\
                    - Range: U+0041-U+005A\n\
                    - Single character: A"
    )]
    pub codepoints_str: Vec<String>,

    /// Text to check for support
    #[arg(
        short,
        long,
        help = "Text to check for support",
        long_help = "Text string to check for font support. \
                    All characters in the text must be supported by the font."
    )]
    pub text: Option<String>,
}

/// Arguments for the update command
#[derive(ClapArgs, Debug)]
pub struct UpdateArgs {
    /// Directories or font files to index
    #[arg(
        required = true,
        help = "Directories or font files to index",
        long_help = "One or more directories or font files to index into the cache. \
                    Directories will be searched recursively for font files."
    )]
    pub paths: Vec<PathBuf>,

    /// Force update even if fonts haven't changed
    #[arg(
        short,
        long,
        help = "Force update even if fonts haven't changed",
        long_help = "Force update of fonts in the cache even if they haven't changed \
                    since the last update (based on modification time and file size)."
    )]
    pub force: bool,

    /// Number of parallel jobs to use
    #[arg(
        short,
        long,
        default_value_t = num_cpus::get(),
        help = "Number of parallel jobs to use",
        long_help = "Number of parallel jobs to use for indexing fonts. \
                    Defaults to the number of CPU cores available."
    )]
    pub jobs: usize,
}

/// Execute the command
pub fn execute(cli: Cli) -> Result<()> {
    // Set up logging
    if cli.verbose {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    } else {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    }

    match &cli.command {
        Commands::Find(args) => {
            // Create query criteria
            let criteria = QueryCriteria {
                variable: args.variable,
                axes: args.axes.clone(),
                features: args.features.clone(),
                scripts: args.scripts.clone(),
                tables: args.tables.clone(),
                name_patterns: args.name.clone(),
                codepoints: parse_codepoints_from_strings(&args.codepoints_str)?,
                charset: args.text.clone().unwrap_or_default(),
            };

            // Create query
            let query = FontQuery::new(criteria, cli.cache_path.as_deref())?;

            // Execute query
            let start_time = Instant::now();
            let results = query.execute(&args.paths)?;
            let elapsed = start_time.elapsed();

            // Output results
            if cli.json {
                let json = serde_json::to_string_pretty(&results)?;
                println!("{}", json);
            } else if cli.verbose {
                // In verbose mode, show detailed information
                println!(
                    "Found {} fonts in {:.2}ms:",
                    results.len(),
                    elapsed.as_secs_f64() * 1000.0
                );
                for result in results {
                    println!("{}", result);
                }
            } else {
                // In non-verbose mode, only show the paths
                for result in results {
                    println!("{}", result);
                }
            }
        }
        Commands::Add(args) => {
            // Initialize cache
            let cache = FontCache::new(cli.cache_path.as_deref())?;

            // In verbose mode, display the cache path
            if cli.verbose {
                println!("# Cache: {}", cache.get_path().display());
            }

            // Collect font files
            let font_files = collect_font_files(&args.paths)?;

            if font_files.is_empty() {
                println!("No font files found in the specified paths.");
                return Ok(());
            }

            // Process font files in parallel using rayon
            let start_time = Instant::now();

            // Create a thread pool with the specified number of threads
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(args.jobs)
                .build()
                .unwrap();

            // Use the thread pool to process fonts in parallel
            let (fonts_to_update, stats) = pool.install(|| {
                use rayon::prelude::*;

                // Process fonts in parallel and collect results
                let results: Vec<_> = font_files
                    .par_iter()
                    .map(|path| {
                        let path_str = path.to_string_lossy().to_string();
                        let mtime = match get_file_mtime(path) {
                            Ok(mtime) => mtime,
                            Err(e) => {
                                log::warn!("Error getting mtime for {}: {}", path.display(), e);
                                return (None, false);
                            }
                        };
                        let size = match get_file_size(path) {
                            Ok(size) => size,
                            Err(e) => {
                                log::warn!("Error getting size for {}: {}", path.display(), e);
                                return (None, false);
                            }
                        };

                        // Check if font needs to be updated
                        let needs_update = match cache.needs_update(&path_str, mtime, size) {
                            Ok(needs_update) => args.force || needs_update,
                            Err(e) => {
                                log::warn!(
                                    "Error checking if {} needs update: {}",
                                    path.display(),
                                    e
                                );
                                return (None, false);
                            }
                        };

                        if needs_update {
                            // Load font info
                            match FontInfo::load(path) {
                                Ok(info) => {
                                    // In verbose mode, print the path of each font being added
                                    if cli.verbose {
                                        println!("Adding: {}", path.display());
                                    }
                                    (Some((path_str, info, mtime, size)), true)
                                }
                                Err(e) => {
                                    log::warn!("Error loading font {}: {}", path.display(), e);
                                    (None, true)
                                }
                            }
                        } else {
                            (None, false)
                        }
                    })
                    .collect();

                // Process results
                let mut fonts_to_update = Vec::new();
                let mut processed = 0;
                let mut skipped = 0;
                let mut errors = 0;

                for (font_info, attempted) in results {
                    if let Some(info) = font_info {
                        fonts_to_update.push(info);
                        processed += 1;
                    } else if attempted {
                        errors += 1;
                    } else {
                        skipped += 1;
                    }
                }

                (fonts_to_update, (processed, skipped, errors))
            });

            let (processed, skipped, errors) = stats;

            // Update cache
            if !fonts_to_update.is_empty() {
                // Update cache
                cache.batch_update_fonts(fonts_to_update)?;
                let elapsed = start_time.elapsed();

                // Optimize cache silently
                cache.optimize()?;

                // Output results
                if cli.json {
                    let result = serde_json::json!({
                        "processed": processed,
                        "skipped": skipped,
                        "errors": errors,
                        "total": processed + skipped + errors,
                        "time_ms": elapsed.as_millis()
                    });
                    println!("{}", serde_json::to_string_pretty(&result)?);
                } else {
                    println!(
                        "Added {} fonts, skipped {} unchanged fonts, encountered {} errors in {:.2} seconds.",
                        processed,
                        skipped,
                        errors,
                        elapsed.as_secs_f64()
                    );
                }
            } else {
                println!("No fonts needed to be added to the cache.");
            }
        }
        Commands::List => {
            // Initialize cache
            let cache = FontCache::new(cli.cache_path.as_deref())?;

            // In verbose mode, display the cache path
            if cli.verbose {
                println!("# Cache: {}", cache.get_path().display());
            }

            // Get all fonts from cache
            let font_paths = cache.get_all_font_paths()?;

            // Output results
            if cli.json {
                let json = serde_json::to_string_pretty(&font_paths)?;
                println!("{}", json);
            } else if cli.verbose {
                // In verbose mode, show detailed information
                println!("Found {} fonts in cache:", font_paths.len());
                for path in font_paths {
                    println!("{}", path);
                }
            } else {
                // In non-verbose mode, only show the paths
                for path in font_paths {
                    println!("{}", path);
                }
            }
        }
        Commands::Clean => {
            // Initialize cache
            let cache = FontCache::new(cli.cache_path.as_deref())?;

            // In verbose mode, display the cache path
            if cli.verbose {
                println!("# Cache: {}", cache.get_path().display());
            }

            // Get all fonts from cache
            let font_paths = cache.get_all_font_paths()?;

            // Check which fonts exist
            let mut existing_paths = HashSet::new();
            for path in font_paths {
                if Path::new(&path).exists() {
                    existing_paths.insert(path);
                }
            }

            // Clean missing fonts
            let start_time = Instant::now();
            let removed = cache.clean_missing_fonts(&existing_paths)?;
            let elapsed = start_time.elapsed();

            // Output results
            if cli.json {
                let result = serde_json::json!({
                    "removed": removed,
                    "remaining": existing_paths.len(),
                    "time_ms": elapsed.as_millis()
                });
                println!("{}", serde_json::to_string_pretty(&result)?);
            } else if cli.verbose {
                // In verbose mode, show detailed information
                println!(
                    "Removed {} missing fonts, {} fonts remain in cache",
                    removed,
                    existing_paths.len()
                );
            } else {
                // In non-verbose mode, only show minimal output if something was removed
                if removed > 0 {
                    println!("Removed {} fonts", removed);
                }
            }

            // Optimize cache
            if removed > 0 {
                if !cli.json && cli.verbose {
                    println!("Optimizing cache...");
                }
                cache.optimize()?;
            }
        }
    }

    Ok(())
}

/// Print cache statistics in a human-readable format
#[allow(dead_code)]
fn print_cache_statistics(stats: &CacheStatistics) {
    println!("Cache Statistics:");
    println!("----------------");
    println!("Total fonts: {}", stats.font_count);
    println!("Variable fonts: {}", stats.variable_font_count);
    println!(
        "Database size: {:.2} MB",
        stats.database_size_bytes as f64 / 1_048_576.0
    );
    println!("Last updated: {}", stats.last_updated);

    if !stats.feature_counts.is_empty() {
        println!("\nTop 10 OpenType Features:");
        for (i, (feature, count)) in stats.feature_counts.iter().take(10).enumerate() {
            println!("  {}. {} ({} fonts)", i + 1, feature, count);
        }
    }

    if !stats.script_counts.is_empty() {
        println!("\nTop 10 OpenType Scripts:");
        for (i, (script, count)) in stats.script_counts.iter().take(10).enumerate() {
            println!("  {}. {} ({} fonts)", i + 1, script, count);
        }
    }
}

/// Collect font files from the specified paths
fn collect_font_files(paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let mut font_files = Vec::new();

    for path in paths {
        if path.is_file() && crate::font::is_font_file(path) {
            font_files.push(path.clone());
        } else if path.is_dir() {
            // Walk directory recursively using a simpler approach
            for entry in WalkDir::new(path).follow_links(true) {
                match entry {
                    Ok(entry) => {
                        if entry.file_type().is_file() {
                            let entry_path = entry.path();
                            if crate::font::is_font_file(&entry_path) {
                                font_files.push(entry_path);
                            }
                        }
                    }
                    Err(e) => {
                        log::warn!("Error walking directory {}: {}", path.display(), e);
                    }
                }
            }
        } else {
            log::warn!("Path does not exist: {}", path.display());
        }
    }

    if font_files.is_empty() {
        log::warn!(
            "No font files found in the specified paths. Searched in: {:?}",
            paths
        );
    } else {
        log::debug!("Found {} font files", font_files.len());
    }

    Ok(font_files)
}

/// Parse codepoints from strings
pub fn parse_codepoints(input: &str) -> Result<Vec<char>> {
    let mut result = Vec::new();

    for item in input.split(',') {
        if item.contains('-') {
            // Parse range
            let parts: Vec<&str> = item.split('-').collect();
            if parts.len() != 2 {
                return Err(FontgrepcError::Parse(format!(
                    "Invalid codepoint range: {}",
                    item
                )));
            }

            let start = parse_codepoint(parts[0])?;
            let end = parse_codepoint(parts[1])?;

            let start_u32 = start as u32;
            let end_u32 = end as u32;

            if start_u32 > end_u32 {
                return Err(FontgrepcError::Parse(format!(
                    "Invalid codepoint range: {} > {}",
                    start_u32, end_u32
                )));
            }

            for cp in start_u32..=end_u32 {
                if let Some(c) = char::from_u32(cp) {
                    result.push(c);
                }
            }
        } else {
            // Parse single codepoint
            result.push(parse_codepoint(item)?);
        }
    }

    Ok(result)
}

/// Parse a single codepoint from a string
fn parse_codepoint(input: &str) -> Result<char> {
    if input.len() == 1 {
        // Single character
        return Ok(input.chars().next().unwrap());
    }

    let input = input.trim_start_matches("U+").trim_start_matches("u+");
    let cp = u32::from_str_radix(input, 16)
        .map_err(|_| FontgrepcError::Parse(format!("Invalid codepoint: {}", input)))?;

    char::from_u32(cp)
        .ok_or_else(|| FontgrepcError::Parse(format!("Invalid Unicode codepoint: U+{:04X}", cp)))
}

/// Parse codepoints from strings
pub fn parse_codepoints_from_strings(input: &[String]) -> Result<Vec<char>> {
    let mut result = Vec::new();

    for item in input {
        result.extend(parse_codepoints(item)?);
    }

    Ok(result)
}
