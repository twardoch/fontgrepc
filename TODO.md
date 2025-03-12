# fontgrepc TODO


## Next Priority: Tests

- [ ] Unit tests for all cache operations:
  - [ ] Test `FontCache::new()` with various path configurations
  - [ ] Test `batch_update_fonts()` with different font inputs
  - [ ] Test `query()` with various criteria combinations
  - [ ] Test `query_by_features()` with edge cases
  - [ ] Test `clean_missing_fonts()` functionality
  - [ ] Test `get_statistics()` accuracy

- [ ] Integration tests for command workflows:
  - [ ] Test the full `add` command workflow with sample fonts
  - [ ] Test the `find` command with various search criteria
  - [ ] Test the `list` command output format
  - [ ] Test the `clean` command behavior
  - [ ] Test JSON output formatting

- [ ] Error handling tests:
  - [ ] Test behavior with corrupted database files
  - [ ] Test with invalid font files
  - [ ] Test with permission issues
  - [ ] Test with concurrent access scenarios
  - [ ] Test with extremely large font collections

## Future Enhancements

- [ ] Remote cache sharing capabilities:
  - [ ] Design a protocol for cache synchronization
  - [ ] Implement cache export/import functionality
  - [ ] Add network transport layer for remote sharing
  - [ ] Implement conflict resolution for merged caches

- [ ] Incremental indexing for large collections:
  - [ ] Track last indexed timestamp
  - [ ] Implement directory watching
  - [ ] Add partial update capabilities
  - [ ] Optimize for minimal re-indexing

- [ ] Cache compression options:
  - [ ] Research SQLite compression extensions
  - [ ] Implement optional charset compression
  - [ ] Add configurable compression levels
  - [ ] Benchmark performance impact

- [ ] Watch mode for automatic cache updates:
  - [ ] Implement filesystem watcher
  - [ ] Add daemon mode for background operation
  - [ ] Create notification system for changes
  - [ ] Optimize for minimal CPU usage

- [ ] Integration with font management tools:
  - [ ] Research popular font management tools
  - [ ] Design plugin architecture
  - [ ] Implement integration with Adobe tools
  - [ ] Create integration with system font managers
