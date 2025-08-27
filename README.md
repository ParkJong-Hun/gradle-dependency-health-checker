# Gradle Dependency Health Checker

A powerful tool to detect duplicate dependencies, version conflicts, and recommend dependency bundles in Gradle projects.

## 🚀 Features

- **Version Conflict Detection**: Identifies when the same library is used with different versions across modules
- **Duplicate Dependency Detection**: Finds dependencies that are declared multiple times across different modules
- **Duplicate Plugin Detection**: Identifies plugins that are declared multiple times across different modules
- **Bundle Recommendations**: Suggests creating shared modules for commonly used dependency groups
- **Version Catalog Support**: Full support for Gradle Version Catalogs with version references (`libs.versions.toml`)
- **Plugin Support**: Detects plugins in `plugins` blocks, `apply plugin`, and Version Catalog references
- **Multiple Declaration Formats**: Supports string format, map format, and libs.xxx format declarations

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
```

### Advanced Options
```bash
gradle-dependency-health-checker \
  --path ./my-project \
  --min-version-conflicts 3 \
  --min-duplicate-dependencies 2 \
  --min-bundle-size 2 \
  --min-bundle-modules 3 \
  --max-bundle-recommendations 10
```

### Option Reference

| Option | Default | Description |
|--------|---------|-------------|
| `--path` | `.` | Path to the Gradle project to analyze |
| `--min-version-conflicts` | `2` | Minimum number of version conflicts to display |
| `--min-duplicate-dependencies` | `2` | Minimum number of duplicate dependencies to display |
| `--min-bundle-size` | `2` | Minimum number of dependencies for bundle recommendation |
| `--min-bundle-modules` | `2` | Minimum number of modules for bundle recommendation |
| `--max-bundle-recommendations` | `5` | Maximum number of bundle recommendations to display |

## 📋 Sample Output

### Version Conflicts
```
🚨 Found 2 version conflicts:

🚨 Dependency: com.squareup.okhttp3:okhttp
  ⚠️ app/build.gradle:12 - implementation configuration (version: 4.12.0)
  ⚠️ feature/build.gradle:8 - implementation configuration (version: 4.10.0)
```

### Duplicate Dependencies
```
⚠️  Found 1 duplicate dependencies:

📦 Dependency: com.squareup.retrofit2:retrofit
  📍 app/build.gradle:15 - implementation configuration (version: 2.9.0)
  📍 feature1/build.gradle:10 - implementation configuration (version: 2.9.0)
  📍 feature2/build.gradle:8 - implementation configuration (version: 2.9.0)
```

### Duplicate Plugins
```
🔌 Found 1 duplicate plugins:

🔌 Plugin: java
  🔍 app/build.gradle:3 - plugin [plugins block]
  🔍 feature1/build.gradle:2 - plugin [plugins block]
  🔍 feature2/build.gradle:4 - plugin [apply plugin]
```

### Bundle Recommendations
```
💡 Bundle recommendations (showing 3 of 5):

📎 1. Recommended Bundle (4 dependencies × 3 modules)
   Dependencies:
     ├─ com.squareup.retrofit2:retrofit
     ├─ com.squareup.retrofit2:converter-gson
     ├─ com.squareup.okhttp3:okhttp
     └─ com.squareup.okhttp3:logging-interceptor
   Configurations: implementation
   Used by modules:
     ├─ app/build.gradle
     ├─ feature1/build.gradle
     └─ feature2/build.gradle
   💭 Consider creating a shared module: networking-bundle
```

## 🔧 Supported Gradle File Formats

### 1. Groovy Build Scripts (build.gradle)
```gradle
plugins {
    id 'java'
    id 'org.springframework.boot' version '2.7.0'
    alias(libs.plugins.kotlin.jvm)
}

apply plugin: 'jacoco'

dependencies {
    implementation 'com.squareup.retrofit2:retrofit:2.9.0'
    implementation group: 'com.google.code.gson', name: 'gson', version: '2.10.1'
    testImplementation libs.junit
}
```

### 2. Kotlin DSL (build.gradle.kts)
```kotlin
plugins {
    java
    id("org.springframework.boot") version "2.7.0"
    kotlin("jvm") version "1.8.0"
    alias(libs.plugins.kotlin.jvm)
}

apply(plugin = "jacoco")

dependencies {
    implementation("com.squareup.retrofit2:retrofit:2.9.0")
    implementation(group = "com.google.code.gson", name = "gson", version = "2.10.1")
    testImplementation(libs.junit)
}
```

### 3. Version Catalog (libs.versions.toml)
```toml
[versions]
retrofit = "2.9.0"
gson = "2.10.1"
kotlin = "1.8.0"

[libraries]
# Using version.ref to reference versions
retrofit = { group = "com.squareup.retrofit2", name = "retrofit", version.ref = "retrofit" }
gson = { group = "com.google.code.gson", name = "gson", version.ref = "gson" }
kotlin-stdlib = { group = "org.jetbrains.kotlin", name = "kotlin-stdlib", version.ref = "kotlin" }

# Direct version specification
junit = "junit:junit:4.13.2"
direct-lib = { group = "com.example", name = "library", version = "1.0.0" }

[plugins]
# Plugin with version reference
kotlin-jvm = { id = "org.jetbrains.kotlin.jvm", version.ref = "kotlin" }
# Plugin with direct version
spring-boot = { id = "org.springframework.boot", version = "2.7.0" }
# Core plugins without version
java-library = { id = "java-library" }
application = { id = "application" }
```

## ⚡ Performance & Features

- **Fast Analysis**: Multi-threaded file scanning for quick processing of large projects
- **Accurate Parsing**: Regex-based precise dependency parsing
- **Smart Bundling**: Intelligent bundle recommendations based on priority scoring
- **Version Reference Resolution**: Automatically resolves `version.ref` references in Version Catalogs
- **Plugin Support**: Supports both library and plugin version references
- **Multi-format Plugin Detection**: Detects plugins in `plugins` blocks, `apply plugin` statements, and Version Catalog references
- **Configurable**: All thresholds and behaviors can be customized

## 🤝 Contributing

Issues and pull requests are welcome!
