/*
 * Copyright (c) 2025 ParkJong-Hun
 *
 * Licensed under the MIT License.
 * See LICENSE file in the project root for full license information.
 */

use gradle_dependency_health_checker::cli::{Args, Commands, validate_args};
use gradle_dependency_health_checker::config::Config;
use clap::Parser;

#[test]
fn test_valid_all_command() {
    let config = Config::default();
    let args = Args {
        path: std::path::PathBuf::from("."),
        output: None,
        silent: false,
        command: Some(Commands::All {
            min_version_conflicts: Some(2),
            min_duplicate_dependencies: Some(3),
            min_duplicate_plugins: Some(2),
            min_bundle_size: Some(2),
            min_bundle_modules: Some(2),
            max_bundle_recommendations: Some(5),
        }),
    };
    
    assert!(validate_args(&args, &config).is_ok());
}

#[test]
fn test_invalid_version_conflicts_threshold() {
    let config = Config::default();
    let args = Args {
        path: std::path::PathBuf::from("."),
        output: None,
        silent: false,
        command: Some(Commands::Conflicts {
            min_version_conflicts: Some(1),
        }),
    };
    
    let result = validate_args(&args, &config);
    assert!(result.is_err());
    assert!(format!("{}", result.unwrap_err()).contains("--min-version-conflicts must be at least 2"));
}

#[test]
fn test_invalid_duplicate_dependencies_threshold() {
    let config = Config::default();
    let args = Args {
        path: std::path::PathBuf::from("."),
        output: None,
        silent: false,
        command: Some(Commands::Dependencies {
            min_duplicate_dependencies: Some(0),
        }),
    };
    
    let result = validate_args(&args, &config);
    assert!(result.is_err());
    assert!(format!("{}", result.unwrap_err()).contains("--min-duplicate-dependencies must be at least 2"));
}

#[test]
fn test_invalid_duplicate_plugins_threshold() {
    let config = Config::default();
    let args = Args {
        path: std::path::PathBuf::from("."),
        output: None,
        silent: false,
        command: Some(Commands::Plugins {
            min_duplicate_plugins: Some(1),
        }),
    };
    
    let result = validate_args(&args, &config);
    assert!(result.is_err());
    assert!(format!("{}", result.unwrap_err()).contains("--min-duplicate-plugins must be at least 2"));
}

#[test]
fn test_parse_args_defaults() {
    let args = Args::try_parse_from(&["program"]).unwrap();
    
    assert_eq!(args.path, std::path::PathBuf::from("."));
    assert!(args.command.is_none());
    assert!(args.output.is_none());
    assert!(!args.silent);
}

#[test]
fn test_parse_subcommand_conflicts() {
    let args = Args::try_parse_from(&[
        "program",
        "--path", "/custom/path",
        "conflicts",
        "--min-version-conflicts", "5"
    ]).unwrap();
    
    assert_eq!(args.path, std::path::PathBuf::from("/custom/path"));
    match args.command {
        Some(Commands::Conflicts { min_version_conflicts }) => {
            assert_eq!(min_version_conflicts, Some(5));
        },
        _ => panic!("Expected Conflicts command"),
    }
}

#[test]
fn test_parse_subcommand_dependencies() {
    let args = Args::try_parse_from(&[
        "program",
        "dependencies",
        "--min-duplicate-dependencies", "3"
    ]).unwrap();
    
    match args.command {
        Some(Commands::Dependencies { min_duplicate_dependencies }) => {
            assert_eq!(min_duplicate_dependencies, Some(3));
        },
        _ => panic!("Expected Dependencies command"),
    }
}

#[test]
fn test_parse_subcommand_plugins() {
    let args = Args::try_parse_from(&[
        "program",
        "plugins",
        "--min-duplicate-plugins", "4"
    ]).unwrap();
    
    match args.command {
        Some(Commands::Plugins { min_duplicate_plugins }) => {
            assert_eq!(min_duplicate_plugins, Some(4));
        },
        _ => panic!("Expected Plugins command"),
    }
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
fn test_default_behavior_validation() {
    let config = Config::default();
    let args = Args {
        path: std::path::PathBuf::from("."),
        output: None,
        silent: false,
        command: None,
    };
    
    // Default behavior should not require validation
    assert!(validate_args(&args, &config).is_ok());
}

#[test]
fn test_parse_output_option() {
    let args = Args::try_parse_from(&[
        "program",
        "--output", "results.json"
    ]).unwrap();
    
    assert_eq!(args.output, Some(std::path::PathBuf::from("results.json")));
}

#[test]
fn test_parse_output_with_subcommand() {
    let args = Args::try_parse_from(&[
        "program",
        "--path", "/test/path",
        "--output", "analysis.json",
        "conflicts",
        "--min-version-conflicts", "3"
    ]).unwrap();
    
    assert_eq!(args.path, std::path::PathBuf::from("/test/path"));
    assert_eq!(args.output, Some(std::path::PathBuf::from("analysis.json")));
    
    match args.command {
        Some(Commands::Conflicts { min_version_conflicts }) => {
            assert_eq!(min_version_conflicts, Some(3));
        },
        _ => panic!("Expected Conflicts command"),
    }
}

#[test]
fn test_parse_silent_option() {
    let args = Args::try_parse_from(&[
        "program",
        "--silent"
    ]).unwrap();
    
    assert!(args.silent);
}

#[test]
fn test_parse_silent_with_output() {
    let args = Args::try_parse_from(&[
        "program",
        "--output", "results.json",
        "--silent"
    ]).unwrap();
    
    assert_eq!(args.output, Some(std::path::PathBuf::from("results.json")));
    assert!(args.silent);
}