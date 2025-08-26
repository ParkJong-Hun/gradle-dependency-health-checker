use crate::parser::DependencyLocation;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct DependencyBundle {
    pub dependencies: Vec<String>, // group:artifact format
    pub modules: Vec<PathBuf>,
    pub bundle_size: usize,
    pub module_count: usize,
    pub configurations: HashSet<String>, // implementation, api, testImplementation, etc.
    pub priority_score: f64,
}

#[derive(Debug)]
pub struct BundleAnalysis {
    pub recommended_bundles: Vec<DependencyBundle>,
    pub total_bundles_found: usize,
}

pub fn find_dependency_bundles(
    dependencies: &[DependencyLocation],
    min_bundle_size: usize,
    min_module_count: usize,
) -> BundleAnalysis {
    // Group dependencies by module (file)
    let mut module_dependencies: HashMap<PathBuf, Vec<&DependencyLocation>> = HashMap::new();
    
    for dep in dependencies {
        module_dependencies
            .entry(dep.file_path.clone())
            .or_default()
            .push(dep);
    }
    
    // Convert to module -> set of dependencies for easier comparison
    let module_dep_sets: HashMap<PathBuf, HashSet<String>> = module_dependencies
        .into_iter()
        .map(|(module, deps)| {
            let dep_set = deps
                .into_iter()
                .map(|d| format!("{}:{}", d.dependency.group, d.dependency.artifact))
                .collect();
            (module, dep_set)
        })
        .collect();
    
    // Find all possible dependency combinations that appear in multiple modules
    let mut bundle_candidates: HashMap<Vec<String>, Vec<PathBuf>> = HashMap::new();
    
    // Get all modules as a vector for easier iteration
    let modules: Vec<_> = module_dep_sets.keys().cloned().collect();
    
    // For each pair of modules, find common dependencies
    for i in 0..modules.len() {
        for j in i + 1..modules.len() {
            let module1 = &modules[i];
            let module2 = &modules[j];
            
            let deps1 = &module_dep_sets[module1];
            let deps2 = &module_dep_sets[module2];
            
            let common: HashSet<_> = deps1.intersection(deps2).cloned().collect();
            
            if common.len() >= min_bundle_size {
                // Generate all subsets of common dependencies that meet min_bundle_size
                let common_vec: Vec<String> = common.into_iter().collect();
                let subsets = generate_subsets(&common_vec, min_bundle_size);
                
                for subset in subsets {
                    let mut sorted_subset = subset;
                    sorted_subset.sort(); // Ensure consistent ordering
                    
                    // Find all modules that have this exact subset
                    let mut modules_with_subset = Vec::new();
                    for (module, deps) in &module_dep_sets {
                        if sorted_subset.iter().all(|dep| deps.contains(dep)) {
                            modules_with_subset.push(module.clone());
                        }
                    }
                    
                    if modules_with_subset.len() >= min_module_count {
                        bundle_candidates.insert(sorted_subset, modules_with_subset);
                    }
                }
            }
        }
    }
    
    // Convert candidates to DependencyBundle and calculate priority scores
    let bundles: Vec<DependencyBundle> = bundle_candidates
        .into_iter()
        .map(|(deps, modules)| {
            let configurations = get_configurations_for_bundle(&deps, dependencies);
            let priority_score = calculate_priority_score(&deps, &modules, &configurations);
            
            DependencyBundle {
                bundle_size: deps.len(),
                module_count: modules.len(),
                dependencies: deps,
                modules,
                configurations,
                priority_score,
            }
        })
        .collect();
    
    // Remove subsets of larger bundles (if bundle A is subset of B and they share same modules, remove A)
    let filtered_bundles = remove_subset_bundles(bundles);
    
    // Sort by priority score (highest first)
    let mut sorted_bundles = filtered_bundles;
    sorted_bundles.sort_by(|a, b| b.priority_score.partial_cmp(&a.priority_score).unwrap());
    
    BundleAnalysis {
        total_bundles_found: sorted_bundles.len(),
        recommended_bundles: sorted_bundles,
    }
}

fn generate_subsets(items: &[String], min_size: usize) -> Vec<Vec<String>> {
    let mut subsets = Vec::new();
    let n = items.len();
    
    // Generate all possible subsets using bit manipulation
    for mask in 1..(1 << n) {
        let mut subset = Vec::new();
        for i in 0..n {
            if mask & (1 << i) != 0 {
                subset.push(items[i].clone());
            }
        }
        if subset.len() >= min_size {
            subsets.push(subset);
        }
    }
    
    subsets
}

fn get_configurations_for_bundle(bundle_deps: &[String], all_dependencies: &[DependencyLocation]) -> HashSet<String> {
    let mut configurations = HashSet::new();
    
    for dep_location in all_dependencies {
        let dep_key = format!("{}:{}", dep_location.dependency.group, dep_location.dependency.artifact);
        if bundle_deps.contains(&dep_key) {
            configurations.insert(dep_location.configuration.clone());
        }
    }
    
    configurations
}

fn calculate_priority_score(
    dependencies: &[String],
    modules: &[PathBuf],
    configurations: &HashSet<String>,
) -> f64 {
    let bundle_size_score = dependencies.len() as f64 * 10.0; // Higher weight for bundle size
    let module_count_score = modules.len() as f64 * 5.0;      // Medium weight for module count
    
    // Configuration type score (implementation/api higher than test)
    let config_score = configurations.iter().map(|config| {
        match config.as_str() {
            "api" => 3.0,
            "implementation" => 2.5,
            "compileOnly" => 2.0,
            "runtimeOnly" => 1.5,
            "testImplementation" => 1.0,
            "testCompileOnly" => 0.5,
            _ => 1.0,
        }
    }).sum::<f64>();
    
    bundle_size_score + module_count_score + config_score
}

fn remove_subset_bundles(mut bundles: Vec<DependencyBundle>) -> Vec<DependencyBundle> {
    bundles.sort_by_key(|b| std::cmp::Reverse(b.bundle_size)); // Largest first
    
    let mut filtered = Vec::new();
    
    for bundle in bundles {
        let is_subset = filtered.iter().any(|existing: &DependencyBundle| {
            // Check if current bundle is a subset of an existing one with same modules
            bundle.modules == existing.modules &&
            bundle.dependencies.iter().all(|dep| existing.dependencies.contains(dep)) &&
            bundle.dependencies.len() < existing.dependencies.len()
        });
        
        if !is_subset {
            filtered.push(bundle);
        }
    }
    
    filtered
}