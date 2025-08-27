/*
 * Copyright (c) 2025 ParkJong-Hun
 *
 * Licensed under the MIT License.
 * See LICENSE file in the project root for full license information.
 */

use gradle_dependency_health_checker::cli::{Args, validate_args};
use gradle_dependency_health_checker::config::Config;
use clap::Parser;

#[test]
fn test_valid_args() {
    let config = Config::default();
    let args = Args {
        path: std::path::PathBuf::from("."),
        min_version_conflicts: Some(2),
        min_duplicate_dependencies: Some(3),
        min_duplicate_plugins: Some(2),
        min_bundle_size: Some(2),
        min_bundle_modules: Some(2),
        max_bundle_recommendations: Some(5),
    };
    
    assert!(validate_args(&args, &config).is_ok());
}

#[test]
fn test_invalid_version_conflicts_threshold() {
    let config = Config::default();
    let args = Args {
        path: std::path::PathBuf::from("."),
        min_version_conflicts: Some(1),
        min_duplicate_dependencies: Some(2),
        min_duplicate_plugins: Some(2),
        min_bundle_size: Some(2),
        min_bundle_modules: Some(2),
        max_bundle_recommendations: Some(5),
    };
    
    let result = validate_args(&args, &config);
    assert!(result.is_err());
    assert!(format!("{}", result.unwrap_err()).contains("min-version-conflicts must be at least 2"));
}

#[test]
fn test_invalid_duplicate_dependencies_threshold() {
    let config = Config::default();
    let args = Args {
        path: std::path::PathBuf::from("."),
        min_version_conflicts: Some(2),
        min_duplicate_dependencies: Some(0),
        min_duplicate_plugins: Some(2),
        min_bundle_size: Some(2),
        min_bundle_modules: Some(2),
        max_bundle_recommendations: Some(5),
    };
    
    let result = validate_args(&args, &config);
    assert!(result.is_err());
    assert!(format!("{}", result.unwrap_err()).contains("min-duplicate-dependencies must be at least 2"));
}

#[test]
fn test_both_invalid_thresholds() {
    let config = Config::default();
    let args = Args {
        path: std::path::PathBuf::from("."),
        min_version_conflicts: Some(1),
        min_duplicate_dependencies: Some(1),
        min_duplicate_plugins: Some(2),
        min_bundle_size: Some(2),
        min_bundle_modules: Some(2),
        max_bundle_recommendations: Some(5),
    };
    
    let result = validate_args(&args, &config);
    assert!(result.is_err());
    // Should fail on the first validation (version conflicts)
    assert!(format!("{}", result.unwrap_err()).contains("min-version-conflicts must be at least 2"));
}

#[test]
fn test_parse_args_defaults() {
    let args = Args::try_parse_from(&["program"]).unwrap();
    
    assert_eq!(args.path, std::path::PathBuf::from("."));
    assert_eq!(args.min_version_conflicts, None);
    assert_eq!(args.min_duplicate_dependencies, None);
    assert_eq!(args.min_duplicate_plugins, None);
}

#[test]
fn test_parse_args_custom_values() {
    let args = Args::try_parse_from(&[
        "program",
        "--path", "/custom/path",
        "--min-version-conflicts", "5",
        "--min-duplicate-dependencies", "3",
        "--min-duplicate-plugins", "4"
    ]).unwrap();
    
    assert_eq!(args.path, std::path::PathBuf::from("/custom/path"));
    assert_eq!(args.min_version_conflicts, Some(5));
    assert_eq!(args.min_duplicate_dependencies, Some(3));
    assert_eq!(args.min_duplicate_plugins, Some(4));
}

#[test]
fn test_parse_args_short_path() {
    let args = Args::try_parse_from(&[
        "program",
        "-p", "/short/path"
    ]).unwrap();
    
    assert_eq!(args.path, std::path::PathBuf::from("/short/path"));
}

#[test]
fn test_parse_args_invalid_threshold() {
    let config = Config::default();
    // This should parse successfully but fail validation
    let args = Args::try_parse_from(&[
        "program",
        "--min-version-conflicts", "0"
    ]).unwrap();
    
    assert_eq!(args.min_version_conflicts, Some(0));
    assert!(validate_args(&args, &config).is_err());
}

#[test]
fn test_invalid_duplicate_plugins_threshold() {
    let config = Config::default();
    let args = Args {
        path: std::path::PathBuf::from("."),
        min_version_conflicts: Some(2),
        min_duplicate_dependencies: Some(2),
        min_duplicate_plugins: Some(1),
        min_bundle_size: Some(2),
        min_bundle_modules: Some(2),
        max_bundle_recommendations: Some(5),
    };
    
    let result = validate_args(&args, &config);
    assert!(result.is_err());
    assert!(format!("{}", result.unwrap_err()).contains("min-duplicate-plugins must be at least 2"));
}