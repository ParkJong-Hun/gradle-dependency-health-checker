use clap::Parser;
use colored::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "gradle-dependency-health-checker")]
#[command(about = "Check for duplicate dependencies and version conflicts in Gradle projects")]
struct Args {
    #[arg(short, long, default_value = ".")]
    path: PathBuf,
    
    #[arg(long, default_value = "2", help = "Minimum number of version conflicts to display")]
    min_version_conflicts: usize,
    
    #[arg(long, default_value = "2", help = "Minimum number of duplicate dependencies to display")]
    min_duplicate_dependencies: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Dependency {
    group: String,
    artifact: String,
    version: Option<String>,
}

#[derive(Debug)]
struct DependencyLocation {
    dependency: Dependency,
    file_path: PathBuf,
    line_number: usize,
    configuration: String,
}

fn find_gradle_files(root_path: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut gradle_files = Vec::new();
    
    for entry in WalkDir::new(root_path) {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                if filename == "build.gradle" || filename == "build.gradle.kts" {
                    gradle_files.push(path.to_path_buf());
                }
            }
        }
    }
    
    Ok(gradle_files)
}

fn parse_dependencies_from_file(file_path: &Path) -> Result<Vec<DependencyLocation>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let mut dependencies = Vec::new();
    let mut in_dependencies_block = false;
    let mut brace_count = 0;
    
    // Regex patterns for different dependency formats
    let string_dep_regex = Regex::new(r#"^\s*(\w+)\s*[\(\s]?\s*["']([^"':]+):([^"':]+):([^"']+)["']\s*[\)\s]?.*$"#)?;
    let map_dep_regex = Regex::new(r#"^\s*(\w+)\s*\(\s*group\s*:\s*["']([^"']+)["']\s*,\s*name\s*:\s*["']([^"']+)["']\s*,\s*version\s*:\s*["']([^"']+)["']\s*\).*$"#)?;
    let map_dep_regex2 = Regex::new(r#"^\s*(\w+)\s*\(\s*name\s*:\s*["']([^"']+)["']\s*,\s*group\s*:\s*["']([^"']+)["']\s*,\s*version\s*:\s*["']([^"']+)["']\s*\).*$"#)?;
    
    for (line_number, line) in content.lines().enumerate() {
        let trimmed_line = line.trim();
        
        // Check if we're entering a dependencies block
        if trimmed_line.starts_with("dependencies") && trimmed_line.contains('{') {
            in_dependencies_block = true;
            brace_count = 1;
            continue;
        }
        
        if in_dependencies_block {
            // Count braces to track nested blocks
            brace_count += trimmed_line.matches('{').count();
            brace_count -= trimmed_line.matches('}').count();
            
            if brace_count == 0 {
                in_dependencies_block = false;
                continue;
            }
            
            // Parse different dependency formats
            if let Some(captures) = string_dep_regex.captures(trimmed_line) {
                let configuration = captures[1].to_string();
                let group = captures[2].to_string();
                let artifact = captures[3].to_string();
                let version = Some(captures[4].to_string());
                
                dependencies.push(DependencyLocation {
                    dependency: Dependency { group, artifact, version },
                    file_path: file_path.to_path_buf(),
                    line_number: line_number + 1,
                    configuration,
                });
            } else if let Some(captures) = map_dep_regex.captures(trimmed_line) {
                let configuration = captures[1].to_string();
                let group = captures[2].to_string();
                let artifact = captures[3].to_string();
                let version = Some(captures[4].to_string());
                
                dependencies.push(DependencyLocation {
                    dependency: Dependency { group, artifact, version },
                    file_path: file_path.to_path_buf(),
                    line_number: line_number + 1,
                    configuration,
                });
            } else if let Some(captures) = map_dep_regex2.captures(trimmed_line) {
                let configuration = captures[1].to_string();
                let artifact = captures[2].to_string();
                let group = captures[3].to_string();
                let version = Some(captures[4].to_string());
                
                dependencies.push(DependencyLocation {
                    dependency: Dependency { group, artifact, version },
                    file_path: file_path.to_path_buf(),
                    line_number: line_number + 1,
                    configuration,
                });
            }
        }
    }
    
    Ok(dependencies)
}

#[derive(Debug)]
struct DuplicateAnalysis {
    regular_duplicates: HashMap<String, Vec<DependencyLocation>>,
    version_conflicts: HashMap<String, Vec<DependencyLocation>>,
}

fn find_duplicate_dependencies(root_path: &Path) -> Result<DuplicateAnalysis, Box<dyn std::error::Error>> {
    let gradle_files = find_gradle_files(root_path)?;
    let mut all_dependencies = Vec::new();
    
    for gradle_file in gradle_files {
        let mut deps = parse_dependencies_from_file(&gradle_file)?;
        all_dependencies.append(&mut deps);
    }
    
    // Group dependencies by group:artifact (ignoring version)
    let mut dependency_groups: HashMap<String, Vec<DependencyLocation>> = HashMap::new();
    
    for dep_location in all_dependencies {
        let key = format!("{}:{}", dep_location.dependency.group, dep_location.dependency.artifact);
        dependency_groups.entry(key).or_default().push(dep_location);
    }
    
    // Separate regular duplicates from version conflicts
    let mut regular_duplicates = HashMap::new();
    let mut version_conflicts = HashMap::new();
    
    for (key, locations) in dependency_groups {
        if locations.len() > 1 {
            let mut unique_files = HashSet::new();
            let mut unique_versions = HashSet::new();
            
            for location in &locations {
                unique_files.insert(&location.file_path);
                if let Some(version) = &location.dependency.version {
                    unique_versions.insert(version);
                }
            }
            
            // Only consider it if it appears in different files
            if unique_files.len() > 1 {
                if unique_versions.len() > 1 {
                    // Same dependency, different versions = version conflict
                    version_conflicts.insert(key, locations);
                } else {
                    // Same dependency, same version = regular duplicate
                    regular_duplicates.insert(key, locations);
                }
            }
        }
    }
    
    Ok(DuplicateAnalysis {
        regular_duplicates,
        version_conflicts,
    })
}

fn print_regular_duplicates(duplicates: &HashMap<String, Vec<DependencyLocation>>) {
    for (dependency_key, locations) in duplicates {
        println!("\n📦 Dependency: {}", dependency_key);
        
        for location in locations {
            let version_str = location.dependency.version
                .as_ref()
                .map(|v| format!(" (version: {})", v.bold()))
                .unwrap_or_default();
                
            println!("  📍 {}:{} - {} configuration{}",
                location.file_path.display(),
                location.line_number,
                location.configuration,
                version_str
            );
        }
    }
}

fn print_version_conflicts(conflicts: &HashMap<String, Vec<DependencyLocation>>) {
    for (dependency_key, locations) in conflicts {
        println!("\n{} {}", "🚨".red(), format!("Dependency: {}", dependency_key).red().bold());
        
        for location in locations {
            let version_str = location.dependency.version
                .as_ref()
                .map(|v| format!(" (version: {})", v.red().bold()).to_string())
                .unwrap_or_default();
                
            println!("  {} {}:{} - {} configuration{}",
                "⚠️".red(),
                location.file_path.display(),
                location.line_number,
                location.configuration,
                version_str
            );
        }
    }
}

fn main() {
    let args = Args::parse();
    
    // Validate threshold arguments
    if args.min_version_conflicts < 2 {
        eprintln!("❌ Error: --min-version-conflicts must be at least 2 (conflicts require at least 2 occurrences)");
        std::process::exit(1);
    }
    
    if args.min_duplicate_dependencies < 2 {
        eprintln!("❌ Error: --min-duplicate-dependencies must be at least 2 (duplicates require at least 2 occurrences)");
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
