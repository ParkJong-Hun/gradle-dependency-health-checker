use crate::parser::{DependencyLocation, find_gradle_files, parse_dependencies_from_file, load_version_catalogs};
use std::collections::{HashMap, HashSet};
use std::path::Path;

#[derive(Debug)]
pub struct DuplicateAnalysis {
    pub regular_duplicates: HashMap<String, Vec<DependencyLocation>>,
    pub version_conflicts: HashMap<String, Vec<DependencyLocation>>,
}

pub fn find_duplicate_dependencies(root_path: &Path) -> Result<DuplicateAnalysis, Box<dyn std::error::Error>> {
    let gradle_files = find_gradle_files(root_path)?;
    let version_catalogs = load_version_catalogs(root_path)?;
    let mut all_dependencies = Vec::new();
    
    for gradle_file in gradle_files {
        let mut deps = parse_dependencies_from_file(&gradle_file, &version_catalogs)?;
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