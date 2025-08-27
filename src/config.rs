/*
 * Copyright (c) 2025 ParkJong-Hun
 *
 * Licensed under the MIT License.
 * See LICENSE file in the project root for full license information.
 */

/// Application configuration constants
pub struct Config {
    pub default_min_version_conflicts: usize,
    pub default_min_duplicate_dependencies: usize,
    pub default_min_duplicate_plugins: usize,
    pub default_min_bundle_size: usize,
    pub default_min_bundle_modules: usize,
    pub default_max_bundle_recommendations: usize,
    pub min_threshold_value: usize,
    pub priority_weights: PriorityWeights,
    pub configuration_scores: ConfigurationScores,
}

pub struct PriorityWeights {
    pub bundle_size: f64,
    pub module_count: f64,
}

pub struct ConfigurationScores {
    pub api: f64,
    pub implementation: f64,
    pub compile_only: f64,
    pub runtime_only: f64,
    pub test_implementation: f64,
    pub test_compile_only: f64,
    pub default: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_min_version_conflicts: 2,
            default_min_duplicate_dependencies: 2,
            default_min_duplicate_plugins: 2,
            default_min_bundle_size: 2,
            default_min_bundle_modules: 2,
            default_max_bundle_recommendations: 5,
            min_threshold_value: 2,
            priority_weights: PriorityWeights {
                bundle_size: 10.0,
                module_count: 5.0,
            },
            configuration_scores: ConfigurationScores {
                api: 3.0,
                implementation: 2.5,
                compile_only: 2.0,
                runtime_only: 1.5,
                test_implementation: 1.0,
                test_compile_only: 0.5,
                default: 1.0,
            },
        }
    }
}

/// Bundle name patterns for common dependency groups
pub struct BundleNamePatterns {
    patterns: Vec<(String, String)>, // (pattern, suggested_name)
}

impl Default for BundleNamePatterns {
    fn default() -> Self {
        Self {
            patterns: vec![
                ("androidx".to_string(), "androidx-bundle".to_string()),
                ("kotlin".to_string(), "kotlin-bundle".to_string()),
                ("jetbrains".to_string(), "kotlin-bundle".to_string()),
                ("test".to_string(), "testing-bundle".to_string()),
                ("junit".to_string(), "testing-bundle".to_string()),
                ("retrofit".to_string(), "networking-bundle".to_string()),
                ("okhttp".to_string(), "networking-bundle".to_string()),
                ("jackson".to_string(), "json-bundle".to_string()),
                ("gson".to_string(), "json-bundle".to_string()),
            ],
        }
    }
}

impl BundleNamePatterns {
    pub fn find_bundle_name(&self, most_common_group: &str) -> String {
        for (pattern, name) in &self.patterns {
            if most_common_group.contains(pattern) {
                return name.clone();
            }
        }
        
        // Fallback to generic name based on last part of group
        format!("{}-bundle", most_common_group.split('.').last().unwrap_or("common"))
    }
}

/// File patterns for Gradle projects
pub mod file_patterns {
    pub const GRADLE_BUILD_FILES: &[&str] = &["build.gradle", "build.gradle.kts"];
    pub const VERSION_CATALOG_FILES: &[&str] = &["libs.versions.toml", "versions.toml"];
}

/// Regex patterns for dependency parsing
pub mod regex_patterns {
    pub const STRING_DEPENDENCY: &str = r#"^\s*(\w+)\s*[\(\s]?\s*["']([^"':]+):([^"':]+):([^"']+)["']\s*[\)\s]?.*$"#;
    pub const MAP_DEPENDENCY_1: &str = r#"^\s*(\w+)\s*\(\s*group\s*:\s*["']([^"']+)["']\s*,\s*name\s*:\s*["']([^"']+)["']\s*,\s*version\s*:\s*["']([^"']+)["']\s*\).*$"#;
    pub const MAP_DEPENDENCY_2: &str = r#"^\s*(\w+)\s*\(\s*name\s*:\s*["']([^"']+)["']\s*,\s*group\s*:\s*["']([^"']+)["']\s*,\s*version\s*:\s*["']([^"']+)["']\s*\).*$"#;
    pub const LIBS_DEPENDENCY: &str = r#"^\s*(\w+)\s+libs\.([a-zA-Z0-9.\-_]+)\s*.*$"#;
    pub const DEPENDENCIES_BLOCK: &str = r"dependencies";
    
    // Plugin patterns
    pub const PLUGINS_BLOCK: &str = r"plugins";
    pub const PLUGIN_ID_VERSION: &str = r#"^\s*id\s+["']([^"']+)["']\s+version\s+["']([^"']+)["'].*$"#;
    pub const PLUGIN_ID_ONLY: &str = r#"^\s*id\s+["']([^"']+)["']\s*$"#;
    pub const PLUGIN_KOTLIN_DSL_ID_VERSION: &str = r#"^\s*id\s*\(\s*["']([^"']+)["']\s*\)\s+version\s+["']([^"']+)["'].*$"#;
    pub const PLUGIN_KOTLIN_DSL_ID_ONLY: &str = r#"^\s*id\s*\(\s*["']([^"']+)["']\s*\)\s*$"#;
    pub const PLUGIN_KOTLIN_SHORTHAND_VERSION: &str = r#"^\s*kotlin\s*\(\s*["']([^"']+)["']\s*\)\s+version\s+["']([^"']+)["'].*$"#;
    pub const PLUGIN_KOTLIN_SHORTHAND_ONLY: &str = r#"^\s*([a-zA-Z\-]+)\s*$"#;
    pub const APPLY_PLUGIN: &str = r#"^\s*apply\s*\(\s*plugin\s*=\s*["']([^"']+)["']\s*\).*$"#;
    pub const APPLY_PLUGIN_GROOVY: &str = r#"^\s*apply\s+plugin\s*:\s*["']([^"']+)["'].*$"#;
    pub const LIBS_PLUGIN: &str = r#"^\s*alias\s*\(\s*libs\.plugins\.([a-zA-Z0-9\.\-_]+)\s*\).*$"#;
}