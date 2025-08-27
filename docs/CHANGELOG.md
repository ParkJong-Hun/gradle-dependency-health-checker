# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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