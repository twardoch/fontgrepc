# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Git-tag-based semantic versioning system
- Comprehensive test suite with 25+ unit tests for cache module
- Integration tests for CLI functionality
- Performance benchmarks using criterion
- Automated build and release scripts
- GitHub Actions workflows for CI/CD, releases, and nightly builds
- Multi-platform binary releases (Linux, macOS, Windows)
- Cross-compilation support for multiple architectures
- Automated dependency vulnerability scanning
- Code coverage reporting
- Development scripts for building, testing, and releasing

### Changed
- Improved cache module with better error handling and transaction management
- Enhanced build system with version extraction from git tags
- Better test coverage and more robust testing infrastructure
- Streamlined development workflow with automated scripts

### Fixed
- Various bug fixes and improvements in cache operations
- Better handling of concurrent database access
- Improved error messages and user feedback

## [1.0.5] - 2024-XX-XX

### Added
- Updated version to 1.0.5
- Enhanced charset search optimization

### Changed
- Updated dependencies
- Simplified connection management in fontgrepc

### Fixed
- Various bug fixes and improvements

## [1.0.2] - 2024-XX-XX

### Added
- Previous release with core functionality

## [1.0.1] - 2024-XX-XX

### Added
- Initial release with basic font caching and search capabilities
- SQLite-based cache for fast font searches
- Support for various font formats (TTF, OTF)
- Command-line interface for font management
- JSON output format support
- Parallel processing for font indexing

### Features
- Font cache management (add, list, clean)
- Font search by features, scripts, tables, and other criteria
- Variable font support
- Unicode range searching
- Regular expression pattern matching for font names
- Verbose output mode
- Custom cache path configuration