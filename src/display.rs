use crate::parser::DependencyLocation;
use colored::*;
use std::collections::HashMap;

pub fn print_regular_duplicates(duplicates: &HashMap<String, Vec<DependencyLocation>>) {
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

pub fn print_version_conflicts(conflicts: &HashMap<String, Vec<DependencyLocation>>) {
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