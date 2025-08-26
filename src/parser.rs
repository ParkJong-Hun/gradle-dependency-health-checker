use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Dependency {
    pub group: String,
    pub artifact: String,
    pub version: Option<String>,
}

#[derive(Debug)]
pub struct DependencyLocation {
    pub dependency: Dependency,
    pub file_path: PathBuf,
    pub line_number: usize,
    pub configuration: String,
}

pub fn find_gradle_files(root_path: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
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

pub fn parse_dependencies_from_file(file_path: &Path) -> Result<Vec<DependencyLocation>, Box<dyn std::error::Error>> {
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