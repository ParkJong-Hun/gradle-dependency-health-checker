# Gradle Dependency Health Checker

A comprehensive tool to analyze Gradle projects for dependency issues, version conflicts, and optimization opportunities. Supports modern Kotlin Multiplatform projects, version catalogs, and provides both console and JSON output.

## 🚀 Features

### Core Analysis
- **Version Conflict Detection**: Identifies when the same library is used with different versions across modules
- **Duplicate Dependency Detection**: Finds dependencies that are declared multiple times across different modules  
- **Duplicate Plugin Detection**: Identifies plugins that are declared multiple times across different modules
- **Bundle Recommendations**: Suggests creating shared modules for commonly used dependency groups with priority scoring

### Modern Gradle Support
- **Kotlin Multiplatform**: Full support for `sourceSets { commonMain, androidMain, iosMain, etc. }`
- **Version Catalogs**: Complete `libs.versions.toml` integration with dot-to-dash conversion
- **Mixed Dependency Styles**: Handles both `libs.xxx` references and direct string declarations
- **Project Dependencies Filtering**: Automatically excludes `project(':module')` and `projects.xxx` references
- **Compose Integration**: Built-in support for `compose.runtime`, `compose.ui` and other compose accessors

### Output & Integration
- **Flexible Subcommands**: Run specific analyses with targeted commands
- **JSON Output**: Export detailed analysis results to structured JSON files
- **Filtered JSON Output**: JSON output includes only relevant sections based on the subcommand used
- **Silent Mode**: Perfect for CI/CD pipelines and automated workflows
- **Rich Console Output**: Color-coded emoji-rich output for developers

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
# Output to JSON file (includes all analysis sections)
gradle-dependency-health-checker --output analysis.json

# Output specific analysis to JSON file (filtered content)
gradle-dependency-health-checker conflicts --output conflicts.json
gradle-dependency-health-checker dependencies --output deps.json
gradle-dependency-health-checker bundles --output bundles.json

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

⚠️ Found 4 duplicate dependencies:

📦 Dependency: org.jetbrains.compose:runtime
  📍 core/model/build.gradle.kts:13 - implementation-commonMain configuration [via libs.compose.runtime]
  📍 core/common/build.gradle.kts:13 - commonMainImplementation configuration [via libs.compose.runtime]
  📍 app-android/build.gradle.kts:72 - implementation configuration [via libs.compose.runtime]

📦 Dependency: io.ktor:ktor-client-core
  📍 core/network/build.gradle.kts:15 - api-commonMain configuration (version: 2.3.5) [via libs.ktor.client.core]
  📍 feature/session/build.gradle.kts:12 - implementation configuration (version: 2.3.5) [via libs.ktor.client.core]

💡 Bundle recommendations (showing 2 of 3):

📎 1. Recommended Bundle (5 dependencies × 4 modules)
   Dependencies:
     ├─ org.jetbrains.compose:runtime
     ├─ org.jetbrains.compose:ui
     ├─ androidx.compose.material3:material3
     └─ androidx.compose.ui:ui-tooling
   Configurations: api-commonMain, implementation, implementation-commonMain
   Used by modules:
     ├─ core/designsystem/build.gradle.kts
     ├─ feature/session/build.gradle.kts
     ├─ feature/timetable/build.gradle.kts
     └─ app-android/build.gradle.kts
   💭 Consider creating a shared module: compose-bundle

📎 2. Recommended Bundle (3 dependencies × 3 modules)
   Dependencies:
     ├─ io.ktor:ktor-client-core
     ├─ io.ktor:ktor-client-json
     └─ org.jetbrains.kotlinx:kotlinx-serialization-json
   Configurations: api, implementation
   Used by modules:
     ├─ core/network/build.gradle.kts
     ├─ feature/session/build.gradle.kts
     └─ feature/sponsors/build.gradle.kts
   💭 Consider creating a shared module: networking-bundle
```

📄 **For JSON output format examples, see [docs/json-output.md](docs/json-output.md)**

## 🔧 Supported Formats

### Build Files
- **Groovy DSL**: `build.gradle` files with complete syntax support
- **Kotlin DSL**: `build.gradle.kts` files with type-safe declarations
- **Kotlin Multiplatform**: Full support for `kotlin { sourceSets { ... } }` blocks

### Dependency Declaration Styles
- **Direct String**: `implementation("group:artifact:version")`
- **Map Syntax**: `implementation(group: "group", name: "artifact", version: "version")`
- **Version Catalogs**: `implementation(libs.library.reference)`
- **Compose Accessors**: `implementation(compose.runtime)`, `implementation(compose.ui)`
- **Mixed Styles**: All above patterns can be used together in the same project

### SourceSet Configurations
- **Standard Configurations**: `implementation`, `api`, `compileOnly`, etc.
- **SourceSet-Specific**: `commonMainImplementation`, `androidMainApi`, etc.
- **Nested SourceSets**: `commonMain.dependencies { }`, `androidMain { dependencies { } }`

### Advanced Features
- **Project Dependencies Exclusion**: `project(':module')` and `projects.xxx` automatically ignored
- **Version Catalog Integration**: Complete `libs.versions.toml` parsing with dot-to-dash conversion
- **BOM Management**: Handles version-less dependencies managed by BOMs

🔍 **For detailed format specifications, see [docs/supported-formats.md](docs/supported-formats.md)**

## 🤝 Contributing

Issues and pull requests are welcome!
