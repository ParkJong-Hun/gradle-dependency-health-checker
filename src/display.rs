use crate::parser::{DependencyLocation, DependencySourceType};
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
            
            let source_str = match &location.source_type {
                DependencySourceType::Direct => "",
                DependencySourceType::VersionCatalog(ref_name) => &format!(" [via libs.{}]", ref_name),
            };
                
            println!("  📍 {}:{} - {} configuration{}{}",
                location.file_path.display(),
                location.line_number,
                location.configuration,
                version_str,
                source_str.dimmed()
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
            
            let source_str = match &location.source_type {
                DependencySourceType::Direct => "",
                DependencySourceType::VersionCatalog(ref_name) => &format!(" [via libs.{}]", ref_name),
            };
                
            println!("  {} {}:{} - {} configuration{}{}",
                "⚠️".red(),
                location.file_path.display(),
                location.line_number,
                location.configuration,
                version_str,
                source_str.dimmed()
            );
        }
    }
}