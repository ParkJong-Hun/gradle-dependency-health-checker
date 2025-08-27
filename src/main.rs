/*
 * Copyright (c) 2025 ParkJong-Hun
 *
 * Licensed under the MIT License.
 * See LICENSE file in the project root for full license information.
 */

mod cli;
mod parser;
mod analyzer;
mod display;
mod version_catalog;
mod bundle_analyzer;
mod config;
mod error;

use clap::Parser;
use colored::*;
use cli::{Args, validate_args, AnalysisOptions};
use config::Config;
use analyzer::perform_complete_analysis;
use display::{print_version_conflicts, print_regular_duplicates, print_bundle_recommendations, print_duplicate_plugins};

fn main() {
    let args = Args::parse();
    let config = Config::default();
    
    // Validate threshold arguments
    if let Err(error) = validate_args(&args, &config) {
        eprintln!("❌ Error: {}", error);
        std::process::exit(1);
    }
    
    let options = args.get_analysis_options(&config);
    
    match perform_complete_analysis(&args.path, options.min_bundle_size, options.min_bundle_modules) {
        Ok(analysis) => {
            let version_conflicts_count = analysis.duplicate_analysis.version_conflicts.len();
            let duplicate_dependencies_count = analysis.duplicate_analysis.regular_duplicates.len();
            let duplicate_plugins_count = analysis.plugin_analysis.duplicate_plugins
                .iter()
                .map(|(_, locations)| locations.len())
                .sum::<usize>();
            let bundle_recommendations_count = analysis.bundle_analysis.recommended_bundles.len();
            
            let show_version_conflicts = version_conflicts_count >= options.min_version_conflicts;
            let show_duplicate_dependencies = duplicate_dependencies_count >= options.min_duplicate_dependencies;
            let show_duplicate_plugins = duplicate_plugins_count >= options.min_duplicate_plugins;
            let show_bundle_recommendations = bundle_recommendations_count > 0 && options.max_bundle_recommendations > 0;
            
            if !show_version_conflicts && !show_duplicate_dependencies && !show_duplicate_plugins && !show_bundle_recommendations {
                println!("✅ No issues found above the specified thresholds.");
                if version_conflicts_count > 0 || duplicate_dependencies_count > 0 || duplicate_plugins_count > 0 {
                    println!("   (Found {} version conflicts, {} duplicate dependencies, and {} duplicate plugins below thresholds)", 
                        version_conflicts_count, duplicate_dependencies_count, duplicate_plugins_count);
                }
            } else {
                if show_version_conflicts {
                    println!("{} {} {}:",
                        "🚨".red(),
                        "Found".red().bold(),
                        format!("{} version conflicts", version_conflicts_count).red().bold()
                    );
                    print_version_conflicts(&analysis.duplicate_analysis.version_conflicts);
                }
                
                if show_duplicate_dependencies {
                    if show_version_conflicts {
                        println!();
                    }
                    println!("⚠️  Found {} duplicate dependencies:", duplicate_dependencies_count);
                    print_regular_duplicates(&analysis.duplicate_analysis.regular_duplicates);
                }
                
                if show_duplicate_plugins {
                    if show_version_conflicts || show_duplicate_dependencies {
                        println!();
                    }
                    println!("🔌 Found {} duplicate plugins:", duplicate_plugins_count);
                    print_duplicate_plugins(&analysis.plugin_analysis.duplicate_plugins);
                }
                
                if show_bundle_recommendations {
                    print_bundle_recommendations(&analysis.bundle_analysis, options.max_bundle_recommendations);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            std::process::exit(1);
        }
    }
}