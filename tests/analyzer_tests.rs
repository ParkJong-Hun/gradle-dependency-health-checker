/*
 * Copyright (c) 2025 ParkJong-Hun
 *
 * Licensed under the MIT License.
 * See LICENSE file in the project root for full license information.
 */

mod common;

use gradle_dependency_health_checker::analyzer::perform_complete_analysis;
use common::create_test_build_gradle;
use tempfile::tempdir;

#[test] 
fn test_basic_analysis() {
    let temp_dir = tempdir().unwrap();
    
    // Create test files
    create_test_build_gradle(temp_dir.path(), "app", r#"
dependencies {
    implementation 'com.example:lib:1.0.0'
}
"#);
    
    // Test basic analysis
    let result = perform_complete_analysis(temp_dir.path(), 2, 2);
    assert!(result.is_ok());
}