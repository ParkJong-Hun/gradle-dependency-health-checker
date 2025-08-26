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