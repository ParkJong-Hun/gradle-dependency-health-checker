/*
 * Copyright (c) 2025 ParkJong-Hun
 *
 * Licensed under the MIT License.
 * See LICENSE file in the project root for full license information.
 */

mod common;

use gradle_dependency_health_checker::parser::{parse_plugins_from_file, load_version_catalogs};
use gradle_dependency_health_checker::analyzer::perform_complete_analysis;
use common::create_test_build_gradle;
use tempfile::tempdir;
use std::fs;

#[test]
fn test_plugins_block_parsing() {
    let temp_dir = tempdir().unwrap();
    
    create_test_build_gradle(temp_dir.path(), "app", r#"
plugins {
    id 'java'
    id 'org.springframework.boot' version '2.7.0'
    id("kotlin-jvm") version "1.8.0"
    id("application")
}

apply plugin: 'jacoco'

dependencies {
    implementation 'org.springframework:spring-core:5.3.0'
}
"#);
    
    let version_catalogs = load_version_catalogs(temp_dir.path()).unwrap();
    let gradle_file = temp_dir.path().join("app/build.gradle");
    let plugins = parse_plugins_from_file(&gradle_file, &version_catalogs).unwrap();
    
    assert_eq!(plugins.len(), 5);
    
    // Check plugins block entries
    let java_plugin = plugins.iter().find(|p| p.plugin.id == "java").unwrap();
    assert_eq!(java_plugin.plugin.version, None);
    
    let spring_plugin = plugins.iter().find(|p| p.plugin.id == "org.springframework.boot").unwrap();
    assert_eq!(spring_plugin.plugin.version, Some("2.7.0".to_string()));
    
    let kotlin_plugin = plugins.iter().find(|p| p.plugin.id == "kotlin-jvm").unwrap();
    assert_eq!(kotlin_plugin.plugin.version, Some("1.8.0".to_string()));
    
    let app_plugin = plugins.iter().find(|p| p.plugin.id == "application").unwrap();
    assert_eq!(app_plugin.plugin.version, None);
    
    // Check apply plugin entry
    let jacoco_plugin = plugins.iter().find(|p| p.plugin.id == "jacoco").unwrap();
    assert_eq!(jacoco_plugin.plugin.version, None);
}

#[test]
fn test_kotlin_dsl_plugin_parsing() {
    let temp_dir = tempdir().unwrap();
    
    let build_gradle_kts_content = r#"
plugins {
    id("org.springframework.boot") version "2.7.0"
    id("java-library")
    application
}

apply(plugin = "jacoco")

dependencies {
    implementation("org.springframework:spring-core:5.3.0")
}
"#;
    
    let app_dir = temp_dir.path().join("app");
    fs::create_dir_all(&app_dir).unwrap();
    fs::write(app_dir.join("build.gradle.kts"), build_gradle_kts_content).unwrap();
    
    let version_catalogs = load_version_catalogs(temp_dir.path()).unwrap();
    let gradle_file = app_dir.join("build.gradle.kts");
    let plugins = parse_plugins_from_file(&gradle_file, &version_catalogs).unwrap();
    
    println!("Found plugins: {:?}", plugins);
    assert_eq!(plugins.len(), 4);
    
    let spring_plugin = plugins.iter().find(|p| p.plugin.id == "org.springframework.boot").unwrap();
    assert_eq!(spring_plugin.plugin.version, Some("2.7.0".to_string()));
    
    let java_lib_plugin = plugins.iter().find(|p| p.plugin.id == "java-library").unwrap();
    assert_eq!(java_lib_plugin.plugin.version, None);
    
    let app_plugin = plugins.iter().find(|p| p.plugin.id == "application").unwrap();
    assert_eq!(app_plugin.plugin.version, None);
    
    let jacoco_plugin = plugins.iter().find(|p| p.plugin.id == "jacoco").unwrap();
    assert_eq!(jacoco_plugin.plugin.version, None);
}

#[test]
fn test_version_catalog_plugin_parsing() {
    let temp_dir = tempdir().unwrap();
    
    // Create version catalog
    let libs_dir = temp_dir.path().join("gradle");
    fs::create_dir_all(&libs_dir).unwrap();
    fs::write(temp_dir.path().join("libs.versions.toml"), r#"
[versions]
kotlin = "1.8.0"
spring = "2.7.0"

[plugins]
kotlinJvm = { id = "org.jetbrains.kotlin.jvm", version.ref = "kotlin" }
springBoot = { id = "org.springframework.boot", version.ref = "spring" }
javaLibrary = { id = "java-library" }
"#).unwrap();
    
    create_test_build_gradle(temp_dir.path(), "app", r#"
plugins {
    alias(libs.plugins.kotlinJvm)
    alias(libs.plugins.springBoot)
    alias(libs.plugins.javaLibrary)
}

dependencies {
    implementation 'org.springframework:spring-core:5.3.0'
}
"#);
    
    let version_catalogs = load_version_catalogs(temp_dir.path()).unwrap();
    let gradle_file = temp_dir.path().join("app/build.gradle");
    let plugins = parse_plugins_from_file(&gradle_file, &version_catalogs).unwrap();
    
    println!("Version catalogs found: {}", version_catalogs.len());
    println!("Found plugins: {:?}", plugins);
    assert_eq!(plugins.len(), 3);
    
    let kotlin_plugin = plugins.iter().find(|p| p.plugin.id == "org.jetbrains.kotlin.jvm").unwrap();
    assert_eq!(kotlin_plugin.plugin.version, Some("1.8.0".to_string()));
    
    let spring_plugin = plugins.iter().find(|p| p.plugin.id == "org.springframework.boot").unwrap();
    assert_eq!(spring_plugin.plugin.version, Some("2.7.0".to_string()));
    
    let java_lib_plugin = plugins.iter().find(|p| p.plugin.id == "java-library").unwrap();
    assert_eq!(java_lib_plugin.plugin.version, None);
}

#[test]
fn test_duplicate_plugin_analysis() {
    let temp_dir = tempdir().unwrap();
    
    create_test_build_gradle(temp_dir.path(), "app", r#"
plugins {
    id 'java'
    id 'org.springframework.boot' version '2.7.0'
}
"#);
    
    create_test_build_gradle(temp_dir.path(), "lib", r#"
plugins {
    id 'java'
    id 'java-library'
}
"#);
    
    let analysis = perform_complete_analysis(temp_dir.path(), 2, 2).unwrap();
    
    // Should detect java plugin as duplicate across modules
    assert_eq!(analysis.plugin_analysis.duplicate_plugins.len(), 1);
    assert!(analysis.plugin_analysis.duplicate_plugins.contains_key("java"));
    
    let java_duplicates = &analysis.plugin_analysis.duplicate_plugins["java"];
    assert_eq!(java_duplicates.len(), 2);
}