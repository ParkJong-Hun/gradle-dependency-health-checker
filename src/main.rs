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
use cli::{Args, validate_args};
use config::Config;
use analyzer::perform_complete_analysis;
use display::{print_version_conflicts, print_regular_duplicates, print_bundle_recommendations};

fn main() {
    let args = Args::parse();
    let config = Config::default();
    let args = args.with_defaults(&config);
    
    // Validate threshold arguments
    if let Err(error) = validate_args(&args, &config) {
        eprintln!("❌ Error: {}", error);
        std::process::exit(1);
    }
    
    match perform_complete_analysis(&args.path, args.min_bundle_size(), args.min_bundle_modules()) {
        Ok(analysis) => {
            let version_conflicts_count = analysis.duplicate_analysis.version_conflicts.len();
            let duplicate_dependencies_count = analysis.duplicate_analysis.regular_duplicates.len();
            let bundle_recommendations_count = analysis.bundle_analysis.recommended_bundles.len();
            
            let show_version_conflicts = version_conflicts_count >= args.min_version_conflicts();
            let show_duplicate_dependencies = duplicate_dependencies_count >= args.min_duplicate_dependencies();
            let show_bundle_recommendations = bundle_recommendations_count > 0;
            
            if !show_version_conflicts && !show_duplicate_dependencies && !show_bundle_recommendations {
                println!("✅ No issues found above the specified thresholds.");
                if version_conflicts_count > 0 || duplicate_dependencies_count > 0 {
                    println!("   (Found {} version conflicts and {} duplicate dependencies below thresholds)", 
                        version_conflicts_count, duplicate_dependencies_count);
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
                
                if show_bundle_recommendations {
                    print_bundle_recommendations(&analysis.bundle_analysis, args.max_bundle_recommendations());
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            std::process::exit(1);
        }
    }
}