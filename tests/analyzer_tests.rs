/*
 * Copyright (c) 2025 ParkJong-Hun
 *
 * Licensed under the MIT License.
 * See LICENSE file in the project root for full license information.
 */

mod common;

use gradle_dependency_health_checker::analyzer::find_duplicate_dependencies;
use common::{create_test_build_gradle, create_test_version_catalog};
use tempfile::tempdir;

#[test]
fn test_detect_version_conflicts() {
    let temp_dir = tempdir().unwrap();
    
    // Create app/build.gradle with okhttp 4.10.0
    let app_build_gradle = r#"
dependencies {
    implementation 'com.squareup.okhttp3:okhttp:4.10.0'
    implementation 'com.google.code.gson:gson:2.10.1'
}
"#;
    create_test_build_gradle(temp_dir.path(), "app/build.gradle", app_build_gradle);
    
    // Create lib/build.gradle with okhttp 4.12.0
    let lib_build_gradle = r#"
dependencies {
    implementation 'com.squareup.okhttp3:okhttp:4.12.0'
    implementation 'com.google.code.gson:gson:2.10.1'
}
"#;
    create_test_build_gradle(temp_dir.path(), "lib/build.gradle", lib_build_gradle);
    
    let analysis = find_duplicate_dependencies(temp_dir.path()).unwrap();
    
    // Should detect version conflict for okhttp
    assert_eq!(analysis.version_conflicts.len(), 1);
    assert!(analysis.version_conflicts.contains_key("com.squareup.okhttp3:okhttp"));
    
    let okhttp_conflicts = analysis.version_conflicts.get("com.squareup.okhttp3:okhttp").unwrap();
    assert_eq!(okhttp_conflicts.len(), 2);
    
    // Should detect duplicate for gson (same version)
    assert_eq!(analysis.regular_duplicates.len(), 1);
    assert!(analysis.regular_duplicates.contains_key("com.google.code.gson:gson"));
}

#[test]
fn test_detect_mixed_catalog_and_direct_conflicts() {
    let temp_dir = tempdir().unwrap();
    
    // Create version catalog with okhttp 4.12.0
    let catalog_content = r#"
[versions]
okhttp = "4.12.0"

[libraries]
okhttp = { group = "com.squareup.okhttp3", name = "okhttp", version.ref = "okhttp" }
"#;
    create_test_version_catalog(temp_dir.path(), catalog_content);
    
    // Create app/build.gradle with libs.okhttp
    let app_build_gradle = r#"
dependencies {
    implementation libs.okhttp
}
"#;
    create_test_build_gradle(temp_dir.path(), "app/build.gradle", app_build_gradle);
    
    // Create lib/build.gradle with direct okhttp 4.10.0
    let lib_build_gradle = r#"
dependencies {
    implementation 'com.squareup.okhttp3:okhttp:4.10.0'
}
"#;
    create_test_build_gradle(temp_dir.path(), "lib/build.gradle", lib_build_gradle);
    
    let analysis = find_duplicate_dependencies(temp_dir.path()).unwrap();
    
    // Should detect version conflict between catalog and direct declaration
    assert_eq!(analysis.version_conflicts.len(), 1);
    assert!(analysis.version_conflicts.contains_key("com.squareup.okhttp3:okhttp"));
    
    let conflicts = analysis.version_conflicts.get("com.squareup.okhttp3:okhttp").unwrap();
    assert_eq!(conflicts.len(), 2);
    
    // Verify one is from version catalog and one is direct
    let catalog_dep = conflicts.iter().find(|d| matches!(d.source_type, gradle_dependency_health_checker::parser::DependencySourceType::VersionCatalog(_))).unwrap();
    let direct_dep = conflicts.iter().find(|d| matches!(d.source_type, gradle_dependency_health_checker::parser::DependencySourceType::Direct)).unwrap();
    
    assert_eq!(catalog_dep.dependency.version, Some("4.12.0".to_string()));
    assert_eq!(direct_dep.dependency.version, Some("4.10.0".to_string()));
}

#[test]
fn test_ignore_same_file_duplicates() {
    let temp_dir = tempdir().unwrap();
    
    // Create single file with same dependency twice (should be ignored)
    let build_gradle = r#"
dependencies {
    implementation 'com.squareup.okhttp3:okhttp:4.12.0'
    testImplementation 'com.squareup.okhttp3:okhttp:4.12.0'
}
"#;
    create_test_build_gradle(temp_dir.path(), "build.gradle", build_gradle);
    
    let analysis = find_duplicate_dependencies(temp_dir.path()).unwrap();
    
    // Should not detect any duplicates (same file)
    assert_eq!(analysis.version_conflicts.len(), 0);
    assert_eq!(analysis.regular_duplicates.len(), 0);
}

#[test]
fn test_no_duplicates_single_dependency() {
    let temp_dir = tempdir().unwrap();
    
    let build_gradle = r#"
dependencies {
    implementation 'com.squareup.okhttp3:okhttp:4.12.0'
}
"#;
    create_test_build_gradle(temp_dir.path(), "build.gradle", build_gradle);
    
    let analysis = find_duplicate_dependencies(temp_dir.path()).unwrap();
    
    assert_eq!(analysis.version_conflicts.len(), 0);
    assert_eq!(analysis.regular_duplicates.len(), 0);
}

#[test]
fn test_complex_scenario_multiple_conflicts_and_duplicates() {
    let temp_dir = tempdir().unwrap();
    
    // Version catalog
    let catalog_content = r#"
[versions]
okhttp = "4.12.0"
gson = "2.10.1"

[libraries]
okhttp = { group = "com.squareup.okhttp3", name = "okhttp", version.ref = "okhttp" }
gson = { group = "com.google.code.gson", name = "gson", version.ref = "gson" }
"#;
    create_test_version_catalog(temp_dir.path(), catalog_content);
    
    // App module - uses catalog
    let app_build_gradle = r#"
dependencies {
    implementation libs.okhttp
    implementation libs.gson
}
"#;
    create_test_build_gradle(temp_dir.path(), "app/build.gradle", app_build_gradle);
    
    // Lib module - direct dependencies, same versions
    let lib_build_gradle = r#"
dependencies {
    implementation 'com.squareup.okhttp3:okhttp:4.12.0'
    implementation 'com.google.code.gson:gson:2.10.1'
}
"#;
    create_test_build_gradle(temp_dir.path(), "lib/build.gradle", lib_build_gradle);
    
    // Feature module - different okhttp version, same gson
    let feature_build_gradle = r#"
dependencies {
    implementation 'com.squareup.okhttp3:okhttp:4.10.0'
    implementation 'com.google.code.gson:gson:2.10.1'
}
"#;
    create_test_build_gradle(temp_dir.path(), "feature/build.gradle", feature_build_gradle);
    
    let analysis = find_duplicate_dependencies(temp_dir.path()).unwrap();
    
    // okhttp should be a version conflict (4.12.0 vs 4.10.0)
    assert_eq!(analysis.version_conflicts.len(), 1);
    assert!(analysis.version_conflicts.contains_key("com.squareup.okhttp3:okhttp"));
    
    let okhttp_conflicts = analysis.version_conflicts.get("com.squareup.okhttp3:okhttp").unwrap();
    assert_eq!(okhttp_conflicts.len(), 3); // app, lib, feature
    
    // gson should be a regular duplicate (all same version)
    assert_eq!(analysis.regular_duplicates.len(), 1);
    assert!(analysis.regular_duplicates.contains_key("com.google.code.gson:gson"));
    
    let gson_duplicates = analysis.regular_duplicates.get("com.google.code.gson:gson").unwrap();
    assert_eq!(gson_duplicates.len(), 3); // app, lib, feature
}