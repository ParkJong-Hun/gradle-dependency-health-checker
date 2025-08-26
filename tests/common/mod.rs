use std::fs;
use std::path::Path;
use tempfile::TempDir;

pub fn create_test_gradle_project() -> TempDir {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    
    // Create gradle directory structure
    fs::create_dir_all(temp_dir.path().join("gradle")).unwrap();
    
    // Create libs.versions.toml
    let libs_content = r#"
[versions]
kotlin = "1.9.0"
okhttp = "4.12.0"
gson = "2.10.1"
junit = "4.13.2"

[libraries]
kotlin-stdlib = { group = "org.jetbrains.kotlin", name = "kotlin-stdlib", version.ref = "kotlin" }
okhttp = { group = "com.squareup.okhttp3", name = "okhttp", version.ref = "okhttp" }
gson = { group = "com.google.code.gson", name = "gson", version.ref = "gson" }
junit = { group = "junit", name = "junit", version.ref = "junit" }

[plugins]
android-application = { id = "com.android.application", version = "8.0.0" }
"#;
    fs::write(temp_dir.path().join("gradle/libs.versions.toml"), libs_content).unwrap();
    
    // Create app module with build.gradle
    fs::create_dir_all(temp_dir.path().join("app")).unwrap();
    let app_build_gradle = r#"
dependencies {
    implementation 'com.squareup.okhttp3:okhttp:4.10.0'
    implementation 'com.google.code.gson:gson:2.10.1'
    testImplementation 'junit:junit:4.13.2'
}
"#;
    fs::write(temp_dir.path().join("app/build.gradle"), app_build_gradle).unwrap();
    
    // Create lib module with build.gradle.kts
    fs::create_dir_all(temp_dir.path().join("lib")).unwrap();
    let lib_build_gradle = r#"
dependencies {
    implementation(libs.okhttp)
    api(libs.gson)
    testImplementation(libs.junit)
}
"#;
    fs::write(temp_dir.path().join("lib/build.gradle.kts"), lib_build_gradle).unwrap();
    
    temp_dir
}

pub fn create_test_version_catalog(temp_dir: &Path, content: &str) {
    fs::create_dir_all(temp_dir.join("gradle")).unwrap();
    fs::write(temp_dir.join("gradle/libs.versions.toml"), content).unwrap();
}

pub fn create_test_build_gradle(temp_dir: &Path, filename: &str, content: &str) {
    let parent = temp_dir.join(filename).parent().unwrap().to_path_buf();
    fs::create_dir_all(parent).unwrap();
    fs::write(temp_dir.join(filename), content).unwrap();
}