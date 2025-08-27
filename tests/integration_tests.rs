/*
 * Copyright (c) 2025 ParkJong-Hun
 *
 * Licensed under the MIT License.
 * See LICENSE file in the project root for full license information.
 */

mod common;

use assert_cmd::Command;
use common::{create_test_build_gradle, create_test_version_catalog};
use tempfile::tempdir;

#[test]
fn test_end_to_end_basic_functionality() {
    let temp_dir = tempdir().unwrap();
    
    // Create version catalog
    let catalog_content = r#"
[versions]
okhttp = "4.12.0"
gson = "2.10.1"

[libraries]
okhttp = { group = "com.squareup.okhttp3", name = "okhttp", version.ref = "okhttp" }
gson = { group = "com.google.code.gson", name = "gson", version.ref = "gson" }
"#;
    create_test_version_catalog(temp_dir.path(), catalog_content);
    
    // Create conflicting dependencies - 2 conflicts to meet threshold
    let app_build_gradle = r#"
dependencies {
    implementation 'com.squareup.okhttp3:okhttp:4.10.0'
    implementation 'com.google.code.gson:gson:2.9.0'
}
"#;
    create_test_build_gradle(temp_dir.path(), "app/build.gradle", app_build_gradle);
    
    let lib_build_gradle = r#"
dependencies {
    implementation libs.okhttp
    implementation libs.gson
}
"#;
    create_test_build_gradle(temp_dir.path(), "lib/build.gradle", lib_build_gradle);
    
    let mut cmd = Command::cargo_bin("gradle-dependency-health-checker").unwrap();
    cmd.arg("--path").arg(temp_dir.path());
    
    let output = cmd.assert().success();
    let stdout = std::str::from_utf8(&output.get_output().stdout).unwrap();
    
    // Should detect version conflicts
    assert!(stdout.contains("🚨 Found 2 version conflicts"));
    assert!(stdout.contains("com.squareup.okhttp3:okhttp"));
    assert!(stdout.contains("com.google.code.gson:gson"));
    assert!(stdout.contains("[via libs.okhttp]"));
    assert!(stdout.contains("[via libs.gson]"));
    assert!(stdout.contains("4.10.0"));
    assert!(stdout.contains("4.12.0"));
    assert!(stdout.contains("2.9.0"));
    assert!(stdout.contains("2.10.1"));
}

#[test]
fn test_threshold_filtering() {
    let temp_dir = tempdir().unwrap();
    
    // Create only one conflict (below default threshold of 2)
    let app_build_gradle = r#"
dependencies {
    implementation 'com.squareup.okhttp3:okhttp:4.10.0'
}
"#;
    create_test_build_gradle(temp_dir.path(), "app/build.gradle", app_build_gradle);
    
    let lib_build_gradle = r#"
dependencies {
    implementation 'com.squareup.okhttp3:okhttp:4.12.0'
}
"#;
    create_test_build_gradle(temp_dir.path(), "lib/build.gradle", lib_build_gradle);
    
    // Test with higher threshold (should show no issues)
    let mut cmd = Command::cargo_bin("gradle-dependency-health-checker").unwrap();
    cmd.arg("--path").arg(temp_dir.path())
       .arg("all")
       .arg("--min-version-conflicts").arg("3");
    
    let output = cmd.assert().success();
    let stdout = std::str::from_utf8(&output.get_output().stdout).unwrap();
    
    assert!(stdout.contains("✅ No issues found above the specified thresholds"));
    assert!(stdout.contains("(Found 1 version conflicts"));
}

#[test]
fn test_invalid_threshold_error() {
    let temp_dir = tempdir().unwrap();
    create_test_build_gradle(temp_dir.path(), "build.gradle", "");
    
    let mut cmd = Command::cargo_bin("gradle-dependency-health-checker").unwrap();
    cmd.arg("--path").arg(temp_dir.path())
       .arg("conflicts")
       .arg("--min-version-conflicts").arg("1");
    
    let output = cmd.assert().failure();
    let stderr = std::str::from_utf8(&output.get_output().stderr).unwrap();
    
    assert!(stderr.contains("--min-version-conflicts must be at least 2"));
}

#[test]
fn test_no_gradle_files() {
    let temp_dir = tempdir().unwrap();
    // Don't create any gradle files
    
    let mut cmd = Command::cargo_bin("gradle-dependency-health-checker").unwrap();
    cmd.arg("--path").arg(temp_dir.path());
    
    let output = cmd.assert().success();
    let stdout = std::str::from_utf8(&output.get_output().stdout).unwrap();
    
    assert!(stdout.contains("✅ No issues found above the specified thresholds"));
}

#[test]
fn test_mixed_catalog_and_direct_dependencies() {
    let temp_dir = tempdir().unwrap();
    
    // Create comprehensive test scenario
    let catalog_content = r#"
[versions]
okhttp = "4.12.0"
gson = "2.10.1"
junit = "4.13.2"

[libraries]
okhttp = { group = "com.squareup.okhttp3", name = "okhttp", version.ref = "okhttp" }
gson = { group = "com.google.code.gson", name = "gson", version.ref = "gson" }
junit = { group = "junit", name = "junit", version.ref = "junit" }
"#;
    create_test_version_catalog(temp_dir.path(), catalog_content);
    
    // App uses catalog
    let app_build_gradle = r#"
dependencies {
    implementation libs.okhttp
    implementation libs.gson
    testImplementation libs.junit
}
"#;
    create_test_build_gradle(temp_dir.path(), "app/build.gradle", app_build_gradle);
    
    // Lib uses direct dependencies - conflicts and duplicates
    let lib_build_gradle = r#"
dependencies {
    implementation 'com.squareup.okhttp3:okhttp:4.10.0'
    implementation 'com.google.code.gson:gson:2.10.1'
    testImplementation 'junit:junit:4.13.2'
}
"#;
    create_test_build_gradle(temp_dir.path(), "lib/build.gradle", lib_build_gradle);
    
    // Add third module to create 2 conflicts  
    let feature_build_gradle = r#"
dependencies {
    implementation 'com.squareup.okhttp3:okhttp:4.11.0'
    implementation 'com.google.code.gson:gson:2.9.0'
    testImplementation 'junit:junit:4.13.2'
}
"#;
    create_test_build_gradle(temp_dir.path(), "feature/build.gradle", feature_build_gradle);
    
    let mut cmd = Command::cargo_bin("gradle-dependency-health-checker").unwrap();
    cmd.arg("--path").arg(temp_dir.path());
    
    let output = cmd.assert().success();
    let stdout = std::str::from_utf8(&output.get_output().stdout).unwrap();
    
    // Should have 2 version conflicts (okhttp and gson)
    assert!(stdout.contains("🚨 Found 2 version conflicts"));
    assert!(stdout.contains("com.squareup.okhttp3:okhttp"));
    assert!(stdout.contains("com.google.code.gson:gson"));
    assert!(stdout.contains("[via libs.okhttp]"));
    assert!(stdout.contains("[via libs.gson]"));
    // junit appears only in duplicates section (not in version conflicts), so we don't test for it here
}

#[test]
fn test_help_message() {
    let mut cmd = Command::cargo_bin("gradle-dependency-health-checker").unwrap();
    cmd.arg("--help");
    
    let output = cmd.assert().success();
    let stdout = std::str::from_utf8(&output.get_output().stdout).unwrap();
    
    assert!(stdout.contains("gradle-dependency-health-checker"));
    assert!(stdout.contains("Check for duplicate dependencies, plugins, and version conflicts"));
    assert!(stdout.contains("conflicts"));
    assert!(stdout.contains("dependencies"));
    assert!(stdout.contains("plugins"));
    assert!(stdout.contains("duplicates"));
    assert!(stdout.contains("bundles"));
    assert!(stdout.contains("all"));
}

#[test]
fn test_nonexistent_path() {
    let mut cmd = Command::cargo_bin("gradle-dependency-health-checker").unwrap();
    cmd.arg("--path").arg("/nonexistent/path");
    
    cmd.assert().failure();
}