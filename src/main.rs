mod cli;
mod parser;
mod analyzer;
mod display;

use clap::Parser;
use colored::*;
use cli::{Args, validate_args};
use analyzer::find_duplicate_dependencies;
use display::{print_version_conflicts, print_regular_duplicates};

fn main() {
    let args = Args::parse();
    
    // Validate threshold arguments
    if let Err(error_message) = validate_args(&args) {
        eprintln!("❌ Error: {}", error_message);
        std::process::exit(1);
    }
    
    match find_duplicate_dependencies(&args.path) {
        Ok(analysis) => {
            let version_conflicts_count = analysis.version_conflicts.len();
            let duplicate_dependencies_count = analysis.regular_duplicates.len();
            
            let show_version_conflicts = version_conflicts_count >= args.min_version_conflicts;
            let show_duplicate_dependencies = duplicate_dependencies_count >= args.min_duplicate_dependencies;
            
            if !show_version_conflicts && !show_duplicate_dependencies {
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
                    print_version_conflicts(&analysis.version_conflicts);
                }
                
                if show_duplicate_dependencies {
                    if show_version_conflicts {
                        println!();
                    }
                    println!("⚠️  Found {} duplicate dependencies:", duplicate_dependencies_count);
                    print_regular_duplicates(&analysis.regular_duplicates);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            std::process::exit(1);
        }
    }
}