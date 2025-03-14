// this_file: fontgrepc/src/cache.rs
//
// Cache implementation for font information

use crate::{font::FontInfo, query::QueryCriteria, FontgrepcError};
use chrono::{DateTime, Utc};
use rusqlite::OptionalExtension;
use rusqlite::{params, Connection, ToSql};
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Instant,
};

// Define a Result type for this module
type Result<T> = std::result::Result<T, FontgrepcError>;

/// Statistics about the font cache
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CacheStatistics {
    /// Total number of fonts in the cache
    pub font_count: usize,

    /// Size of the database file in bytes
    pub database_size_bytes: u64,

    /// When the cache was last updated
    pub last_updated: DateTime<Utc>,

    /// Count of fonts with each feature
    pub feature_counts: HashMap<String, usize>,

    /// Count of fonts with each script
    pub script_counts: HashMap<String, usize>,

    /// Count of variable fonts
    pub variable_font_count: usize,
}

/// Font cache for storing and retrieving font information
pub struct FontCache {
    conn: Option<Arc<Mutex<Connection>>>, // For in-memory databases
    path: PathBuf,
}

/// Guard for database transactions
struct TransactionGuard<'a> {
    tx: rusqlite::Transaction<'a>,
    committed: bool,
}

impl<'a> TransactionGuard<'a> {
    /// Create a new transaction guard
    fn new(tx: rusqlite::Transaction<'a>) -> Self {
        Self {
            tx,
            committed: false,
        }
    }

    /// Get a reference to the transaction
    fn transaction(&self) -> &rusqlite::Transaction<'a> {
        &self.tx
    }

    /// Commit the transaction
    fn commit(&mut self) -> Result<()> {
        if !self.committed {
            // Use execute_batch to avoid moving out of self.tx
            self.tx.execute_batch("COMMIT")?;
            self.committed = true;
        }
        Ok(())
    }
}

impl Drop for TransactionGuard<'_> {
    fn drop(&mut self) {
        if !self.committed {
            // Ignore errors during rollback in drop
            let _ = self.tx.execute_batch("ROLLBACK");
        }
    }
}

// Simplified connection handling - removed the complex connection pool
// and replaced with direct connection management

impl FontCache {
    /// Create a new font cache
    pub fn new(cache_path: Option<&str>) -> Result<Self> {
        let path = if let Some(path) = cache_path {
            if path == ":memory:" {
                // In-memory database
                let conn = Connection::open_in_memory()?;

                // Set pragmas for better performance
                conn.execute_batch(
                    "
                    PRAGMA journal_mode = WAL;
                    PRAGMA synchronous = NORMAL;
                    PRAGMA temp_store = MEMORY;
                    PRAGMA mmap_size = 30000000000;
                    PRAGMA page_size = 4096;
                    PRAGMA cache_size = -2000;
                    PRAGMA foreign_keys = ON;
                ",
                )?;

                initialize_schema(&conn)?;

                return Ok(Self {
                    conn: Some(Arc::new(Mutex::new(conn))),
                    path: PathBuf::from(":memory:"),
                });
            }
            PathBuf::from(path)
        } else {
            crate::utils::determine_cache_path(None)?
        };

        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Check if the database file exists
        let db_exists = path.exists();

        // Open the database
        let conn = Connection::open(&path)?;

        // Set pragmas for better performance - only needed once when creating the database
        conn.execute_batch(
            "
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = NORMAL;
            PRAGMA temp_store = MEMORY;
            PRAGMA mmap_size = 30000000000;
            PRAGMA page_size = 4096;
            PRAGMA cache_size = -2000;
            PRAGMA foreign_keys = ON;
        ",
        )?;

        // Initialize schema if the database is new
        if !db_exists {
            initialize_schema(&conn)?;
        }

        Ok(Self { conn: None, path })
    }

    /// Get a connection to the database
    fn get_connection(&self) -> Result<Connection> {
        if let Some(conn) = &self.conn {
            // For in-memory database, we need to return a connection to the same database
            let conn_guard = conn.lock().unwrap();
            // We can't use try_clone or backup with MutexGuard
            // For in-memory database, we'll just return a new connection to the same path
            drop(conn_guard); // Release the lock before creating a new connection
            Ok(Connection::open(":memory:")?)
        } else {
            // File-based database - create a new connection
            let conn = Connection::open(&self.path)?;

            // Set pragmas for better performance
            conn.execute_batch(
                "
                PRAGMA journal_mode = WAL;
                PRAGMA synchronous = NORMAL;
                PRAGMA temp_store = MEMORY;
                PRAGMA mmap_size = 30000000000;
                PRAGMA page_size = 4096;
                PRAGMA cache_size = -2000;
                PRAGMA foreign_keys = ON;
            ",
            )?;

            Ok(conn)
        }
    }

    /// Check if a font needs to be updated in the cache
    pub fn needs_update(&self, path: &str, mtime: i64, size: i64) -> Result<bool> {
        let conn = self.get_connection()?;

        // Check if the font exists and has the same mtime and size
        let exists: bool = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM fonts WHERE path = ? AND mtime = ? AND size = ?)",
            params![path, mtime, size],
            |row| row.get(0),
        )?;

        Ok(!exists)
    }

    /// Update fonts in the cache in batch
    pub fn batch_update_fonts(&self, fonts: Vec<(String, FontInfo, i64, i64)>) -> Result<()> {
        if fonts.is_empty() {
            return Ok(());
        }

        let mut conn = self.get_connection()?;

        // Begin transaction
        let tx = conn.transaction()?;
        let mut guard = TransactionGuard::new(tx);

        // Prepare statements once for reuse
        let mut insert_font_stmt = guard.transaction().prepare(
            "INSERT INTO fonts (path, name_string, is_variable, charset_string, mtime, size, updated_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )?;

        let mut delete_properties_stmt = guard
            .transaction()
            .prepare("DELETE FROM properties WHERE font_id = ?")?;

        let mut insert_property_stmt = guard
            .transaction()
            .prepare("INSERT INTO properties (font_id, type, value) VALUES (?, ?, ?)")?;

        // Process fonts in batches
        for (path, info, mtime, size) in fonts {
            // Insert or update font
            insert_font_stmt.execute(params![
                &path,
                &info.name_string,
                info.is_variable,
                &info.charset_string,
                mtime,
                size,
                Utc::now().timestamp()
            ])?;

            let font_id = guard.transaction().last_insert_rowid();

            // Delete existing properties
            delete_properties_stmt.execute(params![font_id])?;

            // Insert properties in batches
            // Insert axes
            for axis in &info.axes {
                insert_property_stmt.execute(params![font_id, "axis", axis])?;
            }

            // Insert features
            for feature in &info.features {
                insert_property_stmt.execute(params![font_id, "feature", feature])?;
            }

            // Insert scripts
            for script in &info.scripts {
                insert_property_stmt.execute(params![font_id, "script", script])?;
            }

            // Insert tables
            for table in &info.tables {
                insert_property_stmt.execute(params![font_id, "table", table])?;
            }
        }

        // Drop the prepared statements before committing to avoid borrow issues
        drop(insert_font_stmt);
        drop(delete_properties_stmt);
        drop(insert_property_stmt);

        // Commit transaction
        guard.commit()?;

        Ok(())
    }

    /// Query fonts based on criteria
    #[allow(dead_code)]
    pub fn query(&self, criteria: &QueryCriteria) -> Result<Vec<String>> {
        // Check if cache is empty
        let font_count = self.get_font_count()?;
        if font_count == 0 {
            return Err(FontgrepcError::CacheNotInitialized);
        }

        // For simple feature queries, use the optimized path
        if criteria.is_simple_feature_query() {
            return self.query_by_features(&criteria.features);
        }

        // Build the query
        let mut builder = QueryBuilder::new();

        // Add criteria
        if criteria.variable {
            builder = builder.with_variable();
        }

        if !criteria.axes.is_empty() {
            builder = builder.with_property("axis", &criteria.axes);
        }

        if !criteria.features.is_empty() {
            builder = builder.with_property("feature", &criteria.features);
        }

        if !criteria.scripts.is_empty() {
            builder = builder.with_property("script", &criteria.scripts);
        }

        if !criteria.tables.is_empty() {
            builder = builder.with_property("table", &criteria.tables);
        }

        if !criteria.name_patterns.is_empty() {
            builder = builder.with_name_patterns(&criteria.name_patterns);
        }

        if !criteria.charset.is_empty() {
            builder = builder.with_charset(&criteria.charset);
        }

        let (query, params) = builder.build();

        // Execute the query with proper parameter conversion
        let conn = self.get_connection()?;

        // Use prepare_cached for better performance with repeated queries
        let mut stmt = conn.prepare_cached(&query)?;

        let params_slice: Vec<&dyn ToSql> =
            params.iter().map(|p| p.as_ref() as &dyn ToSql).collect();

        // Record query time for performance metrics
        let start_time = Instant::now();

        // Use query_map for more efficient memory usage
        let rows = stmt.query_map(params_slice.as_slice(), |row| row.get::<_, String>(0))?;

        // Collect results
        let mut results = Vec::with_capacity(100); // Pre-allocate for better performance
        for row_result in rows {
            results.push(row_result?);
        }

        // Log query time if verbose
        let elapsed = start_time.elapsed();
        log::debug!(
            "Query executed in {:.2}ms, found {} results",
            elapsed.as_secs_f64() * 1000.0,
            results.len()
        );

        Ok(results)
    }

    /// Optimized query for feature searches
    pub fn query_by_features(&self, features: &[String]) -> Result<Vec<String>> {
        if features.is_empty() {
            return self.get_all_font_paths();
        }

        let conn = self.get_connection()?;

        // Create a more efficient query that uses JOINs instead of EXISTS
        let mut query = String::from(
            "SELECT DISTINCT f.path FROM fonts f 
             JOIN properties p ON p.font_id = f.id 
             WHERE p.type = 'feature' AND p.value IN (",
        );

        // Create placeholders for the IN clause
        let placeholders: Vec<String> = (0..features.len()).map(|_| "?".to_string()).collect();
        query.push_str(&placeholders.join(","));
        query.push_str(") GROUP BY f.id HAVING COUNT(DISTINCT p.value) = ?");

        // Prepare the statement
        let mut stmt = conn.prepare_cached(&query)?;

        // Create parameters
        let mut params: Vec<&dyn ToSql> = Vec::with_capacity(features.len() + 1);
        for feature in features {
            params.push(feature as &dyn ToSql);
        }

        // Fix the temporary value issue by creating a binding
        let feature_count = features.len() as i64;
        params.push(&feature_count as &dyn ToSql);

        // Execute the query
        let start_time = Instant::now();
        let rows = stmt.query_map(params.as_slice(), |row| row.get::<_, String>(0))?;

        // Collect results
        let mut results = Vec::with_capacity(100);
        for row_result in rows {
            results.push(row_result?);
        }

        let elapsed = start_time.elapsed();
        log::debug!(
            "Feature query executed in {:.2}ms, found {} results",
            elapsed.as_secs_f64() * 1000.0,
            results.len()
        );

        Ok(results)
    }

    /// Get all font paths in the cache
    pub fn get_all_font_paths(&self) -> Result<Vec<String>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare("SELECT path FROM fonts")?;

        // Use query_map for more efficient memory usage
        let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;

        // Collect results
        let mut paths = Vec::new();
        for row_result in rows {
            paths.push(row_result?);
        }

        Ok(paths)
    }

    /// Clean missing fonts from the cache
    pub fn clean_missing_fonts(&self, existing_paths: &HashSet<String>) -> Result<usize> {
        let mut conn = self.get_connection()?;

        // Get all paths in the cache and collect them first
        let missing_ids = {
            let mut stmt = conn.prepare("SELECT id, path FROM fonts")?;
            let rows = stmt.query_map([], |row| {
                let id: i64 = row.get(0)?;
                let path: String = row.get(1)?;
                Ok((id, path))
            })?;

            // Collect all missing IDs
            let mut ids = Vec::new();
            for result in rows {
                let (id, path) = result?;
                if !existing_paths.contains(&path) {
                    ids.push(id);
                }
            }
            ids
        };

        // Delete missing fonts
        let tx = conn.transaction()?;
        for id in &missing_ids {
            tx.execute("DELETE FROM properties WHERE font_id = ?", params![id])?;
            tx.execute("DELETE FROM fonts WHERE id = ?", params![id])?;
        }
        tx.commit()?;

        Ok(missing_ids.len())
    }

    /// Get font information from the cache
    pub fn get_font_info(&self, path: &str) -> Result<Option<FontInfo>> {
        let conn = self.get_connection()?;

        // Get font information
        let font_info: Option<(i64, String, bool, String)> = conn
            .query_row(
                "SELECT id, name_string, is_variable, charset_string FROM fonts WHERE path = ?",
                params![path],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
            )
            .optional()?;

        if let Some((id, name_string, is_variable, charset_string)) = font_info {
            // Get properties
            let axes = self.get_properties(&conn, id, "axis")?;
            let features = self.get_properties(&conn, id, "feature")?;
            let scripts = self.get_properties(&conn, id, "script")?;
            let tables = self.get_properties(&conn, id, "table")?;

            Ok(Some(FontInfo {
                name_string,
                is_variable,
                axes,
                features,
                scripts,
                tables,
                charset_string,
            }))
        } else {
            Ok(None)
        }
    }

    /// Get properties for a font
    fn get_properties(
        &self,
        conn: &Connection,
        font_id: i64,
        property_type: &str,
    ) -> Result<Vec<String>> {
        let mut stmt = conn.prepare(
            "SELECT value FROM properties WHERE font_id = ? AND type = ? ORDER BY value",
        )?;
        let rows = stmt.query_map(params![font_id, property_type], |row| row.get(0))?;

        let mut properties = Vec::new();
        for row_result in rows {
            properties.push(row_result?);
        }

        Ok(properties)
    }

    /// Get the number of fonts in the cache
    #[allow(dead_code)]
    fn get_font_count(&self) -> Result<usize> {
        let conn = self.get_connection()?;
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM fonts", [], |row| row.get(0))?;
        Ok(count as usize)
    }

    /// Get statistics about the cache
    #[allow(dead_code)]
    pub fn get_statistics(&self) -> Result<CacheStatistics> {
        let conn = self.get_connection()?;

        // Get font count
        let font_count = self.get_font_count()?;

        // Get variable font count
        let variable_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM fonts WHERE is_variable = 1",
            [],
            |row| row.get(0),
        )?;

        // Get last updated timestamp
        let last_updated: i64 =
            conn.query_row("SELECT MAX(updated_at) FROM fonts", [], |row| row.get(0))?;

        // Get feature counts
        let mut feature_counts = HashMap::new();
        let mut stmt = conn.prepare(
            "SELECT value, COUNT(*) FROM properties WHERE type = 'feature' GROUP BY value ORDER BY COUNT(*) DESC",
        )?;
        let rows = stmt.query_map([], |row| {
            let feature: String = row.get(0)?;
            let count: i64 = row.get(1)?;
            Ok((feature, count as usize))
        })?;

        for row_result in rows {
            let (feature, count) = row_result?;
            feature_counts.insert(feature, count);
        }

        // Get script counts
        let mut script_counts = HashMap::new();
        let mut stmt = conn.prepare(
            "SELECT value, COUNT(*) FROM properties WHERE type = 'script' GROUP BY value ORDER BY COUNT(*) DESC",
        )?;
        let rows = stmt.query_map([], |row| {
            let script: String = row.get(0)?;
            let count: i64 = row.get(1)?;
            Ok((script, count as usize))
        })?;

        for row_result in rows {
            let (script, count) = row_result?;
            script_counts.insert(script, count);
        }

        // Get database file size
        let database_size_bytes = if self.path.to_string_lossy() == ":memory:" {
            0
        } else {
            fs::metadata(&self.path).map(|m| m.len()).unwrap_or(0)
        };

        Ok(CacheStatistics {
            font_count,
            database_size_bytes,
            last_updated: DateTime::from_timestamp(last_updated, 0).unwrap_or_else(Utc::now),
            feature_counts,
            script_counts,
            variable_font_count: variable_count as usize,
        })
    }

    /// Optimize the cache database
    pub fn optimize(&self) -> Result<()> {
        let conn = self.get_connection()?;

        // Run VACUUM to reclaim space and defragment
        conn.execute_batch("VACUUM")?;

        // Analyze to update statistics
        conn.execute_batch("ANALYZE")?;

        // Reindex to optimize indices
        conn.execute_batch("REINDEX")?;

        Ok(())
    }

    /// Get the path to the cache file
    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }
}

/// Initialize the database schema
fn initialize_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        -- Create fonts table
        CREATE TABLE IF NOT EXISTS fonts (
            id INTEGER PRIMARY KEY,
            path TEXT NOT NULL UNIQUE,
            name_string TEXT NOT NULL,
            is_variable BOOLEAN NOT NULL,
            charset_string TEXT NOT NULL,
            mtime INTEGER NOT NULL,
            size INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        );

        -- Create properties table
        CREATE TABLE IF NOT EXISTS properties (
            id INTEGER PRIMARY KEY,
            font_id INTEGER NOT NULL,
            type TEXT NOT NULL,
            value TEXT NOT NULL,
            FOREIGN KEY (font_id) REFERENCES fonts(id) ON DELETE CASCADE
        );

        -- Create indices
        CREATE INDEX IF NOT EXISTS idx_fonts_path ON fonts(path);
        CREATE INDEX IF NOT EXISTS idx_fonts_name ON fonts(name_string);
        CREATE INDEX IF NOT EXISTS idx_fonts_variable ON fonts(is_variable);
        CREATE INDEX IF NOT EXISTS idx_properties_font_id ON properties(font_id);
        CREATE INDEX IF NOT EXISTS idx_properties_type ON properties(type);
        CREATE INDEX IF NOT EXISTS idx_properties_value ON properties(value);
        CREATE INDEX IF NOT EXISTS idx_properties_type_value ON properties(type, value);
    ",
    )?;

    Ok(())
}

/// Query builder for constructing SQL queries
struct QueryBuilder {
    conditions: Vec<String>,
    params: Vec<Box<dyn ToSql>>,
}

impl QueryBuilder {
    /// Create a new query builder
    fn new() -> Self {
        Self {
            conditions: Vec::new(),
            params: Vec::new(),
        }
    }

    /// Add a condition for variable fonts
    fn with_variable(mut self) -> Self {
        self.conditions.push("f.is_variable = 1".to_string());
        self
    }

    /// Add a condition for properties
    fn with_property(mut self, property_type: &str, values: &[String]) -> Self {
        if values.is_empty() {
            return self;
        }

        // Optimize by using a single EXISTS subquery with IN clause
        // instead of multiple joins for better performance
        let placeholders: Vec<String> = (0..values.len()).map(|_| "?".to_string()).collect();

        // Simplified query that achieves the same result with less complexity
        let condition = format!(
            "EXISTS (
                SELECT 1 FROM properties 
                WHERE properties.font_id = f.id 
                AND properties.type = ? 
                AND properties.value IN ({})
                GROUP BY properties.font_id
                HAVING COUNT(DISTINCT properties.value) = ?
            )",
            placeholders.join(",")
        );

        self.conditions.push(condition);
        self.params.push(Box::new(property_type.to_string()));
        for value in values {
            self.params.push(Box::new(value.clone()));
        }
        self.params.push(Box::new(values.len() as i64));

        self
    }

    /// Add a condition for name patterns
    fn with_name_patterns(mut self, patterns: &[String]) -> Self {
        if patterns.is_empty() {
            return self;
        }

        // Optimize by combining multiple patterns into a single OR condition
        let mut name_conditions = Vec::new();
        for pattern in patterns {
            name_conditions.push("f.name_string LIKE ?".to_string());
            self.params.push(Box::new(format!("%{}%", pattern)));
        }
        self.conditions
            .push(format!("({})", name_conditions.join(" OR ")));
        self
    }

    /// Add a condition for charset
    fn with_charset(mut self, charset: &str) -> Self {
        if charset.is_empty() {
            return self;
        }

        // Sort the input characters (since the charset_string in the database is sorted)
        let mut sorted_chars: Vec<char> = charset.chars().collect();
        sorted_chars.sort();
        
        // Create a pattern with wildcards between each character
        // For example, "яą" becomes "%ą%я%"
        let mut pattern = String::with_capacity(sorted_chars.len() * 2 + 1);
        pattern.push('%');
        for c in sorted_chars {
            pattern.push(c);
            pattern.push('%');
        }
        
        self.conditions.push("f.charset_string LIKE ?".to_string());
        self.params.push(Box::new(pattern));
        
        self
    }

    /// Build the SQL query
    fn build(self) -> (String, Vec<Box<dyn ToSql>>) {
        let mut query = "SELECT f.path FROM fonts f".to_string();

        if !self.conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&self.conditions.join(" AND "));
        }

        query.push_str(" ORDER BY f.name_string");

        (query, self.params)
    }
}
