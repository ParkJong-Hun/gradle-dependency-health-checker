/*
 * Copyright (c) 2025 ParkJong-Hun
 *
 * Licensed under the MIT License.
 * See LICENSE file in the project root for full license information.
 */

use crate::config::BundleNamePatterns;
use crate::parser::{DependencyLocation, DependencySourceType};
use crate::bundle_analyzer::{DependencyBundle, BundleAnalysis};
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

pub fn print_bundle_recommendations(analysis: &BundleAnalysis, max_recommendations: usize) {
    if analysis.recommended_bundles.is_empty() {
        return;
    }
    
    let bundles_to_show = analysis.recommended_bundles.len().min(max_recommendations);
    
    println!("\n{} {} {}:",
        "💡".yellow(),
        "Bundle recommendations".yellow().bold(),
        format!("(showing {} of {})", bundles_to_show, analysis.total_bundles_found).dimmed()
    );
    
    for (index, bundle) in analysis.recommended_bundles.iter().take(bundles_to_show).enumerate() {
        print_bundle_recommendation(bundle, index + 1);
    }
}

fn print_bundle_recommendation(bundle: &DependencyBundle, rank: usize) {
    println!("\n{} {}. {} ({} dependencies × {} modules)",
        "📎".cyan(),
        rank,
        "Recommended Bundle".cyan().bold(),
        bundle.bundle_size.to_string().bright_cyan(),
        bundle.module_count.to_string().bright_cyan()
    );
    
    // Show dependencies
    println!("   {}", "Dependencies:".bright_white());
    for (i, dep) in bundle.dependencies.iter().enumerate() {
        let prefix = if i == bundle.dependencies.len() - 1 { "└─" } else { "├─" };
        println!("     {} {}", prefix.dimmed(), dep);
    }
    
    // Show configurations
    if !bundle.configurations.is_empty() {
        let mut configs: Vec<_> = bundle.configurations.iter().collect();
        configs.sort();
        let configs_str: Vec<&str> = configs.iter().map(|s| s.as_str()).collect();
        println!("   {}: {}", 
            "Configurations".bright_white(), 
            configs_str.join(", ").dimmed()
        );
    }
    
    // Show affected modules
    println!("   {}", "Used by modules:".bright_white());
    for (i, module) in bundle.modules.iter().enumerate() {
        let prefix = if i == bundle.modules.len() - 1 { "└─" } else { "├─" };
        let module_name = module.file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("unknown");
        let parent_dir = module.parent()
            .and_then(|p| p.file_name())
            .and_then(|f| f.to_str())
            .unwrap_or("");
        
        if parent_dir.is_empty() {
            println!("     {} {}", prefix.dimmed(), module_name);
        } else {
            println!("     {} {}/{}", prefix.dimmed(), parent_dir, module_name);
        }
    }
    
    // Show recommendation
    let bundle_name_patterns = BundleNamePatterns::default();
    let bundle_name = generate_bundle_name(&bundle.dependencies, &bundle_name_patterns);
    println!("   {} Consider creating a shared module: {}", 
        "💭".bright_blue(), 
        bundle_name.bright_green()
    );
}

fn generate_bundle_name(dependencies: &[String], patterns: &BundleNamePatterns) -> String {
    let most_common_group = find_most_common_group(dependencies);
    patterns.find_bundle_name(&most_common_group)
}

fn find_most_common_group(dependencies: &[String]) -> String {
    let common_groups: HashMap<String, usize> = dependencies
        .iter()
        .map(|dep| dep.split(':').next().unwrap_or("unknown").to_string())
        .fold(HashMap::new(), |mut acc, group| {
            *acc.entry(group).or_insert(0) += 1;
            acc
        });
    
    common_groups
        .iter()
        .max_by_key(|(_, count)| *count)
        .map(|(group, _)| group.clone())
        .unwrap_or_else(|| "common".to_string())
}