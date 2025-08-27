/*
 * Copyright (c) 2025 ParkJong-Hun
 *
 * Licensed under the MIT License.
 * See LICENSE file in the project root for full license information.
 */

use crate::error::{Result};
use crate::parser::{DependencyLocation, PluginLocation, find_gradle_files, parse_dependencies_from_file, parse_plugins_from_file, load_version_catalogs};
use crate::bundle_analyzer::{find_dependency_bundles, BundleAnalysis};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct DuplicateAnalysis {
    pub regular_duplicates: HashMap<String, Vec<DependencyLocation>>,
    pub version_conflicts: HashMap<String, Vec<DependencyLocation>>,
}

#[derive(Debug, Serialize)]
pub struct PluginAnalysis {
    pub duplicate_plugins: HashMap<String, Vec<PluginLocation>>,
}

#[derive(Debug, Serialize)]
pub struct CompleteAnalysis {
    pub duplicate_analysis: DuplicateAnalysis,
    pub plugin_analysis: PluginAnalysis,
    pub bundle_analysis: BundleAnalysis,
}

pub fn perform_complete_analysis(
    root_path: &Path,
    min_bundle_size: usize,
    min_bundle_modules: usize,
) -> Result<CompleteAnalysis> {
    let all_dependencies = load_all_dependencies(root_path)?;
    let all_plugins = load_all_plugins(root_path)?;
    
    // Perform duplicate analysis
    let duplicate_analysis = analyze_duplicates(&all_dependencies);
    
    // Perform plugin analysis
    let plugin_analysis = analyze_plugins(&all_plugins);
    
    // Perform bundle analysis
    let bundle_analysis = find_dependency_bundles(&all_dependencies, min_bundle_size, min_bundle_modules);
    
    Ok(CompleteAnalysis {
        duplicate_analysis,
        plugin_analysis,
        bundle_analysis,
    })
}

fn load_all_dependencies(root_path: &Path) -> Result<Vec<DependencyLocation>> {
    let gradle_files = find_gradle_files(root_path)?;
    let version_catalogs = load_version_catalogs(root_path)?;
    let mut all_dependencies = Vec::new();
    
    for gradle_file in gradle_files {
        let mut deps = parse_dependencies_from_file(&gradle_file, &version_catalogs)?;
        all_dependencies.append(&mut deps);
    }
    
    Ok(all_dependencies)
}

fn load_all_plugins(root_path: &Path) -> Result<Vec<PluginLocation>> {
    let gradle_files = find_gradle_files(root_path)?;
    let version_catalogs = load_version_catalogs(root_path)?;
    let mut all_plugins = Vec::new();
    
    for gradle_file in gradle_files {
        let mut plugins = parse_plugins_from_file(&gradle_file, &version_catalogs)?;
        all_plugins.append(&mut plugins);
    }
    
    Ok(all_plugins)
}

fn create_dependency_key(group: &str, artifact: &str) -> String {
    format!("{}:{}", group, artifact)
}

fn analyze_duplicates(all_dependencies: &[DependencyLocation]) -> DuplicateAnalysis {
    // Group dependencies by group:artifact (ignoring version)
    let mut dependency_groups: HashMap<String, Vec<&DependencyLocation>> = HashMap::new();
    
    for dep_location in all_dependencies {
        let key = create_dependency_key(&dep_location.dependency.group, &dep_location.dependency.artifact);
        dependency_groups.entry(key).or_default().push(dep_location);
    }
    
    let (regular_duplicates, version_conflicts) = process_dependency_groups(dependency_groups);
    
    DuplicateAnalysis {
        regular_duplicates,
        version_conflicts,
    }
}

fn process_dependency_groups(
    dependency_groups: HashMap<String, Vec<&DependencyLocation>>
) -> (HashMap<String, Vec<DependencyLocation>>, HashMap<String, Vec<DependencyLocation>>) {
    let mut regular_duplicates = HashMap::new();
    let mut version_conflicts = HashMap::new();
    
    for (key, locations) in dependency_groups {
        if let Some((is_version_conflict, owned_locations)) = analyze_dependency_group(&locations) {
            if is_version_conflict {
                version_conflicts.insert(key, owned_locations);
            } else {
                regular_duplicates.insert(key, owned_locations);
            }
        }
    }
    
    (regular_duplicates, version_conflicts)
}

fn analyze_dependency_group(locations: &[&DependencyLocation]) -> Option<(bool, Vec<DependencyLocation>)> {
    if locations.len() <= 1 {
        return None;
    }
    
    let mut unique_files = HashSet::new();
    let mut unique_versions = HashSet::new();
    
    for location in locations {
        unique_files.insert(&location.file_path);
        if let Some(version) = &location.dependency.version {
            unique_versions.insert(version);
        }
    }
    
    // Only consider it if it appears in different files
    if unique_files.len() > 1 {
        let locations_owned: Vec<DependencyLocation> = locations.iter().map(|&loc| loc.clone()).collect();
        let is_version_conflict = unique_versions.len() > 1;
        Some((is_version_conflict, locations_owned))
    } else {
        None
    }
}

fn analyze_plugins(all_plugins: &[PluginLocation]) -> PluginAnalysis {
    let mut plugin_groups: HashMap<String, Vec<&PluginLocation>> = HashMap::new();
    
    // Group plugins by ID
    for plugin_location in all_plugins {
        let key = plugin_location.plugin.id.clone();
        plugin_groups.entry(key).or_default().push(plugin_location);
    }
    
    let mut duplicate_plugins = HashMap::new();
    
    for (plugin_id, locations) in plugin_groups {
        if locations.len() > 1 {
            // Check if plugins appear in different files
            let mut unique_files = HashSet::new();
            for location in &locations {
                unique_files.insert(&location.file_path);
            }
            
            if unique_files.len() > 1 {
                let locations_owned: Vec<PluginLocation> = locations.iter().map(|&loc| loc.clone()).collect();
                duplicate_plugins.insert(plugin_id, locations_owned);
            }
        }
    }
    
    PluginAnalysis {
        duplicate_plugins,
    }
}