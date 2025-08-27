# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-01-27

### Added
- **Plugin Detection Support**: Comprehensive plugin detection across all Gradle formats
  - Support for `plugins` block in both Groovy and Kotlin DSL
  - Support for `apply plugin` statements in both formats
  - Support for Version Catalog plugin references (`alias(libs.plugins.xxx)`)
  - Detection of versionless plugins (core plugins like `java`, `application`)
- **Duplicate Plugin Analysis**: Identifies plugins declared multiple times across different modules
- **Enhanced Version Catalog Support**:
  - Full support for `versions` section in Version Catalogs
  - Complete `version.ref` resolution for both libraries and plugins
  - Support for plugins without versions in Version Catalogs
- **CLI Output Enhancements**:
  - Added duplicate plugin reporting with detailed source information
  - Plugin source type indication (plugins block, apply plugin, version catalog)
  - Enhanced summary reporting including plugin statistics

### Improved
- **Version Catalog Parsing**: Enhanced parsing to handle all plugin definition formats
- **Documentation**: Updated README with comprehensive plugin support examples
- **Test Coverage**: Added comprehensive test suite for all plugin formats and scenarios

### Technical Details
- Added `PluginLocation` and `Plugin` data structures for plugin tracking
- Implemented regex patterns for all supported plugin declaration formats:
  - Groovy DSL: `id 'plugin-id' version 'version'`, `apply plugin: 'plugin-id'`
  - Kotlin DSL: `id("plugin-id") version "version"`, `kotlin("jvm") version "version"`, `apply(plugin = "plugin-id")`
  - Version Catalog references: `alias(libs.plugins.pluginName)`
- Enhanced `CompleteAnalysis` to include `PluginAnalysis`
- Added plugin-specific display functions for formatted output

## [0.1.0] - 2025-01-27

### Added
- **Core Dependency Analysis**:
  - Version conflict detection across modules
  - Duplicate dependency detection
  - Smart dependency bundle recommendations
- **Version Catalog Support**:
  - Basic parsing of `libs.versions.toml` files
  - Support for `libs.xxx` references in build files
- **Multiple Build Script Formats**:
  - Groovy DSL (`build.gradle`) support
  - Kotlin DSL (`build.gradle.kts`) support
  - String format, map format, and Version Catalog format parsing
- **CLI Interface**:
  - Configurable analysis thresholds
  - Colored output for better readability
  - Bundle recommendation with priority scoring
- **Bundle Analysis**:
  - Intelligent grouping of related dependencies
  - Suggested bundle names based on dependency patterns
  - Module usage tracking for bundle recommendations

### Technical Features
- Multi-threaded file scanning for performance
- Regex-based precise dependency parsing
- Configurable thresholds and behaviors
- Comprehensive test suite with integration tests

[0.2.0]: https://github.com/ParkJong-Hun/gradle-dependency-health-checker/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/ParkJong-Hun/gradle-dependency-health-checker/releases/tag/v0.1.0