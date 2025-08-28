# Documentation

## 📚 Available Documentation

### 🚀 [Advanced Usage](advanced-usage.md)
Detailed command-line options, subcommands, and configuration examples for power users.

**Topics covered:**
- Kotlin Multiplatform projects and sourceSets
- Custom thresholds and filtering
- All available subcommands  
- CI/CD integration (GitHub Actions, Jenkins)
- Performance optimization tips for large projects

### 📄 [JSON Output](json-output.md)  
Complete guide to JSON file output format and integration with other tools.

**Topics covered:**
- JSON export usage examples with real project data
- Silent mode for automation workflows
- Complete JSON schema with sourceSet configurations
- Integration examples (Python, shell scripts, jq)
- Version catalog and compose accessor tracking

### 🔧 [Supported Formats](supported-formats.md)
Comprehensive guide to all supported Gradle file formats and modern syntax patterns.

**Topics covered:**
- Groovy DSL (`build.gradle`) and Kotlin DSL (`build.gradle.kts`)
- Kotlin Multiplatform sourceSets (`commonMain`, `androidMain`, etc.)
- Version Catalogs (`libs.versions.toml`) with dot-to-dash conversion
- Compose Multiplatform support (`compose.runtime`, `compose.ui`)
- Project dependencies filtering (`project(':module')` exclusion)
- Mixed dependency styles and advanced parsing features

## 🔗 Quick Links

- [Main README](../README.md) - Project overview and basic usage
- [Advanced Usage](advanced-usage.md) - Detailed CLI reference
- [JSON Output](json-output.md) - Automation and integration
- [Supported Formats](supported-formats.md) - File format specifications

## 📝 Release Notes

- [CHANGELOG.md](../CHANGELOG.md) - Version history and release notes

## 💡 Getting Help

- Check the [GitHub Issues](https://github.com/ParkJong-Hun/gradle-dependency-health-checker/issues) for known issues
- Use `gradle-dependency-health-checker --help` for command-line help
- Use `gradle-dependency-health-checker <subcommand> --help` for subcommand-specific help