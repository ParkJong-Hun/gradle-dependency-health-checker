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
mod loading;

use clap::Parser;
use colored::*;
use cli::{Args, validate_args, AnalysisOptions};
use config::Config;
use analyzer::{perform_complete_analysis, CompleteAnalysis};
use display::{print_version_conflicts, print_regular_duplicates, print_bundle_recommendations, print_duplicate_plugins};
use loading::{ProgressBar, LoadingSpinner};
use std::fs;
use std::io::Write;

fn main() {
    let args = Args::parse();
    let config = Config::default();
    
    // Validate threshold arguments
    if let Err(error) = validate_args(&args, &config) {
        if !args.silent {
            eprintln!("❌ Error: {}", error);
        }
        std::process::exit(1);
    }
    
    let options = args.get_analysis_options(&config);
    
    // Only show loading animation if not in silent mode
    let analysis_result = if args.silent {
        perform_complete_analysis(&args.path, options.min_bundle_size, options.min_bundle_modules)
    } else {
        let mut progress = ProgressBar::new("Analyzing Gradle project dependencies");
        let result = perform_complete_analysis(&args.path, options.min_bundle_size, options.min_bundle_modules);
        match &result {
            Ok(_) => progress.finish_with_message("✅ Analysis completed successfully"),
            Err(_) => progress.finish(),
        }
        result
    };
    
    match analysis_result {
        Ok(analysis) => {
            // Handle output based on whether file output is requested
            if let Some(output_path) = &args.output {
                let write_result = if args.silent {
                    write_analysis_to_file(&analysis, output_path)
                } else {
                    let mut spinner = LoadingSpinner::new("Writing results to file");
                    let result = write_analysis_to_file(&analysis, output_path);
                    match &result {
                        Ok(_) => spinner.finish_with_message(&format!("✅ Analysis results written to: {}", output_path.display())),
                        Err(_) => spinner.finish(),
                    }
                    result
                };
                
                if let Err(e) = write_result {
                    if !args.silent {
                        eprintln!("❌ Error writing to file: {}", e);
                    }
                    std::process::exit(1);
                }
            } else {
                // In silent mode without output file, don't print anything
                if !args.silent {
                    print_analysis_to_console(&analysis, &options);
                }
            }
        }
        Err(e) => {
            if !args.silent {
                eprintln!("❌ Error: {}", e);
            }
            std::process::exit(1);
        }
    }
}

fn write_analysis_to_file(analysis: &CompleteAnalysis, output_path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let json_output = serde_json::to_string_pretty(analysis)?;
    let mut file = fs::File::create(output_path)?;
    file.write_all(json_output.as_bytes())?;
    Ok(())
}

fn print_analysis_to_console(analysis: &CompleteAnalysis, options: &AnalysisOptions) {
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