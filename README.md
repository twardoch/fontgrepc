# fontgrepc

A cache-based command-line tool for fast finding of font files.

## Overview

With `fontgrepc`, you can add folders containing font files to a cache and then search within that cache. 

`fontgrepc` is a Rust tool, and a sister project to [`fontgrep`](https://github.com/simoncozens/fontgrep). `fontgrep` performs live filesystem searches, and _is very fast_ for tens of thousands of fonts. `fontgrepc` uses a simple SQLite cache, and can be _slightly_ faster when searching within hundreds of thousands of fonts. 

## Installation

```bash
cargo install fontgrepc
```

or 

```bash
cargo install --git https://github.com/twardoch/fontgrepc
```


## Usage

### Adding Fonts to Cache

Before searching, you must add your fonts to the cache:

```bash
# Add fonts from a directory
fontgrepc add /folder/with/fonts

# Add fonts with verbose output (shows cache path)
fontgrepc add -v /folder/with/fonts

# Force re-adding of fonts
fontgrepc add --force /folder/with/fonts

# Specify a custom cache location
fontgrepc add --cache-path /path/to/cache.db /folder/with/fonts
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


## Configuration

* **Cache Location**: Use `--cache-path` to specify a custom cache location (defaults to user's data directory).
* **Parallel Jobs**: Use `-j/--jobs` to control the number of parallel jobs for adding fonts (defaults to CPU core count).
* **Verbose Output**: Use `-v/--verbose` for detailed logging and to display the cache path.

## Development

### Building from Source

```bash
git clone https://github.com/twardoch/fontgrepc.git
cd fontgrepc
cargo build --release
```

### Linting

```bash
cargo fmt --all && cargo clippy --all-targets --all-features -- -D warnings
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
