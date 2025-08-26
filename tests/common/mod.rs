/*
 * Copyright (c) 2025 ParkJong-Hun
 *
 * Licensed under the MIT License.
 * See LICENSE file in the project root for full license information.
 */

use std::fs;
use std::path::Path;

pub fn create_test_version_catalog(temp_dir: &Path, content: &str) {
    fs::create_dir_all(temp_dir.join("gradle")).unwrap();
    fs::write(temp_dir.join("gradle/libs.versions.toml"), content).unwrap();
}

pub fn create_test_build_gradle(temp_dir: &Path, module_name: &str, content: &str) {
    let module_dir = temp_dir.join(module_name);
    fs::create_dir_all(&module_dir).unwrap();
    fs::write(module_dir.join("build.gradle"), content).unwrap();
}