/*
 * Copyright (c) 2025 ParkJong-Hun
 *
 * Licensed under the MIT License.
 * See LICENSE file in the project root for full license information.
 */

mod common;

use gradle_dependency_health_checker::parser::{find_gradle_files, parse_dependencies_from_file, load_version_catalogs};
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