/*
 * Copyright (c) 2025 ParkJong-Hun
 *
 * Licensed under the MIT License.
 * See LICENSE file in the project root for full license information.
 */

mod common;

use gradle_dependency_health_checker::version_catalog::{parse_version_catalog, find_version_catalog_files};
use common::create_test_version_catalog;
use tempfile::tempdir;

#[test]
fn test_find_version_catalog_files() {
    let temp_dir = tempdir().unwrap();
    
    create_test_version_catalog(temp_dir.path(), r#"
[versions]
test = "1.0.0"

[libraries]
test-lib = { group = "com.example", name = "test", version.ref = "test" }
"#);
    
    let catalog_files = find_version_catalog_files(temp_dir.path()).unwrap();
    assert!(!catalog_files.is_empty());
}

#[test]
fn test_basic_catalog_parsing() {
    let temp_dir = tempdir().unwrap();
    
    create_test_version_catalog(temp_dir.path(), r#"
[versions]
test = "1.0.0"

[libraries]
test-lib = { group = "com.example", name = "test", version.ref = "test" }
"#);
    
    let catalog_files = find_version_catalog_files(temp_dir.path()).unwrap();
    
    for catalog_file in catalog_files {
        let catalog = parse_version_catalog(&catalog_file).unwrap();
        assert!(catalog.versions.is_some());
        assert!(catalog.libraries.is_some());
    }
}

#[test]
fn test_version_ref_resolution_for_library() {
    let temp_dir = tempdir().unwrap();
    
    create_test_version_catalog(temp_dir.path(), r#"
[versions]
kotlin = "1.8.0"
junit = "5.9.0"

[libraries]
kotlin-stdlib = { group = "org.jetbrains.kotlin", name = "kotlin-stdlib", version.ref = "kotlin" }
junit-api = { group = "org.junit.jupiter", name = "junit-jupiter-api", version.ref = "junit" }
direct-version = { group = "com.example", name = "direct", version = "2.0.0" }
"#);
    
    let catalog_files = find_version_catalog_files(temp_dir.path()).unwrap();
    let catalog = parse_version_catalog(&catalog_files[0]).unwrap();
    
    // Test version.ref resolution
    let (group, name, version) = catalog.resolve_library_version("kotlin-stdlib").unwrap();
    assert_eq!(group, "org.jetbrains.kotlin");
    assert_eq!(name, "kotlin-stdlib");
    assert_eq!(version, "1.8.0");
    
    let (group, name, version) = catalog.resolve_library_version("junit-api").unwrap();
    assert_eq!(group, "org.junit.jupiter");
    assert_eq!(name, "junit-jupiter-api");
    assert_eq!(version, "5.9.0");
    
    // Test direct version
    let (group, name, version) = catalog.resolve_library_version("direct-version").unwrap();
    assert_eq!(group, "com.example");
    assert_eq!(name, "direct");
    assert_eq!(version, "2.0.0");
}

#[test]
fn test_version_ref_resolution_for_plugin() {
    let temp_dir = tempdir().unwrap();
    
    create_test_version_catalog(temp_dir.path(), r#"
[versions]
kotlin = "1.8.0"
spring = "3.0.0"

[plugins]
kotlin-jvm = { id = "org.jetbrains.kotlin.jvm", version.ref = "kotlin" }
spring-boot = { id = "org.springframework.boot", version = "2.7.0" }
java-library = { id = "java-library" }
application = { id = "application" }
"#);
    
    let catalog_files = find_version_catalog_files(temp_dir.path()).unwrap();
    let catalog = parse_version_catalog(&catalog_files[0]).unwrap();
    
    // Test version.ref resolution for plugins
    let (id, version) = catalog.resolve_plugin_version("kotlin-jvm").unwrap();
    assert_eq!(id, "org.jetbrains.kotlin.jvm");
    assert_eq!(version, Some("1.8.0".to_string()));
    
    // Test direct version for plugins
    let (id, version) = catalog.resolve_plugin_version("spring-boot").unwrap();
    assert_eq!(id, "org.springframework.boot");
    assert_eq!(version, Some("2.7.0".to_string()));
    
    // Test plugins without version (core plugins)
    let (id, version) = catalog.resolve_plugin_version("java-library").unwrap();
    assert_eq!(id, "java-library");
    assert_eq!(version, None);
    
    let (id, version) = catalog.resolve_plugin_version("application").unwrap();
    assert_eq!(id, "application");
    assert_eq!(version, None);
}