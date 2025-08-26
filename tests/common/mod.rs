use std::fs;
use std::path::Path;

pub fn create_test_version_catalog(temp_dir: &Path, content: &str) {
    fs::create_dir_all(temp_dir.join("gradle")).unwrap();
    fs::write(temp_dir.join("gradle/libs.versions.toml"), content).unwrap();
}