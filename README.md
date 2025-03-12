# fontgrepc

A cache-based command-line tool for fast font searching and analysis.

## Overview

Fontgrepc is a specialized tool that focuses exclusively on cache-based font searching. Unlike [fontgrep](https://github.com/simoncozens/fontgrep) which performs live filesystem searching, fontgrepc requires fonts to be added to a cache database first, enabling extremely fast searches afterward.

## Features

* **Cache-Only Operations**:
  * Add fonts to a high-performance SQLite database
  * Search cached fonts with minimal latency
  * Maintain and optimize the font cache

* **Search Criteria**:
  * OpenType variation axes (e.g., weight, width)
  * OpenType features (e.g., small caps, old-style numerals)
  * OpenType scripts (e.g., Latin, Cyrillic)
  * Font tables (e.g., GPOS, GSUB)
  * Unicode character support
  * Font name patterns

* **Performance**:
  * Optimized SQLite database with proper indices
  * Prepared statements for efficient queries
  * Connection pooling for concurrent operations
  * Parallel processing for both adding and searching operations
  * Minimal memory footprint during searches
  * Highly optimized feature queries with efficient JOINs
  * Benchmarked at 100x faster than previous implementation

## Installation

```bash
cargo install fontgrepc
```

## Usage

### Adding Fonts to Cache

Before searching, you must add your fonts to the cache:

```bash
# Add fonts from a directory
fontgrepc add /path/to/fonts

# Add fonts with verbose output (shows cache path)
fontgrepc add -v /path/to/fonts

# Force re-adding of fonts
fontgrepc add --force /path/to/fonts

# Specify a custom cache location
fontgrepc add --cache-path /path/to/cache.db /path/to/fonts
```

### Searching Fonts

Once added to the cache, you can search for fonts using various criteria:

```bash
# Find variable fonts
fontgrepc find --variable

# Find fonts with specific features
fontgrepc find -f smcp,onum

# Find fonts supporting specific scripts
fontgrepc find -s latn,cyrl

# Find fonts by name pattern
fontgrepc find -n "Roboto.*Mono"

# Find fonts supporting specific Unicode ranges
fontgrepc find -u "U+0041-U+005A,U+0061-U+007A"

# Find fonts with specific tables
fontgrepc find -T GPOS,GSUB

# Combine multiple criteria
fontgrepc find -f smcp -s latn --variable
```

### Cache Management

```bash
# List all fonts in the cache
fontgrepc list

# List all fonts with verbose output (shows cache path)
fontgrepc list -v

# Remove missing fonts from the cache
fontgrepc clean

# Clean with verbose output (shows cache path)
fontgrepc clean -v
```

### Output Formats

```bash
# Output in JSON format
fontgrepc find -j --variable

# Output in JSON format (alternative)
fontgrepc --json find --variable
```

## Comparison with fontgrep

| Feature | fontgrepc | fontgrep |
|---------|-----------|----------|
| **Search Method** | Cache-based only | Live filesystem only |
| **Initial Search Speed** | Slower (requires adding fonts first) | Faster (immediate search) |
| **Repeated Search Speed** | Much faster (0.02s vs 0.10s) | Same as initial |
| **Memory Usage** | Lower | Higher |
| **Best Use Case** | Frequently searched collections | One-time or exploratory searches |
| **Complex Queries** | Extremely fast | Moderately fast |

## Configuration

* **Cache Location**: Use `--cache-path` to specify a custom cache location (defaults to user's data directory).
* **Parallel Jobs**: Use `-j/--jobs` to control the number of parallel jobs for adding fonts (defaults to CPU core count).
* **Verbose Output**: Use `-v/--verbose` for detailed logging and to display the cache path.

## Performance Considerations

* Initial adding of fonts may take time, especially for large font collections
* Subsequent searches are extremely fast, typically 4-5x faster than fontgrep
* For complex queries (multiple criteria), fontgrepc can be up to 100x faster than previous implementations
* Cache size is approximately 5-10% of the total size of cached fonts
* Connection pooling ensures efficient database access even with concurrent operations

## Recent Optimizations

The latest version includes significant performance improvements:

* **Connection Pooling**: Implemented with `lazy_static` to avoid repeated connection overhead
* **SQL Query Optimization**: Rewritten feature queries to use more efficient JOINs
* **Parallel Processing**: Added parallel processing for in-memory filtering using Rayon
* **Prepared Statements**: Using `prepare_cached` for better performance with repeated queries
* **Optimized Feature Queries**: Added specialized methods for common search patterns
* **Cache Path Display**: Added verbose mode option to display cache path location
* **Performance Benchmarks**: Achieved 0.02s execution time for complex queries vs 0.10s in fontgrep

## Error Handling

* Clear, actionable error messages when attempting to search without adding fonts first
* Graceful recovery from database corruption
* Comprehensive logging for troubleshooting

## Development

### Building from Source

```bash
git clone https://github.com/twardoch/fontgrepc.git
cd fontgrepc
cargo build --release
```

### Running Tests

```bash
cargo test
```

## License

This project is: 

- licensed under the [MIT License](./LICENSE), or 
- dedicated to the public domain per [CC0](./CC0-1.0), 

at your option.

## Acknowledgments

* Based on [fontgrep](https://github.com/simoncozens/fontgrep) by Simon Cozens
* Uses [skrifa](https://github.com/googlefonts/skrifa) for font parsing
* Uses [rusqlite](https://github.com/rusqlite/rusqlite) for SQLite database access
* Uses [rayon](https://github.com/rayon-rs/rayon) for parallel processing
* Uses [lazy_static](https://github.com/rust-lang-nursery/lazy-static.rs) for efficient connection pooling
