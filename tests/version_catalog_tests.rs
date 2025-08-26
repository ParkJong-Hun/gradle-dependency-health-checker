/*
 * Copyright (c) 2025 ParkJong-Hun
 *
 * Licensed under the MIT License.
 * See LICENSE file in the project root for full license information.
 */

mod common;

use gradle_dependency_health_checker::version_catalog::{parse_version_catalog, find_version_catalog_files};
use common::{create_test_version_catalog};
use tempfile::tempdir;

#[test]
fn test_parse_simple_version_catalog() {
    let temp_dir = tempdir().unwrap();
    let catalog_content = r#"
[versions]
kotlin = "1.9.0"
okhttp = "4.12.0"

[libraries]
kotlin-stdlib = { group = "org.jetbrains.kotlin", name = "kotlin-stdlib", version.ref = "kotlin" }
okhttp = { group = "com.squareup.okhttp3", name = "okhttp", version.ref = "okhttp" }

[plugins]
android-application = { id = "com.android.application", version = "8.0.0" }
"#;
    
    create_test_version_catalog(temp_dir.path(), catalog_content);
    let catalog_path = temp_dir.path().join("gradle/libs.versions.toml");
    
    let catalog = parse_version_catalog(&catalog_path).unwrap();
    
    // Test versions section
    assert!(catalog.versions.is_some());
    let versions = catalog.versions.as_ref().unwrap();
    assert_eq!(versions.get("kotlin"), Some(&"1.9.0".to_string()));
    assert_eq!(versions.get("okhttp"), Some(&"4.12.0".to_string()));
    
    // Test libraries section
    assert!(catalog.libraries.is_some());
    let libraries = catalog.libraries.as_ref().unwrap();
    assert!(libraries.contains_key("kotlin-stdlib"));
    assert!(libraries.contains_key("okhttp"));
    
    // Test plugins section
    assert!(catalog.plugins.is_some());
    let plugins = catalog.plugins.as_ref().unwrap();
    assert!(plugins.contains_key("android-application"));
}

#[test]
fn test_resolve_library_version() {
    let temp_dir = tempdir().unwrap();
    let catalog_content = r#"
[versions]
kotlin = "1.9.0"
gson = "2.10.1"

[libraries]
kotlin-stdlib = { group = "org.jetbrains.kotlin", name = "kotlin-stdlib", version.ref = "kotlin" }
gson = { group = "com.google.code.gson", name = "gson", version.ref = "gson" }
retrofit = { group = "com.squareup.retrofit2", name = "retrofit", version = "2.9.0" }
"#;
    
    create_test_version_catalog(temp_dir.path(), catalog_content);
    let catalog_path = temp_dir.path().join("gradle/libs.versions.toml");
    let catalog = parse_version_catalog(&catalog_path).unwrap();
    
    // Test version reference resolution
    let (group, name, version) = catalog.resolve_library_version("kotlin-stdlib").unwrap();
    assert_eq!(group, "org.jetbrains.kotlin");
    assert_eq!(name, "kotlin-stdlib");
    assert_eq!(version, "1.9.0");
    
    // Test direct version
    let (group, name, version) = catalog.resolve_library_version("retrofit").unwrap();
    assert_eq!(group, "com.squareup.retrofit2");
    assert_eq!(name, "retrofit");
    assert_eq!(version, "2.9.0");
    
    // Test non-existent library
    assert!(catalog.resolve_library_version("non-existent").is_none());
}

#[test]
fn test_find_version_catalog_files() {
    let temp_dir = tempdir().unwrap();
    
    // Create nested structure with multiple catalog files
    create_test_version_catalog(temp_dir.path(), "[versions]\ntest = \"1.0\"");
    
    let subdir = temp_dir.path().join("submodule");
    std::fs::create_dir_all(&subdir).unwrap();
    create_test_version_catalog(&subdir, "[versions]\nother = \"2.0\"");
    
    let catalog_files = find_version_catalog_files(temp_dir.path()).unwrap();
    
    assert_eq!(catalog_files.len(), 2);
    assert!(catalog_files.iter().any(|p| p.file_name().unwrap() == "libs.versions.toml"));
}

#[test]
fn test_complex_library_name_resolution() {
    let temp_dir = tempdir().unwrap();
    let catalog_content = r#"
[versions]
androidx-core = "1.13.0"

[libraries]
androidx-core-ktx = { group = "androidx.core", name = "core-ktx", version.ref = "androidx-core" }
"#;
    
    create_test_version_catalog(temp_dir.path(), catalog_content);
    let catalog_path = temp_dir.path().join("gradle/libs.versions.toml");
    let catalog = parse_version_catalog(&catalog_path).unwrap();
    
    let (group, name, version) = catalog.resolve_library_version("androidx-core-ktx").unwrap();
    assert_eq!(group, "androidx.core");
    assert_eq!(name, "core-ktx");
    assert_eq!(version, "1.13.0");
}