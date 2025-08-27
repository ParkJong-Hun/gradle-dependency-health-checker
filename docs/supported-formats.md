# Supported Gradle File Formats

## Overview

This tool supports all major Gradle build file formats and declaration styles used in modern Gradle projects.

## 1. Groovy Build Scripts (build.gradle)

### Plugin Declarations
```gradle
plugins {
    id 'java'
    id 'org.springframework.boot' version '2.7.0'
    alias(libs.plugins.kotlin.jvm)
}

apply plugin: 'jacoco'
```

### Dependency Declarations
```gradle
dependencies {
    // String format
    implementation 'com.squareup.retrofit2:retrofit:2.9.0'
    
    // Map format
    implementation group: 'com.google.code.gson', name: 'gson', version: '2.10.1'
    
    // Version catalog reference
    testImplementation libs.junit
    
    // Mixed configurations
    api 'androidx.core:core-ktx:1.12.0'
    compileOnly 'org.projectlombok:lombok:1.18.28'
    annotationProcessor 'org.projectlombok:lombok:1.18.28'
}
```

## 2. Kotlin DSL (build.gradle.kts)

### Plugin Declarations
```kotlin
plugins {
    java
    id("org.springframework.boot") version "2.7.0"
    kotlin("jvm") version "1.8.0"
    alias(libs.plugins.kotlin.jvm)
}

apply(plugin = "jacoco")
```

### Dependency Declarations
```kotlin
dependencies {
    // String format
    implementation("com.squareup.retrofit2:retrofit:2.9.0")
    
    // Map format
    implementation(group = "com.google.code.gson", name = "gson", version = "2.10.1")
    
    // Version catalog reference
    testImplementation(libs.junit)
    
    // Mixed configurations
    api("androidx.core:core-ktx:1.12.0")
    compileOnly("org.projectlombok:lombok:1.18.28")
    kapt("org.projectlombok:lombok:1.18.28")
}
```

## 3. Version Catalog (libs.versions.toml)

### Complete Example
```toml
[versions]
retrofit = "2.9.0"
gson = "2.10.1"
kotlin = "1.8.0"
spring-boot = "2.7.0"

[libraries]
# Using version.ref to reference versions
retrofit = { group = "com.squareup.retrofit2", name = "retrofit", version.ref = "retrofit" }
gson = { group = "com.google.code.gson", name = "gson", version.ref = "gson" }
kotlin-stdlib = { group = "org.jetbrains.kotlin", name = "kotlin-stdlib", version.ref = "kotlin" }

# Direct version specification
junit = "junit:junit:4.13.2"
direct-lib = { group = "com.example", name = "library", version = "1.0.0" }

# Without version (for platform BOMs)
spring-bom = { group = "org.springframework.boot", name = "spring-boot-dependencies" }

[bundles]
# Bundle definitions (detected as multiple dependencies)
testing = ["junit", "mockito-core"]
networking = ["retrofit", "okhttp", "gson"]

[plugins]
# Plugin with version reference
kotlin-jvm = { id = "org.jetbrains.kotlin.jvm", version.ref = "kotlin" }

# Plugin with direct version
spring-boot = { id = "org.springframework.boot", version.ref = "spring-boot" }

# Core plugins without version
java-library = { id = "java-library" }
application = { id = "application" }
```

## 4. Detection Capabilities

### Dependency Detection
- **String format**: `implementation 'group:artifact:version'`
- **Map format**: `implementation group: 'group', name: 'artifact', version: 'version'`
- **Version catalog**: `implementation libs.dependency`
- **All configurations**: `implementation`, `api`, `compileOnly`, `testImplementation`, `kapt`, etc.

### Plugin Detection
- **Plugins block**: `id 'plugin-name' version 'version'`
- **Apply plugin**: `apply plugin: 'plugin-name'`
- **Version catalog**: `alias(libs.plugins.pluginName)`
- **Core plugins**: `java`, `kotlin("jvm")`, etc.

### Version Catalog Features
- **Version references**: `version.ref = "version-key"`
- **Direct versions**: `version = "1.0.0"`
- **Bundle detection**: Bundles are treated as individual dependencies
- **Plugin catalog**: Full support for plugin version management

## 5. Project Structure Support

### Multi-Module Projects
```
project/
├── build.gradle                 # Root build script
├── settings.gradle              # Settings file
├── gradle/
│   └── libs.versions.toml      # Version catalog
├── app/
│   └── build.gradle            # App module
├── lib/
│   └── build.gradle.kts        # Library module (Kotlin DSL)
└── feature/
    └── build.gradle            # Feature module
```

### Subproject Detection
- Automatically scans all subdirectories for build files
- Supports both Groovy and Kotlin DSL in the same project
- Handles nested module structures
- Respects `.gitignore` patterns for build directories

## 6. Advanced Parsing Features

### Configuration Handling
- Detects all standard Gradle configurations
- Handles custom configurations
- Tracks configuration inheritance (e.g., `api` extends `implementation`)

### Source Attribution
- Tracks exact file location and line numbers
- Identifies declaration source (direct vs. version catalog)
- Maintains full context for dependency resolution

### Error Resilience
- Continues analysis even with parsing errors in individual files
- Reports parsing issues without stopping the entire analysis
- Handles malformed or incomplete build files gracefully