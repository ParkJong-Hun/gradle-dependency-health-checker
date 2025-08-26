use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "gradle-dependency-health-checker")]
#[command(about = "Check for duplicate dependencies and version conflicts in Gradle projects")]
pub struct Args {
    #[arg(short, long, default_value = ".")]
    pub path: PathBuf,
    
    #[arg(long, default_value = "2", help = "Minimum number of version conflicts to display")]
    pub min_version_conflicts: usize,
    
    #[arg(long, default_value = "2", help = "Minimum number of duplicate dependencies to display")]
    pub min_duplicate_dependencies: usize,
    
    #[arg(long, default_value = "2", help = "Minimum number of dependencies required for a bundle recommendation")]
    pub min_bundle_size: usize,
    
    #[arg(long, default_value = "2", help = "Minimum number of modules sharing dependencies for bundle recommendation")]
    pub min_bundle_modules: usize,
    
    #[arg(long, default_value = "5", help = "Maximum number of bundle recommendations to display")]
    pub max_bundle_recommendations: usize,
}

pub fn validate_args(args: &Args) -> Result<(), String> {
    if args.min_version_conflicts < 2 {
        return Err("--min-version-conflicts must be at least 2 (conflicts require at least 2 occurrences)".to_string());
    }
    
    if args.min_duplicate_dependencies < 2 {
        return Err("--min-duplicate-dependencies must be at least 2 (duplicates require at least 2 occurrences)".to_string());
    }
    
    if args.min_bundle_size < 2 {
        return Err("--min-bundle-size must be at least 2 (bundles require at least 2 dependencies)".to_string());
    }
    
    if args.min_bundle_modules < 2 {
        return Err("--min-bundle-modules must be at least 2 (bundles require at least 2 modules)".to_string());
    }
    
    if args.max_bundle_recommendations == 0 {
        return Err("--max-bundle-recommendations must be at least 1".to_string());
    }
    
    Ok(())
}