// this_file: fontgrepc/src/lib.rs
//
// Main library entry point for fontgrepc

use thiserror::Error;

/// Default batch size for database operations
pub const DEFAULT_BATCH_SIZE: usize = 100;

/// Error type for fontgrepc
#[derive(Error, Debug)]
pub enum FontgrepcError {
    /// IO errors (only used during indexing)
    #[error("IO error: {0}")]
    Io(String),

    /// Font parsing errors
    #[error("Font error: {0}")]
    Font(String),

    /// Database errors
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    /// Cache errors
    #[error("Cache error: {0}")]
    Cache(String),

    /// Cache not initialized
    #[error("Cache not initialized. Run 'fontgrepc index' first.")]
    CacheNotInitialized,

    /// Font not in cache
    #[error("Font not found in cache: {0}")]
    FontNotInCache(String),

    /// Parsing errors
    #[error("Parse error: {0}")]
    Parse(String),

    /// Connection pool errors
    #[error("Connection pool error: {0}")]
    Pool(#[from] r2d2::Error),

    /// Serialization errors
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Regex errors
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    /// Other errors
    #[error("Error: {0}")]
    Other(String),
}

/// Result type for fontgrepc
pub type Result<T> = std::result::Result<T, FontgrepcError>;

// Modules
mod cache;
pub mod cli;
mod font;
pub mod matchers;
mod query;
mod utils;

// Re-export key types
pub use cache::CacheStatistics;
pub use font::FontInfo;
pub use query::FontQuery;

// Implement From for common error types
impl From<std::io::Error> for FontgrepcError {
    fn from(err: std::io::Error) -> Self {
        FontgrepcError::Io(format!("{:?}: {}", err.kind(), err))
    }
}

impl From<String> for FontgrepcError {
    fn from(err: String) -> Self {
        FontgrepcError::Other(err)
    }
}

impl From<&str> for FontgrepcError {
    fn from(err: &str) -> Self {
        FontgrepcError::Other(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let fontgrepc_err: FontgrepcError = io_err.into();

        match fontgrepc_err {
            FontgrepcError::Io(msg) => {
                assert!(msg.contains("file not found"));
                assert!(msg.contains("NotFound"));
            }
            _ => panic!("Expected Io error"),
        }

        let str_err = "test error";
        let fontgrepc_err: FontgrepcError = str_err.into();

        match fontgrepc_err {
            FontgrepcError::Other(msg) => assert_eq!(msg, "test error"),
            _ => panic!("Expected Other error"),
        }
    }

    #[test]
    fn test_cache_errors() {
        let not_init_err = FontgrepcError::CacheNotInitialized;
        assert!(format!("{}", not_init_err).contains("Run 'fontgrepc index' first"));

        let not_found_err = FontgrepcError::FontNotInCache("test.ttf".to_string());
        assert!(format!("{}", not_found_err).contains("test.ttf"));
    }
}
