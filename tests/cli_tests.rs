/*
 * Copyright (c) 2025 ParkJong-Hun
 *
 * Licensed under the MIT License.
 * See LICENSE file in the project root for full license information.
 */

use gradle_dependency_health_checker::cli::{Args, validate_args};
use clap::Parser;

#[test]
fn test_valid_args() {
    let args = Args {
        path: std::path::PathBuf::from("."),
        min_version_conflicts: 2,
        min_duplicate_dependencies: 3,
    };
    
    assert!(validate_args(&args).is_ok());
}

#[test]
fn test_invalid_version_conflicts_threshold() {
    let args = Args {
        path: std::path::PathBuf::from("."),
        min_version_conflicts: 1,
        min_duplicate_dependencies: 2,
    };
    
    let result = validate_args(&args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("min-version-conflicts must be at least 2"));
}

#[test]
fn test_invalid_duplicate_dependencies_threshold() {
    let args = Args {
        path: std::path::PathBuf::from("."),
        min_version_conflicts: 2,
        min_duplicate_dependencies: 0,
    };
    
    let result = validate_args(&args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("min-duplicate-dependencies must be at least 2"));
}

#[test]
fn test_both_invalid_thresholds() {
    let args = Args {
        path: std::path::PathBuf::from("."),
        min_version_conflicts: 1,
        min_duplicate_dependencies: 1,
    };
    
    let result = validate_args(&args);
    assert!(result.is_err());
    // Should fail on the first validation (version conflicts)
    assert!(result.unwrap_err().contains("min-version-conflicts must be at least 2"));
}

#[test]
fn test_parse_args_defaults() {
    let args = Args::try_parse_from(&["program"]).unwrap();
    
    assert_eq!(args.path, std::path::PathBuf::from("."));
    assert_eq!(args.min_version_conflicts, 2);
    assert_eq!(args.min_duplicate_dependencies, 2);
}

#[test]
fn test_parse_args_custom_values() {
    let args = Args::try_parse_from(&[
        "program",
        "--path", "/custom/path",
        "--min-version-conflicts", "5",
        "--min-duplicate-dependencies", "3"
    ]).unwrap();
    
    assert_eq!(args.path, std::path::PathBuf::from("/custom/path"));
    assert_eq!(args.min_version_conflicts, 5);
    assert_eq!(args.min_duplicate_dependencies, 3);
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
    // This should parse successfully but fail validation
    let args = Args::try_parse_from(&[
        "program",
        "--min-version-conflicts", "0"
    ]).unwrap();
    
    assert_eq!(args.min_version_conflicts, 0);
    assert!(validate_args(&args).is_err());
}