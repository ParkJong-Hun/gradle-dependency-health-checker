# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.1] - 2025-08-28

### Fixed
- Bug fixes and improvements

## [0.4.0] - 2025-08-28

### Added
- **Loading Animations**: Added visual progress indicators for better user experience
  - Spinner animation for file writing operations using Unicode characters (⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏)
  - Progress animation for analysis operations with emoji and dots (🔍 Analyzing...)
  - Non-blocking animations using separate threads and atomic operations
  - Automatic disable in silent mode (`--silent` flag)
- **Kotlin Multiplatform Support**: Enhanced parsing for complex Kotlin DSL configurations
  - Support for nested `kotlin { sourceSets { ... } }` block structures
  - Enhanced state machine parser with `InKotlin` state for proper nesting
  - Support for all sourceSet configurations (commonMain, androidMain, iosMain, etc.)
- **Version Catalog Integration**: Comprehensive support for Gradle Version Catalogs
  - Dot-to-dash conversion for library references (`libs.kotlinx.coroutines.core` → `kotlinx-coroutines-core`)
  - Support for Compose BOM accessors (`compose.runtime`, `compose.ui`)
  - Enhanced TOML parsing with fallback resolution strategies
- **Project Dependency Filtering**: Added intelligent filtering to focus on external libraries
  - Exclude `project(':module')` style dependencies from analysis
  - Exclude `projects.xxx` accessor-based project dependencies
  - Configurable patterns for different project reference styles

### Enhanced
- **Parser Robustness**: Significantly improved parsing accuracy for real-world projects
  - Better handling of complex nested block structures
  - Enhanced regex patterns for various dependency declaration styles
  - Improved state machine transitions with proper ownership handling
- **Terminal Integration**: Better terminal control with cursor management
  - Proper cleanup of loading animations on completion
  - Smooth animation transitions without screen flicker
- **Error Handling**: Enhanced error reporting for parsing edge cases
- **Performance**: Optimized parsing for large multi-module projects

### Technical Improvements
- **New Module**: Created `src/loading.rs` with `LoadingSpinner` and `ProgressBar` structs
- **State Machine**: Enhanced parser state machine with additional states for nested blocks
- **Configuration**: Extended regex patterns in `src/config.rs` for better matching
- **Version Resolution**: Improved version catalog resolution logic in `src/version_catalog.rs`

## [0.3.0] - 2025-08-27

### Added
- **Subcommand Structure**: Introduced targeted analysis subcommands for specific checks
  - `conflicts` - Check version conflicts only
  - `dependencies` - Check duplicate dependencies only  
  - `plugins` - Check duplicate plugins only
  - `duplicates` - Check both dependency and plugin duplicates
  - `bundles` - Generate bundle recommendations only
  - `all` - Run all checks explicitly (maintains backward compatibility)
- **JSON Export**: Added `--output <file>` option to export analysis results to JSON files
- **Silent Mode**: Added `--silent` flag to suppress console output for CI/CD automation
- **Plugin Duplicate Thresholds**: Added `--min-duplicate-plugins` option for configurable plugin duplicate detection (default: 2)
- **Comprehensive Documentation**: Created modular documentation structure in `/docs` directory
  - Advanced usage guide with all CLI options and subcommands
  - JSON output format specification with integration examples
  - Supported Gradle file formats documentation

### Changed
- **Plugin Counting Logic**: Plugin duplicates now count individual occurrences instead of duplicate groups for consistency with dependency counting
- **Documentation Structure**: Moved detailed documentation from main README to separate files in `/docs` for better organization
- **CLI Interface**: Enhanced command structure with subcommands while maintaining backward compatibility
- **Output Format**: All analysis data structures now support JSON serialization

### Enhanced
- **Error Handling**: Improved error messages and validation for new CLI options
- **Test Coverage**: Extended test suite to cover all new features (36 total tests)
- **Performance**: Subcommands allow running only necessary analyses for faster execution
- **Automation Support**: JSON output and silent mode enable seamless CI/CD integration

## [0.2.0] - Previous Release
- Initial release with basic dependency and plugin analysis
- Version conflict detection
- Bundle recommendation system
- Multi-module Gradle project support

## [0.1.0] - Initial Release
- Basic Gradle dependency analysis functionality