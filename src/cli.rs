/*
 * Copyright (c) 2025 ParkJong-Hun
 *
 * Licensed under the MIT License.
 * See LICENSE file in the project root for full license information.
 */

use crate::config::Config;
use crate::error::{AnalysisError, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "gradle-dependency-health-checker")]
#[command(about = "Check for duplicate dependencies, plugins, and version conflicts in Gradle projects")]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
    
    #[arg(short, long, default_value = ".", global = true, help = "Path to the Gradle project to analyze")]
    pub path: PathBuf,
    
    #[arg(short, long, global = true, help = "Output results to JSON file instead of console")]
    pub output: Option<PathBuf>,
    
    #[arg(short, long, global = true, help = "Suppress all output messages (useful with --output)")]
    pub silent: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run all checks (version conflicts, duplicates, bundles) - default behavior
    All {
        #[arg(long, help = "Minimum number of version conflicts to display")]
        min_version_conflicts: Option<usize>,
        
        #[arg(long, help = "Minimum number of duplicate dependencies to display")]
        min_duplicate_dependencies: Option<usize>,
        
        #[arg(long, help = "Minimum number of duplicate plugins to display")]
        min_duplicate_plugins: Option<usize>,
        
        #[arg(long, help = "Minimum number of dependencies required for a bundle recommendation")]
        min_bundle_size: Option<usize>,
        
        #[arg(long, help = "Minimum number of modules sharing dependencies for bundle recommendation")]
        min_bundle_modules: Option<usize>,
        
        #[arg(long, help = "Maximum number of bundle recommendations to display")]
        max_bundle_recommendations: Option<usize>,
    },
    /// Check for version conflicts only
    Conflicts {
        #[arg(long, help = "Minimum number of version conflicts to display")]
        min_version_conflicts: Option<usize>,
    },
    /// Check for duplicate dependencies only
    Dependencies {
        #[arg(long, help = "Minimum number of duplicate dependencies to display")]
        min_duplicate_dependencies: Option<usize>,
    },
    /// Check for duplicate plugins only
    Plugins {
        #[arg(long, help = "Minimum number of duplicate plugins to display")]
        min_duplicate_plugins: Option<usize>,
    },
    /// Check for both duplicate dependencies and plugins
    Duplicates {
        #[arg(long, help = "Minimum number of duplicate dependencies to display")]
        min_duplicate_dependencies: Option<usize>,
        
        #[arg(long, help = "Minimum number of duplicate plugins to display")]
        min_duplicate_plugins: Option<usize>,
    },
    /// Generate bundle recommendations only
    Bundles {
        #[arg(long, help = "Minimum number of dependencies required for a bundle recommendation")]
        min_bundle_size: Option<usize>,
        
        #[arg(long, help = "Minimum number of modules sharing dependencies for bundle recommendation")]
        min_bundle_modules: Option<usize>,
        
        #[arg(long, help = "Maximum number of bundle recommendations to display")]
        max_bundle_recommendations: Option<usize>,
    },
}

pub struct AnalysisOptions {
    pub min_version_conflicts: usize,
    pub min_duplicate_dependencies: usize,
    pub min_duplicate_plugins: usize,
    pub min_bundle_size: usize,
    pub min_bundle_modules: usize,
    pub max_bundle_recommendations: usize,
}

impl Args {
    pub fn get_analysis_options(&self, config: &Config) -> AnalysisOptions {
        match &self.command {
            Some(Commands::All { 
                min_version_conflicts, 
                min_duplicate_dependencies, 
                min_duplicate_plugins, 
                min_bundle_size, 
                min_bundle_modules, 
                max_bundle_recommendations 
            }) => {
                AnalysisOptions {
                    min_version_conflicts: min_version_conflicts.unwrap_or(config.default_min_version_conflicts),
                    min_duplicate_dependencies: min_duplicate_dependencies.unwrap_or(config.default_min_duplicate_dependencies),
                    min_duplicate_plugins: min_duplicate_plugins.unwrap_or(config.default_min_duplicate_plugins),
                    min_bundle_size: min_bundle_size.unwrap_or(config.default_min_bundle_size),
                    min_bundle_modules: min_bundle_modules.unwrap_or(config.default_min_bundle_modules),
                    max_bundle_recommendations: max_bundle_recommendations.unwrap_or(config.default_max_bundle_recommendations),
                }
            }
            Some(Commands::Conflicts { min_version_conflicts }) => {
                AnalysisOptions {
                    min_version_conflicts: min_version_conflicts.unwrap_or(config.default_min_version_conflicts),
                    min_duplicate_dependencies: usize::MAX,
                    min_duplicate_plugins: usize::MAX,
                    min_bundle_size: usize::MAX,
                    min_bundle_modules: usize::MAX,
                    max_bundle_recommendations: 0,
                }
            }
            Some(Commands::Dependencies { min_duplicate_dependencies }) => {
                AnalysisOptions {
                    min_version_conflicts: usize::MAX,
                    min_duplicate_dependencies: min_duplicate_dependencies.unwrap_or(config.default_min_duplicate_dependencies),
                    min_duplicate_plugins: usize::MAX,
                    min_bundle_size: usize::MAX,
                    min_bundle_modules: usize::MAX,
                    max_bundle_recommendations: 0,
                }
            }
            Some(Commands::Plugins { min_duplicate_plugins }) => {
                AnalysisOptions {
                    min_version_conflicts: usize::MAX,
                    min_duplicate_dependencies: usize::MAX,
                    min_duplicate_plugins: min_duplicate_plugins.unwrap_or(config.default_min_duplicate_plugins),
                    min_bundle_size: usize::MAX,
                    min_bundle_modules: usize::MAX,
                    max_bundle_recommendations: 0,
                }
            }
            Some(Commands::Duplicates { min_duplicate_dependencies, min_duplicate_plugins }) => {
                AnalysisOptions {
                    min_version_conflicts: usize::MAX,
                    min_duplicate_dependencies: min_duplicate_dependencies.unwrap_or(config.default_min_duplicate_dependencies),
                    min_duplicate_plugins: min_duplicate_plugins.unwrap_or(config.default_min_duplicate_plugins),
                    min_bundle_size: usize::MAX,
                    min_bundle_modules: usize::MAX,
                    max_bundle_recommendations: 0,
                }
            }
            Some(Commands::Bundles { min_bundle_size, min_bundle_modules, max_bundle_recommendations }) => {
                AnalysisOptions {
                    min_version_conflicts: usize::MAX,
                    min_duplicate_dependencies: usize::MAX,
                    min_duplicate_plugins: usize::MAX,
                    min_bundle_size: min_bundle_size.unwrap_or(config.default_min_bundle_size),
                    min_bundle_modules: min_bundle_modules.unwrap_or(config.default_min_bundle_modules),
                    max_bundle_recommendations: max_bundle_recommendations.unwrap_or(config.default_max_bundle_recommendations),
                }
            }
            None => {
                // Default behavior: run all checks
                AnalysisOptions {
                    min_version_conflicts: config.default_min_version_conflicts,
                    min_duplicate_dependencies: config.default_min_duplicate_dependencies,
                    min_duplicate_plugins: config.default_min_duplicate_plugins,
                    min_bundle_size: config.default_min_bundle_size,
                    min_bundle_modules: config.default_min_bundle_modules,
                    max_bundle_recommendations: config.default_max_bundle_recommendations,
                }
            }
        }
    }
}

pub fn validate_args(args: &Args, config: &Config) -> Result<()> {
    let min_threshold = config.min_threshold_value;
    
    match &args.command {
        Some(Commands::All { 
            min_version_conflicts, 
            min_duplicate_dependencies, 
            min_duplicate_plugins, 
            min_bundle_size, 
            min_bundle_modules, 
            max_bundle_recommendations 
        }) => {
            validate_threshold("--min-version-conflicts", *min_version_conflicts, min_threshold)?;
            validate_threshold("--min-duplicate-dependencies", *min_duplicate_dependencies, min_threshold)?;
            validate_threshold("--min-duplicate-plugins", *min_duplicate_plugins, min_threshold)?;
            validate_threshold("--min-bundle-size", *min_bundle_size, min_threshold)?;
            validate_threshold("--min-bundle-modules", *min_bundle_modules, min_threshold)?;
            if let Some(value) = max_bundle_recommendations {
                if *value == 0 {
                    return Err(AnalysisError::Validation(
                        "--max-bundle-recommendations must be at least 1".to_string()
                    ));
                }
            }
        }
        Some(Commands::Conflicts { min_version_conflicts }) => {
            validate_threshold("--min-version-conflicts", *min_version_conflicts, min_threshold)?;
        }
        Some(Commands::Dependencies { min_duplicate_dependencies }) => {
            validate_threshold("--min-duplicate-dependencies", *min_duplicate_dependencies, min_threshold)?;
        }
        Some(Commands::Plugins { min_duplicate_plugins }) => {
            validate_threshold("--min-duplicate-plugins", *min_duplicate_plugins, min_threshold)?;
        }
        Some(Commands::Duplicates { min_duplicate_dependencies, min_duplicate_plugins }) => {
            validate_threshold("--min-duplicate-dependencies", *min_duplicate_dependencies, min_threshold)?;
            validate_threshold("--min-duplicate-plugins", *min_duplicate_plugins, min_threshold)?;
        }
        Some(Commands::Bundles { min_bundle_size, min_bundle_modules, max_bundle_recommendations }) => {
            validate_threshold("--min-bundle-size", *min_bundle_size, min_threshold)?;
            validate_threshold("--min-bundle-modules", *min_bundle_modules, min_threshold)?;
            if let Some(value) = max_bundle_recommendations {
                if *value == 0 {
                    return Err(AnalysisError::Validation(
                        "--max-bundle-recommendations must be at least 1".to_string()
                    ));
                }
            }
        }
        None => {
            // No validation needed for default behavior
        }
    }
    
    Ok(())
}

fn validate_threshold(arg_name: &str, value: Option<usize>, min_threshold: usize) -> Result<()> {
    if let Some(val) = value {
        if val < min_threshold {
            return Err(AnalysisError::Validation(format!(
                "{} must be at least {} (requires at least {} occurrences)",
                arg_name, min_threshold, min_threshold
            )));
        }
    }
    Ok(())
}