/*
 * Copyright (c) 2025 ParkJong-Hun
 *
 * Licensed under the MIT License.
 * See LICENSE file in the project root for full license information.
 */

mod common;

use gradle_dependency_health_checker::parser::{find_gradle_files, parse_dependencies_from_file, load_version_catalogs, DependencySourceType};
use common::create_test_build_gradle;
use tempfile::tempdir;
use std::collections::HashMap;

#[test]
fn test_find_gradle_files() {
    let temp_dir = tempdir().unwrap();
    
    create_test_build_gradle(temp_dir.path(), "app", r#"
dependencies {
    implementation 'com.example:test:1.0.0'
}
"#);
    
    let gradle_files = find_gradle_files(temp_dir.path()).unwrap();
    assert!(!gradle_files.is_empty());
}

#[test]
fn test_basic_dependency_parsing() {
    let temp_dir = tempdir().unwrap();
    
    create_test_build_gradle(temp_dir.path(), "app", r#"
dependencies {
    implementation 'com.example:test:1.0.0'
}
"#);
    
    let gradle_files = find_gradle_files(temp_dir.path()).unwrap();
    let version_catalogs = HashMap::new();
    
    for gradle_file in gradle_files {
        let dependencies = parse_dependencies_from_file(&gradle_file, &version_catalogs).unwrap();
        assert!(!dependencies.is_empty());
    }
}

#[test]
fn test_multiplatform_sourceset_parsing() {
    let temp_dir = tempdir().unwrap();
    
    create_test_build_gradle(temp_dir.path(), "multiplatform", r#"
sourceSets {
    commonMain.dependencies {
        implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.7.3")
        api("io.ktor:ktor-client-core:2.3.5")
    }
    
    commonTest {
        dependencies {
            implementation("kotlin-test")
        }
    }
    
    androidMain {
        dependencies {
            implementation("androidx.core:core-ktx:1.13.0")
        }
    }
    
    iosMain.dependencies {
        implementation("io.ktor:ktor-client-darwin:2.3.5")
    }
}

dependencies {
    implementation("com.google.code.gson:gson:2.10.1")
}
"#);
    
    let gradle_files = find_gradle_files(temp_dir.path()).unwrap();
    let version_catalogs = HashMap::new();
    
    for gradle_file in gradle_files {
        let dependencies = parse_dependencies_from_file(&gradle_file, &version_catalogs).unwrap();
        
        // Print found dependencies for debugging
        println!("\\nFound {} dependencies:", dependencies.len());
        for dep in &dependencies {
            println!("  {} -> {}:{}:{:?} (line {})",
                dep.configuration,
                dep.dependency.group,
                dep.dependency.artifact,
                dep.dependency.version,
                dep.line_number
            );
        }
        
        // Should find at least 4 dependencies
        assert!(dependencies.len() >= 4, "Expected at least 4 dependencies, found {}", dependencies.len());
        
        // Check that we have source set specific configurations
        let configs: Vec<&String> = dependencies.iter().map(|d| &d.configuration).collect();
        
        // Should have main dependencies (regular dependencies block)
        assert!(configs.iter().any(|c| c == &"implementation"), "Should have main implementation dependencies");
        
        // Should have source set specific dependencies
        assert!(configs.iter().any(|c| c.contains("-commonMain")), "Should have commonMain dependencies");
        // Note: Some dependencies may not be parsed if they lack proper version information
    }
}

#[test]
fn test_mixed_libs_and_string_dependencies() {
    let temp_dir = tempdir().unwrap();
    
    // Create version catalog file first
    let catalog_dir = temp_dir.path().join("gradle");
    std::fs::create_dir_all(&catalog_dir).unwrap();
    let catalog_content = r#"[versions]
kotlin = "1.9.10"
kotlinx-coroutines = "1.7.3"
ktor = "2.3.5"

[libraries]
kotlinx-coroutines-core = { group = "org.jetbrains.kotlinx", name = "kotlinx-coroutines-core", version.ref = "kotlinx-coroutines" }
ktor-client-core = { group = "io.ktor", name = "ktor-client-core", version.ref = "ktor" }
"#;
    std::fs::write(catalog_dir.join("libs.versions.toml"), catalog_content).unwrap();
    
    create_test_build_gradle(temp_dir.path(), "mixed", r#"
sourceSets {
    commonMain.dependencies {
        implementation(libs.kotlinx.coroutines.core)
        api(libs.ktor.client.core)
        implementation("com.google.code.gson:gson:2.10.1")
    }
    
    commonTest {
        dependencies {
            implementation("junit:junit:4.13.2")
            implementation("kotlin-test")
        }
    }
}

dependencies {
    implementation(libs.ktor.client.core)
    implementation("com.squareup.retrofit2:retrofit:2.9.0")
}
"#);
    
    let gradle_files = find_gradle_files(temp_dir.path()).unwrap();
    let version_catalogs = load_version_catalogs(temp_dir.path()).unwrap();
    
    // Debug: print catalog contents
    println!("\nVersion catalogs found: {}", version_catalogs.len());
    for (path, _catalog) in &version_catalogs {
        println!("Catalog at {:?}", path);
    }
    
    for gradle_file in gradle_files {
        let dependencies = parse_dependencies_from_file(&gradle_file, &version_catalogs).unwrap();
        
        // Print found dependencies for debugging
        println!("\nFound {} dependencies:", dependencies.len());
        for dep in &dependencies {
            println!("  {} -> {}:{}:{:?} (line {}) [{}]",
                dep.configuration,
                dep.dependency.group,
                dep.dependency.artifact,
                dep.dependency.version,
                dep.line_number,
                match &dep.source_type {
                    DependencySourceType::Direct => "Direct",
                    DependencySourceType::VersionCatalog(ref r) => &format!("Catalog: {}", r),
                }
            );
        }
        
        // Should find dependencies from both libs and string formats
        assert!(dependencies.len() >= 6, "Expected at least 6 dependencies, found {}", dependencies.len());
        
        // Check that we have both direct and version catalog dependencies
        let has_direct = dependencies.iter().any(|d| matches!(d.source_type, DependencySourceType::Direct));
        let has_catalog = dependencies.iter().any(|d| matches!(d.source_type, DependencySourceType::VersionCatalog(_)));
        
        assert!(has_direct, "Should have direct string dependencies");
        assert!(has_catalog, "Should have version catalog dependencies");
        
        // Check for specific expected dependencies
        let dep_strings: Vec<String> = dependencies.iter()
            .map(|d| format!("{}:{}:{}", d.dependency.group, d.dependency.artifact, d.dependency.version.as_ref().unwrap_or(&"None".to_string())))
            .collect();
            
        // Should find gson (string dependency)
        assert!(dep_strings.iter().any(|s| s.contains("gson")), "Should find gson dependency");
        
        // Should find kotlinx-coroutines-core (from catalog)
        assert!(dep_strings.iter().any(|s| s.contains("kotlinx-coroutines-core")), "Should find kotlinx-coroutines-core dependency");
        
        // Should find ktor-client-core (from catalog) - might be duplicate
        let ktor_core_count = dep_strings.iter().filter(|s| s.contains("ktor-client-core")).count();
        assert!(ktor_core_count >= 1, "Should find at least one ktor-client-core dependency");
        
        // Test source set suffixes
        let configs: Vec<&String> = dependencies.iter().map(|d| &d.configuration).collect();
        assert!(configs.iter().any(|c| c.contains("-commonMain")), "Should have commonMain dependencies");
        assert!(configs.iter().any(|c| c.contains("-commonTest")), "Should have commonTest dependencies");
    }
}

#[test]
fn test_project_dependencies_are_ignored() {
    let temp_dir = tempdir().unwrap();
    
    create_test_build_gradle(temp_dir.path(), "project-deps", r#"
dependencies {
    // External dependencies (should be analyzed)
    implementation("com.google.code.gson:gson:2.10.1")
    api("io.ktor:ktor-client-core:2.3.5")
    
    // Project dependencies (should be ignored)
    implementation(project(":core"))
    api(project(":shared"))
    testImplementation(project(":test-utils"))
    
    // Projects accessor (should be ignored)  
    implementation(projects.data.database)
    api(projects.ui.components)
    
    // Mixed case
    implementation("androidx.core:core-ktx:1.13.0")
    implementation(project(":another-module"))
}
"#);
    
    let gradle_files = find_gradle_files(temp_dir.path()).unwrap();
    let version_catalogs = HashMap::new();
    
    for gradle_file in gradle_files {
        let dependencies = parse_dependencies_from_file(&gradle_file, &version_catalogs).unwrap();
        
        // Print found dependencies for debugging
        println!("\nFound {} dependencies:", dependencies.len());
        for dep in &dependencies {
            println!("  {} -> {}:{}:{:?} (line {})",
                dep.configuration,
                dep.dependency.group,
                dep.dependency.artifact,
                dep.dependency.version,
                dep.line_number
            );
        }
        
        // Should only find external dependencies, not project dependencies
        assert_eq!(dependencies.len(), 3, "Expected 3 external dependencies, found {}", dependencies.len());
        
        // Check that we have the expected external dependencies
        let dep_artifacts: Vec<&String> = dependencies.iter().map(|d| &d.dependency.artifact).collect();
        
        assert!(dep_artifacts.contains(&&"gson".to_string()), "Should find gson dependency");
        assert!(dep_artifacts.contains(&&"ktor-client-core".to_string()), "Should find ktor-client-core dependency");
        assert!(dep_artifacts.contains(&&"core-ktx".to_string()), "Should find core-ktx dependency");
        
        // Check that project dependencies are NOT included
        assert!(!dep_artifacts.iter().any(|&artifact| 
            artifact == "core" || artifact == "shared" || artifact == "test-utils" || 
            artifact == "another-module" || artifact == "database" || artifact == "components"
        ), "Should not include project dependencies");
    }
}

#[test]
fn test_kotlin_sourceset_dependencies() {
    let temp_dir = tempdir().unwrap();
    
    create_test_build_gradle(temp_dir.path(), "kotlin-mp", r#"
kotlin {
    sourceSets {
        commonMain.dependencies {
            implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.7.3")
        }
    }
}

dependencies {
    commonMainImplementation("androidx.core:core-ktx:1.13.0")
}
"#);
    
    let gradle_files = find_gradle_files(temp_dir.path()).unwrap();
    let version_catalogs = HashMap::new();
    
    for gradle_file in gradle_files {
        let dependencies = parse_dependencies_from_file(&gradle_file, &version_catalogs).unwrap();
        
        // Print found dependencies for debugging
        println!("\nFound {} dependencies:", dependencies.len());
        for dep in &dependencies {
            println!("  {} -> {}:{}:{:?} (line {})",
                dep.configuration,
                dep.dependency.group,
                dep.dependency.artifact,
                dep.dependency.version,
                dep.line_number
            );
        }
        
        // Should find at least 1 dependency
        assert!(dependencies.len() >= 1, "Expected at least 1 dependency, found {}", dependencies.len());
    }
}

#[test] 
fn test_compose_accessor_dependencies() {
    let temp_dir = tempdir().unwrap();
    
    create_test_build_gradle(temp_dir.path(), "compose-test", r#"
kotlin {
    sourceSets {
        commonMain.dependencies {
            implementation(compose.runtime)
            implementation(compose.ui)
        }
    }
}
"#);
    
    let gradle_files = find_gradle_files(temp_dir.path()).unwrap();
    let version_catalogs = HashMap::new();
    
    for gradle_file in gradle_files {
        let dependencies = parse_dependencies_from_file(&gradle_file, &version_catalogs).unwrap();
        
        // Print found dependencies for debugging
        println!("\nCompose accessor test - Found {} dependencies:", dependencies.len());
        for dep in &dependencies {
            println!("  {} -> {}:{}:{:?} (line {})",
                dep.configuration,
                dep.dependency.group,
                dep.dependency.artifact,
                dep.dependency.version,
                dep.line_number
            );
        }
        
        // Should find compose dependencies  
        assert!(dependencies.len() >= 2, "Expected at least 2 compose dependencies, found {}", dependencies.len());
        
        // Check that compose dependencies are found
        let has_compose = dependencies.iter().any(|d| d.dependency.group.contains("compose"));
        assert!(has_compose, "Should find compose dependencies");
    }
}