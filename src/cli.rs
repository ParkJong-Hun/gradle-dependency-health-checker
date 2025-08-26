/*
 * Copyright (c) 2025 ParkJong-Hun
 *
 * Licensed under the MIT License.
 * See LICENSE file in the project root for full license information.
 */

use crate::config::Config;
use crate::error::{AnalysisError, Result};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "gradle-dependency-health-checker")]
#[command(about = "Check for duplicate dependencies and version conflicts in Gradle projects")]
pub struct Args {
    #[arg(short, long, default_value = ".")]
    pub path: PathBuf,
    
    #[arg(long, help = "Minimum number of version conflicts to display")]
    pub min_version_conflicts: Option<usize>,
    
    #[arg(long, help = "Minimum number of duplicate dependencies to display")]
    pub min_duplicate_dependencies: Option<usize>,
    
    #[arg(long, help = "Minimum number of dependencies required for a bundle recommendation")]
    pub min_bundle_size: Option<usize>,
    
    #[arg(long, help = "Minimum number of modules sharing dependencies for bundle recommendation")]
    pub min_bundle_modules: Option<usize>,
    
    #[arg(long, help = "Maximum number of bundle recommendations to display")]
    pub max_bundle_recommendations: Option<usize>,
}

impl Args {
    /// Apply defaults from config to None values
    pub fn with_defaults(mut self, config: &Config) -> Self {
        self.min_version_conflicts = Some(
            self.min_version_conflicts.unwrap_or(config.default_min_version_conflicts)
        );
        self.min_duplicate_dependencies = Some(
            self.min_duplicate_dependencies.unwrap_or(config.default_min_duplicate_dependencies)
        );
        self.min_bundle_size = Some(
            self.min_bundle_size.unwrap_or(config.default_min_bundle_size)
        );
        self.min_bundle_modules = Some(
            self.min_bundle_modules.unwrap_or(config.default_min_bundle_modules)
        );
        self.max_bundle_recommendations = Some(
            self.max_bundle_recommendations.unwrap_or(config.default_max_bundle_recommendations)
        );
        self
    }
    
    pub fn min_version_conflicts(&self) -> usize {
        self.min_version_conflicts.unwrap()
    }
    
    pub fn min_duplicate_dependencies(&self) -> usize {
        self.min_duplicate_dependencies.unwrap()
    }
    
    pub fn min_bundle_size(&self) -> usize {
        self.min_bundle_size.unwrap()
    }
    
    pub fn min_bundle_modules(&self) -> usize {
        self.min_bundle_modules.unwrap()
    }
    
    pub fn max_bundle_recommendations(&self) -> usize {
        self.max_bundle_recommendations.unwrap()
    }
}

pub fn validate_args(args: &Args, config: &Config) -> Result<()> {
    let min_threshold = config.min_threshold_value;
    
    if let Some(value) = args.min_version_conflicts {
        if value < min_threshold {
            return Err(AnalysisError::Validation(format!(
                "--min-version-conflicts must be at least {} (conflicts require at least {} occurrences)",
                min_threshold, min_threshold
            )));
        }
    }
    
    if let Some(value) = args.min_duplicate_dependencies {
        if value < min_threshold {
            return Err(AnalysisError::Validation(format!(
                "--min-duplicate-dependencies must be at least {} (duplicates require at least {} occurrences)",
                min_threshold, min_threshold
            )));
        }
    }
    
    if let Some(value) = args.min_bundle_size {
        if value < min_threshold {
            return Err(AnalysisError::Validation(format!(
                "--min-bundle-size must be at least {} (bundles require at least {} dependencies)",
                min_threshold, min_threshold
            )));
        }
    }
    
    if let Some(value) = args.min_bundle_modules {
        if value < min_threshold {
            return Err(AnalysisError::Validation(format!(
                "--min-bundle-modules must be at least {} (bundles require at least {} modules)",
                min_threshold, min_threshold
            )));
        }
    }
    
    if let Some(value) = args.max_bundle_recommendations {
        if value == 0 {
            return Err(AnalysisError::Validation(
                "--max-bundle-recommendations must be at least 1".to_string()
            ));
        }
    }
    
    Ok(())
}