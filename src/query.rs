// this_file: fontgrepc/src/query.rs
//
// Query implementation for finding fonts

use crate::{
    cache::FontCache,
    font::FontInfo,
    matchers::{
        AxesMatcher, CodepointsMatcher, FeaturesMatcher, FontMatcher, NameMatcher, ScriptsMatcher,
        TablesMatcher, VariableFontMatcher,
    },
    FontgrepcError, Result,
};
use regex::Regex;
use skrifa::Tag;
use std::{path::PathBuf, str::FromStr, time::Instant};

/// Query criteria for finding fonts
#[derive(Debug, Default, Clone)]
pub struct QueryCriteria {
    /// Whether to match variable fonts
    pub variable: bool,

    /// Variation axes to match
    pub axes: Vec<String>,

    /// OpenType features to match
    pub features: Vec<String>,

    /// OpenType scripts to match
    pub scripts: Vec<String>,

    /// Font tables to match
    pub tables: Vec<String>,

    /// Name patterns to match
    pub name_patterns: Vec<String>,

    /// Codepoints to match
    pub codepoints: Vec<char>,

    /// Charset to match
    pub charset: String,
}

impl QueryCriteria {
    /// Check if the criteria is empty
    pub fn is_empty(&self) -> bool {
        !self.variable
            && self.axes.is_empty()
            && self.features.is_empty()
            && self.scripts.is_empty()
            && self.tables.is_empty()
            && self.name_patterns.is_empty()
            && self.codepoints.is_empty()
            && self.charset.is_empty()
    }

    /// Get the charset query
    pub fn get_charset_query(&self) -> Option<String> {
        if self.charset.is_empty() {
            None
        } else {
            Some(self.charset.clone())
        }
    }
}

/// Font query for finding fonts
pub struct FontQuery {
    /// Query criteria
    criteria: QueryCriteria,

    /// Font cache
    cache: FontCache,

    /// Compiled name regexes
    name_regexes: Vec<Regex>,
}

impl FontQuery {
    /// Create a new font query
    pub fn new(criteria: QueryCriteria, cache_path: Option<&str>) -> Result<Self> {
        // Compile name regexes
        let name_regexes = criteria
            .name_patterns
            .iter()
            .filter_map(|pattern| match Regex::new(pattern) {
                Ok(regex) => Some(regex),
                Err(e) => {
                    log::warn!("Invalid regex pattern '{}': {}", pattern, e);
                    None
                }
            })
            .collect();

        // Initialize cache
        let cache = FontCache::new(cache_path)?;

        Ok(Self {
            criteria,
            cache,
            name_regexes,
        })
    }

    /// Execute the query on the given paths
    pub fn execute(&self, paths: &[PathBuf]) -> Result<Vec<String>> {
        let start_time = Instant::now();

        let results = if paths.is_empty() {
            self.query_cache_all()?
        } else {
            self.query_cache_filtered(paths)?
        };

        let elapsed = start_time.elapsed();
        log::info!(
            "Query executed in {:.2}ms, found {} results",
            elapsed.as_secs_f64() * 1000.0,
            results.len()
        );

        Ok(results)
    }

    /// Query the cache for all fonts
    fn query_cache_all(&self) -> Result<Vec<String>> {
        // Get all fonts from the cache
        let font_paths = self.cache.get_all_font_paths()?;

        if font_paths.is_empty() {
            return Err(FontgrepcError::CacheNotInitialized);
        }

        // Filter fonts based on criteria
        let mut matching_fonts = Vec::new();

        for path in font_paths {
            if let Ok(Some(font_info)) = self.cache.get_font_info(&path) {
                if self.font_matches(&font_info)? {
                    matching_fonts.push(path);
                }
            }
        }

        Ok(matching_fonts)
    }

    /// Query the cache for fonts in the given paths
    fn query_cache_filtered(&self, paths: &[PathBuf]) -> Result<Vec<String>> {
        // Get all fonts from the cache
        let all_font_paths = self.cache.get_all_font_paths()?;

        if all_font_paths.is_empty() {
            return Err(FontgrepcError::CacheNotInitialized);
        }

        // Filter fonts based on paths and criteria
        let mut matching_fonts = Vec::new();

        for path_str in all_font_paths {
            let path = std::path::Path::new(&path_str);

            // Check if the path is within any of the specified directories
            let in_search_path = paths
                .iter()
                .any(|search_path| path.starts_with(search_path));

            if in_search_path {
                if let Ok(Some(font_info)) = self.cache.get_font_info(&path_str) {
                    if self.font_matches(&font_info)? {
                        matching_fonts.push(path_str);
                    }
                }
            }
        }

        Ok(matching_fonts)
    }

    /// Check if a font matches the criteria
    fn font_matches(&self, font_info: &FontInfo) -> Result<bool> {
        // Create matchers based on criteria
        let mut matchers: Vec<Box<dyn FontMatcher>> = Vec::new();

        if self.criteria.variable {
            matchers.push(Box::new(VariableFontMatcher::new()));
        }

        if !self.criteria.axes.is_empty() {
            matchers.push(Box::new(AxesMatcher::new(&self.criteria.axes)));
        }

        if !self.criteria.features.is_empty() {
            matchers.push(Box::new(FeaturesMatcher::new(&self.criteria.features)));
        }

        if !self.criteria.scripts.is_empty() {
            matchers.push(Box::new(ScriptsMatcher::new(&self.criteria.scripts)));
        }

        if !self.criteria.tables.is_empty() {
            let tables: Vec<Tag> = self
                .criteria
                .tables
                .iter()
                .map(|s| Tag::from_str(s).unwrap_or_default())
                .collect();
            matchers.push(Box::new(TablesMatcher::new(&tables)));
        }

        if !self.name_regexes.is_empty() {
            matchers.push(Box::new(NameMatcher::new(&self.name_regexes)));
        }

        let mut codepoints = self.criteria.codepoints.clone();
        if !self.criteria.charset.is_empty() {
            codepoints.extend(self.criteria.charset.chars());
        }

        if !codepoints.is_empty() {
            matchers.push(Box::new(CodepointsMatcher::new(&codepoints)));
        }

        // Check if the font matches all criteria
        Ok(matchers.iter().all(|matcher| matcher.matches(font_info)))
    }
}
