# Gradle Dependency Health Checker

A powerful tool to detect duplicate dependencies, version conflicts, and recommend dependency bundles in Gradle projects.

## 🚀 Features

- **Version Conflict Detection**: Identifies when the same library is used with different versions across modules
- **Duplicate Dependency Detection**: Finds dependencies that are declared multiple times across different modules  
- **Duplicate Plugin Detection**: Identifies plugins that are declared multiple times across different modules
- **Bundle Recommendations**: Suggests creating shared modules for commonly used dependency groups
- **Flexible Subcommands**: Run specific analyses with targeted commands
- **JSON Output**: Export analysis results to structured JSON files for integration with other tools
- **Silent Mode**: Suppress all console output for CI/CD pipelines and automated workflows
- **Full Gradle Support**: Version Catalogs, all file formats, and modern Gradle features

## 📦 Installation

### Build from Source
```bash
git clone https://github.com/ParkJong-Hun/gradle-dependency-health-checker.git
cd gradle-dependency-health-checker
cargo build --release
```

### Run the Binary
```bash
# After release build
./target/release/gradle-dependency-health-checker --path <project-path>
```

## 🎯 Usage

### Basic Usage
```bash
# Analyze current directory
gradle-dependency-health-checker

# Analyze specific path
gradle-dependency-health-checker --path /path/to/gradle/project

# Run specific checks only
gradle-dependency-health-checker conflicts
gradle-dependency-health-checker dependencies
gradle-dependency-health-checker plugins
gradle-dependency-health-checker bundles
```

### Output Options
```bash
# Output to JSON file
gradle-dependency-health-checker --output analysis.json

# Silent mode (no console output)
gradle-dependency-health-checker --output analysis.json --silent
```

### Available Commands
- **`conflicts`** - Check version conflicts only
- **`dependencies`** - Check duplicate dependencies only  
- **`plugins`** - Check duplicate plugins only
- **`duplicates`** - Check both dependency and plugin duplicates
- **`bundles`** - Generate bundle recommendations only
- **`all`** - Run all checks explicitly (default behavior)

📖 **For detailed usage examples and advanced configuration, see [docs/advanced-usage.md](docs/advanced-usage.md)**

## 📋 Sample Output

### Console Output
```
🚨 Found 2 version conflicts:

🚨 Dependency: com.squareup.okhttp3:okhttp
  ⚠️ app/build.gradle:12 - implementation configuration (version: 4.12.0)
  ⚠️ feature/build.gradle:8 - implementation configuration (version: 4.10.0)

⚠️ Found 3 duplicate dependencies:

📦 Dependency: com.squareup.retrofit2:retrofit
  📍 app/build.gradle:15 - implementation configuration (version: 2.9.0)
  📍 feature1/build.gradle:10 - implementation configuration (version: 2.9.0)
  📍 feature2/build.gradle:8 - implementation configuration (version: 2.9.0)

💡 Bundle recommendations (showing 3 of 5):
📎 1. Recommended Bundle (4 dependencies × 3 modules)
   💭 Consider creating a shared module: networking-bundle
```

📄 **For JSON output format examples, see [docs/json-output.md](docs/json-output.md)**

## 🔧 Supported Formats

- **Groovy DSL**: `build.gradle` files with full syntax support
- **Kotlin DSL**: `build.gradle.kts` files with type-safe declarations  
- **Version Catalogs**: `libs.versions.toml` with version references
- **All Gradle Features**: Plugins, dependencies, configurations, and modern Gradle patterns

🔍 **For detailed format specifications, see [docs/supported-formats.md](docs/supported-formats.md)**

## 🤝 Contributing

Issues and pull requests are welcome!
