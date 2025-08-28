/*
 * Copyright (c) 2025 ParkJong-Hun
 *
 * Licensed under the MIT License.
 * See LICENSE file in the project root for full license information.
 */

use crate::config::{file_patterns, regex_patterns};
use crate::error::{Result};
use crate::version_catalog::{find_version_catalog_files, parse_version_catalog, VersionCatalog};
use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct Dependency {
    pub group: String,
    pub artifact: String,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DependencyLocation {
    pub dependency: Dependency,
    pub file_path: PathBuf,
    pub line_number: usize,
    pub configuration: String,
    pub source_type: DependencySourceType,
}

#[derive(Debug, Clone, Serialize)]
pub enum DependencySourceType {
    Direct,
    VersionCatalog(String), // The libs.xxx reference
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct Plugin {
    pub id: String,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PluginLocation {
    pub plugin: Plugin,
    pub file_path: PathBuf,
    pub line_number: usize,
    pub source_type: PluginSourceType,
}

#[derive(Debug, Clone, Serialize)]
pub enum PluginSourceType {
    PluginsBlock,
    ApplyPlugin,
    VersionCatalog(String), // The libs.plugins.xxx reference
}

struct DependencyPatterns {
    string_dep: Regex,
    map_dep_group_first: Regex,
    map_dep_name_first: Regex,
    libs_dep: Regex,
    version_catalog_dep: Regex,
    project_dep: Regex,
    projects_accessor_dep: Regex,
}

struct PluginPatterns {
    plugin_id_version: Regex,
    plugin_id_only: Regex,
    plugin_kotlin_dsl_id_version: Regex,
    plugin_kotlin_dsl_id_only: Regex,
    plugin_kotlin_shorthand_version: Regex,
    plugin_kotlin_shorthand_only: Regex,
    apply_plugin: Regex,
    apply_plugin_groovy: Regex,
    libs_plugin: Regex,
}

fn create_dependency_patterns() -> Result<DependencyPatterns> {
    Ok(DependencyPatterns {
        string_dep: Regex::new(regex_patterns::STRING_DEPENDENCY)?,
        map_dep_group_first: Regex::new(regex_patterns::MAP_DEPENDENCY_1)?,
        map_dep_name_first: Regex::new(regex_patterns::MAP_DEPENDENCY_2)?,
        libs_dep: Regex::new(regex_patterns::LIBS_DEPENDENCY)?,
        version_catalog_dep: Regex::new(regex_patterns::VERSION_CATALOG_DEPENDENCY)?,
        project_dep: Regex::new(regex_patterns::PROJECT_DEPENDENCY)?,
        projects_accessor_dep: Regex::new(regex_patterns::PROJECTS_ACCESSOR_DEPENDENCY)?,
    })
}

fn create_plugin_patterns() -> Result<PluginPatterns> {
    Ok(PluginPatterns {
        plugin_id_version: Regex::new(regex_patterns::PLUGIN_ID_VERSION)?,
        plugin_id_only: Regex::new(regex_patterns::PLUGIN_ID_ONLY)?,
        plugin_kotlin_dsl_id_version: Regex::new(regex_patterns::PLUGIN_KOTLIN_DSL_ID_VERSION)?,
        plugin_kotlin_dsl_id_only: Regex::new(regex_patterns::PLUGIN_KOTLIN_DSL_ID_ONLY)?,
        plugin_kotlin_shorthand_version: Regex::new(regex_patterns::PLUGIN_KOTLIN_SHORTHAND_VERSION)?,
        plugin_kotlin_shorthand_only: Regex::new(regex_patterns::PLUGIN_KOTLIN_SHORTHAND_ONLY)?,
        apply_plugin: Regex::new(regex_patterns::APPLY_PLUGIN)?,
        apply_plugin_groovy: Regex::new(regex_patterns::APPLY_PLUGIN_GROOVY)?,
        libs_plugin: Regex::new(regex_patterns::LIBS_PLUGIN)?,
    })
}

pub fn find_gradle_files(root_path: &Path) -> Result<Vec<PathBuf>> {
    let mut gradle_files = Vec::new();
    
    for entry in WalkDir::new(root_path) {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                if file_patterns::GRADLE_BUILD_FILES.contains(&filename) {
                    gradle_files.push(path.to_path_buf());
                }
            }
        }
    }
    
    Ok(gradle_files)
}

pub fn load_version_catalogs(root_path: &Path) -> Result<HashMap<PathBuf, VersionCatalog>> {
    let catalog_files = find_version_catalog_files(root_path)?;
    let mut catalogs = HashMap::new();
    
    for catalog_file in catalog_files {
        let catalog = parse_version_catalog(&catalog_file)?;
        catalogs.insert(catalog_file, catalog);
    }
    
    Ok(catalogs)
}

pub fn parse_plugins_from_file(
    file_path: &Path,
    version_catalogs: &HashMap<PathBuf, VersionCatalog>
) -> Result<Vec<PluginLocation>> {
    let content = fs::read_to_string(file_path)?;
    let mut plugins = Vec::new();
    let mut in_plugins_block = false;
    let mut brace_count = 0;
    
    let patterns = create_plugin_patterns()?;
    
    for (line_number, line) in content.lines().enumerate() {
        let trimmed_line = line.trim();
        
        // Check if we're entering a plugins block
        if trimmed_line.starts_with(regex_patterns::PLUGINS_BLOCK) && trimmed_line.contains('{') {
            in_plugins_block = true;
            brace_count = 1;
            continue;
        }
        
        // Parse apply plugin statements anywhere in the file
        if let Some(plugin) = parse_apply_plugin(&patterns.apply_plugin, trimmed_line, file_path, line_number + 1)? {
            plugins.push(plugin);
        } else if let Some(plugin) = parse_apply_plugin(&patterns.apply_plugin_groovy, trimmed_line, file_path, line_number + 1)? {
            plugins.push(plugin);
        }
        
        if in_plugins_block {
            // Count braces to track nested blocks
            brace_count += trimmed_line.matches('{').count();
            brace_count -= trimmed_line.matches('}').count();
            
            if brace_count == 0 {
                in_plugins_block = false;
                continue;
            }
            
            // Parse different plugin formats inside plugins block - check versioned patterns first
            if let Some(plugin) = parse_plugin_id_version(&patterns.plugin_id_version, trimmed_line, file_path, line_number + 1)? {
                plugins.push(plugin);
            } else if let Some(plugin) = parse_plugin_kotlin_dsl_id_version(&patterns.plugin_kotlin_dsl_id_version, trimmed_line, file_path, line_number + 1)? {
                plugins.push(plugin);
            } else if let Some(plugin) = parse_plugin_kotlin_shorthand_version(&patterns.plugin_kotlin_shorthand_version, trimmed_line, file_path, line_number + 1)? {
                plugins.push(plugin);
            } else if let Some(plugin) = parse_plugin_id_only(&patterns.plugin_id_only, trimmed_line, file_path, line_number + 1)? {
                plugins.push(plugin);
            } else if let Some(plugin) = parse_plugin_kotlin_dsl_id_only(&patterns.plugin_kotlin_dsl_id_only, trimmed_line, file_path, line_number + 1)? {
                plugins.push(plugin);
            } else if let Some(plugin) = parse_plugin_kotlin_shorthand_only(&patterns.plugin_kotlin_shorthand_only, trimmed_line, file_path, line_number + 1)? {
                plugins.push(plugin);
            } else if let Some(plugin) = parse_libs_plugin(&patterns.libs_plugin, trimmed_line, file_path, line_number + 1, version_catalogs)? {
                plugins.push(plugin);
            }
        }
    }
    
    Ok(plugins)
}

pub fn parse_dependencies_from_file(
    file_path: &Path, 
    version_catalogs: &HashMap<PathBuf, VersionCatalog>
) -> Result<Vec<DependencyLocation>> {
    let content = fs::read_to_string(file_path)?;
    let mut dependencies = Vec::new();
    
    let patterns = create_dependency_patterns()?;
    
    #[derive(Debug)]
    enum ParserState {
        Normal,
        InKotlin(i32), // brace count for kotlin block
        InSourceSets(i32), // brace count for sourceSets block  
        InSourceSet(String, i32), // source set name, brace count for this sourceSet
        InDependencies(String, i32), // source set name, brace count for dependencies block
    }
    
    let mut state = ParserState::Normal;
    
    for (line_number, line) in content.lines().enumerate() {
        let trimmed_line = line.trim();
        
        state = match state {
            ParserState::Normal => {
                if trimmed_line.starts_with("kotlin") && trimmed_line.contains('{') {
                    ParserState::InKotlin(1)
                } else if trimmed_line.starts_with("sourceSets") && trimmed_line.contains('{') {
                    ParserState::InSourceSets(1)
                } else if trimmed_line.starts_with(regex_patterns::DEPENDENCIES_BLOCK) && trimmed_line.contains('{') {
                    ParserState::InDependencies("main".to_string(), 1)
                } else {
                    ParserState::Normal
                }
            }
            
            ParserState::InKotlin(mut brace_count) => {
                brace_count += trimmed_line.matches('{').count() as i32;
                brace_count -= trimmed_line.matches('}').count() as i32;
                
                if brace_count == 0 {
                    ParserState::Normal
                } else if trimmed_line.starts_with("sourceSets") && trimmed_line.contains('{') {
                    ParserState::InSourceSets(1)
                } else {
                    ParserState::InKotlin(brace_count)
                }
            }
            
            ParserState::InSourceSets(mut brace_count) => {
                brace_count += trimmed_line.matches('{').count() as i32;
                brace_count -= trimmed_line.matches('}').count() as i32;
                
                if brace_count == 0 {
                    ParserState::Normal
                } else if let Some(source_set_name) = extract_source_set_dependencies(trimmed_line) {
                    // Pattern: commonMain.dependencies {
                    ParserState::InDependencies(source_set_name, 1)
                } else if let Some(source_set_name) = extract_source_set_block(trimmed_line) {
                    // Pattern: commonMain {
                    ParserState::InSourceSet(source_set_name, 1)
                } else {
                    ParserState::InSourceSets(brace_count)
                }
            }
            
            ParserState::InSourceSet(source_set_name, mut brace_count) => {
                if trimmed_line.starts_with(regex_patterns::DEPENDENCIES_BLOCK) && trimmed_line.contains('{') {
                    // Found dependencies { inside this sourceSet - don't update brace_count for the sourceSet
                    ParserState::InDependencies(source_set_name, 1)
                } else {
                    brace_count += trimmed_line.matches('{').count() as i32;
                    brace_count -= trimmed_line.matches('}').count() as i32;
                    
                    if brace_count == 0 {
                        // Exited this sourceSet, back to sourceSets level
                        ParserState::InSourceSets(1)
                    } else {
                        ParserState::InSourceSet(source_set_name, brace_count)
                    }
                }
            }
            
            ParserState::InDependencies(source_set_name, mut brace_count) => {
                brace_count += trimmed_line.matches('{').count() as i32;
                brace_count -= trimmed_line.matches('}').count() as i32;
                
                if brace_count == 0 {
                    // Dependencies block ended
                    if source_set_name == "main" {
                        ParserState::Normal
                    } else {
                        // We were in a sourceSet dependencies block, go back to sourceSets level
                        // Need to recalculate the sourceSets brace count
                        ParserState::InSourceSets(1)
                    }
                } else {
                    // Parse dependencies in this block
                    let source_set_suffix = if source_set_name == "main" {
                        "".to_string()
                    } else {
                        format!("-{}", source_set_name)
                    };
                    
                    // Check if this is a project dependency (should be ignored)
                    if is_project_dependency(&patterns, trimmed_line) {
                        // Skip project dependencies - clone source_set_name for next iteration
                        ParserState::InDependencies(source_set_name.clone(), brace_count)
                    } else if let Some(mut dep) = parse_string_dependency(&patterns.string_dep, trimmed_line, file_path, line_number + 1)? {
                        dep.configuration = format!("{}{}", dep.configuration, source_set_suffix);
                        dependencies.push(dep);
                        ParserState::InDependencies(source_set_name, brace_count)
                    } else if let Some(mut dep) = parse_map_dependency_group_first(&patterns.map_dep_group_first, trimmed_line, file_path, line_number + 1)? {
                        dep.configuration = format!("{}{}", dep.configuration, source_set_suffix);
                        dependencies.push(dep);
                        ParserState::InDependencies(source_set_name, brace_count)
                    } else if let Some(mut dep) = parse_map_dependency_name_first(&patterns.map_dep_name_first, trimmed_line, file_path, line_number + 1)? {
                        dep.configuration = format!("{}{}", dep.configuration, source_set_suffix);
                        dependencies.push(dep);
                        ParserState::InDependencies(source_set_name, brace_count)
                    } else if let Some(mut dep) = parse_libs_dependency(&patterns.libs_dep, trimmed_line, file_path, line_number + 1, version_catalogs)? {
                        dep.configuration = format!("{}{}", dep.configuration, source_set_suffix);
                        dependencies.push(dep);
                        ParserState::InDependencies(source_set_name, brace_count)
                    } else if let Some(mut dep) = parse_version_catalog_dependency(&patterns.version_catalog_dep, trimmed_line, file_path, line_number + 1, version_catalogs)? {
                        dep.configuration = format!("{}{}", dep.configuration, source_set_suffix);
                        dependencies.push(dep);
                        ParserState::InDependencies(source_set_name, brace_count)
                    } else {
                        ParserState::InDependencies(source_set_name, brace_count)
                    }
                }
            }
        };
    }
    
    Ok(dependencies)
}

fn create_dependency_location(
    group: String,
    artifact: String,
    version: Option<String>,
    configuration: String,
    file_path: &Path,
    line_number: usize,
    source_type: DependencySourceType,
) -> DependencyLocation {
    DependencyLocation {
        dependency: Dependency { group, artifact, version },
        file_path: file_path.to_path_buf(),
        line_number,
        configuration,
        source_type,
    }
}

fn parse_string_dependency(
    regex: &Regex,
    line: &str,
    file_path: &Path,
    line_number: usize,
) -> Result<Option<DependencyLocation>> {
    if let Some(captures) = regex.captures(line) {
        let configuration = captures[1].to_string();
        let group = captures[2].to_string();
        let artifact = captures[3].to_string();
        let version = captures.get(4).map(|m| m.as_str().to_string());
        
        Ok(Some(create_dependency_location(
            group,
            artifact,
            version,
            configuration,
            file_path,
            line_number,
            DependencySourceType::Direct,
        )))
    } else {
        Ok(None)
    }
}

fn parse_map_dependency_group_first(
    regex: &Regex,
    line: &str,
    file_path: &Path,
    line_number: usize,
) -> Result<Option<DependencyLocation>> {
    if let Some(captures) = regex.captures(line) {
        let configuration = captures[1].to_string();
        let group = captures[2].to_string();
        let artifact = captures[3].to_string();
        let version = Some(captures[4].to_string());
        
        Ok(Some(create_dependency_location(
            group,
            artifact,
            version,
            configuration,
            file_path,
            line_number,
            DependencySourceType::Direct,
        )))
    } else {
        Ok(None)
    }
}

fn parse_map_dependency_name_first(
    regex: &Regex,
    line: &str,
    file_path: &Path,
    line_number: usize,
) -> Result<Option<DependencyLocation>> {
    if let Some(captures) = regex.captures(line) {
        let configuration = captures[1].to_string();
        let artifact = captures[2].to_string();
        let group = captures[3].to_string();
        let version = Some(captures[4].to_string());
        
        Ok(Some(create_dependency_location(
            group,
            artifact,
            version,
            configuration,
            file_path,
            line_number,
            DependencySourceType::Direct,
        )))
    } else {
        Ok(None)
    }
}

fn is_project_dependency(patterns: &DependencyPatterns, line: &str) -> bool {
    patterns.project_dep.is_match(line) || patterns.projects_accessor_dep.is_match(line)
}

fn extract_source_set_dependencies(line: &str) -> Option<String> {
    // Match patterns like "commonMain.dependencies {" or "androidMain.dependencies {"
    let re = regex::Regex::new(r"^\s*([a-zA-Z0-9]+)\.dependencies\s*\{\s*$").ok()?;
    if let Some(captures) = re.captures(line) {
        Some(captures[1].to_string())
    } else {
        None
    }
}

fn extract_source_set_block(line: &str) -> Option<String> {
    // Match patterns like "commonMain {" or "androidMain {"
    let re = regex::Regex::new(r"^\s*([a-zA-Z0-9]+)\s*\{\s*$").ok()?;
    if let Some(captures) = re.captures(line) {
        let name = captures[1].to_string();
        // Only match common source set names to avoid false positives
        if name.ends_with("Main") || name.ends_with("Test") || 
           name == "common" || name == "android" || name == "ios" || name == "jvm" || name == "js" {
            Some(name)
        } else {
            None
        }
    } else {
        None
    }
}

fn parse_version_catalog_dependency(
    regex: &Regex,
    line: &str,
    file_path: &Path,
    line_number: usize,
    version_catalogs: &HashMap<PathBuf, VersionCatalog>,
) -> Result<Option<DependencyLocation>> {
    if let Some(captures) = regex.captures(line) {
        let configuration = captures[1].to_string();
        let catalog_name = captures[2].to_string();
        let lib_reference = captures[3].to_string();
        
        // Skip known non-library catalogs
        if catalog_name == "projects" {
            return Ok(None); // This is a projects accessor, not a version catalog
        }
        
        // Try to resolve from version catalogs
        for catalog in version_catalogs.values() {
            // For compose.xxx, look for compose-xxx or compose.xxx in catalog
            let lookup_key = if catalog_name == "compose" {
                format!("compose-{}", lib_reference.replace('.', "-"))
            } else {
                lib_reference.replace('.', "-")
            };
            
            if let Some((group, artifact, version)) = catalog.resolve_library_version(&lookup_key) {
                return Ok(Some(create_dependency_location(
                    group,
                    artifact,
                    Some(version),
                    configuration,
                    file_path,
                    line_number,
                    DependencySourceType::VersionCatalog(format!("{}.{}", catalog_name, lib_reference)),
                )));
            }
        }
        
        // If not found in catalogs, check if it might be a built-in accessor like compose.runtime
        if catalog_name == "compose" {
            // Create a synthetic dependency for compose BOM entries
            return Ok(Some(create_dependency_location(
                "org.jetbrains.compose".to_string(),
                lib_reference.replace('.', "-"),
                None, // Version managed by compose BOM
                configuration,
                file_path,
                line_number,
                DependencySourceType::VersionCatalog(format!("{}.{}", catalog_name, lib_reference)),
            )));
        }
    }
    
    Ok(None)
}

fn parse_libs_dependency(
    regex: &Regex,
    line: &str,
    file_path: &Path,
    line_number: usize,
    version_catalogs: &HashMap<PathBuf, VersionCatalog>,
) -> Result<Option<DependencyLocation>> {
    if let Some(captures) = regex.captures(line) {
        let configuration = captures[1].to_string();
        let lib_reference = captures[2].to_string();
        
        // Try to resolve from version catalogs
        for catalog in version_catalogs.values() {
            if let Some((group, artifact, version)) = catalog.resolve_library_version(&lib_reference) {
                return Ok(Some(create_dependency_location(
                    group,
                    artifact,
                    Some(version),
                    configuration,
                    file_path,
                    line_number,
                    DependencySourceType::VersionCatalog(lib_reference),
                )));
            }
        }
    }
    
    Ok(None)
}

fn create_plugin_location(
    id: String,
    version: Option<String>,
    file_path: &Path,
    line_number: usize,
    source_type: PluginSourceType,
) -> PluginLocation {
    PluginLocation {
        plugin: Plugin { id, version },
        file_path: file_path.to_path_buf(),
        line_number,
        source_type,
    }
}

fn parse_plugin_id_version(
    regex: &Regex,
    line: &str,
    file_path: &Path,
    line_number: usize,
) -> Result<Option<PluginLocation>> {
    if let Some(captures) = regex.captures(line) {
        let id = captures[1].to_string();
        let version = Some(captures[2].to_string());
        
        Ok(Some(create_plugin_location(
            id,
            version,
            file_path,
            line_number,
            PluginSourceType::PluginsBlock,
        )))
    } else {
        Ok(None)
    }
}

fn parse_plugin_id_only(
    regex: &Regex,
    line: &str,
    file_path: &Path,
    line_number: usize,
) -> Result<Option<PluginLocation>> {
    if let Some(captures) = regex.captures(line) {
        let id = captures[1].to_string();
        
        Ok(Some(create_plugin_location(
            id,
            None,
            file_path,
            line_number,
            PluginSourceType::PluginsBlock,
        )))
    } else {
        Ok(None)
    }
}

fn parse_plugin_kotlin_dsl_id_version(
    regex: &Regex,
    line: &str,
    file_path: &Path,
    line_number: usize,
) -> Result<Option<PluginLocation>> {
    if let Some(captures) = regex.captures(line) {
        let id = captures[1].to_string();
        let version = Some(captures[2].to_string());
        
        Ok(Some(create_plugin_location(
            id,
            version,
            file_path,
            line_number,
            PluginSourceType::PluginsBlock,
        )))
    } else {
        Ok(None)
    }
}

fn parse_plugin_kotlin_dsl_id_only(
    regex: &Regex,
    line: &str,
    file_path: &Path,
    line_number: usize,
) -> Result<Option<PluginLocation>> {
    if let Some(captures) = regex.captures(line) {
        let id = captures[1].to_string();
        
        Ok(Some(create_plugin_location(
            id,
            None,
            file_path,
            line_number,
            PluginSourceType::PluginsBlock,
        )))
    } else {
        Ok(None)
    }
}

fn parse_plugin_kotlin_shorthand_version(
    regex: &Regex,
    line: &str,
    file_path: &Path,
    line_number: usize,
) -> Result<Option<PluginLocation>> {
    if let Some(captures) = regex.captures(line) {
        let kotlin_type = captures[1].to_string();
        let version = Some(captures[2].to_string());
        let id = format!("org.jetbrains.kotlin.{}", kotlin_type);
        
        Ok(Some(create_plugin_location(
            id,
            version,
            file_path,
            line_number,
            PluginSourceType::PluginsBlock,
        )))
    } else {
        Ok(None)
    }
}

fn parse_plugin_kotlin_shorthand_only(
    regex: &Regex,
    line: &str,
    file_path: &Path,
    line_number: usize,
) -> Result<Option<PluginLocation>> {
    if let Some(captures) = regex.captures(line) {
        let plugin_name = captures[1].to_string();
        
        // Only handle common plugin names to avoid false matches
        let id = match plugin_name.as_str() {
            "application" => "application".to_string(),
            "java-library" => "java-library".to_string(),
            "java" => "java".to_string(),
            _ => return Ok(None),
        };
        
        Ok(Some(create_plugin_location(
            id,
            None,
            file_path,
            line_number,
            PluginSourceType::PluginsBlock,
        )))
    } else {
        Ok(None)
    }
}

fn parse_apply_plugin(
    regex: &Regex,
    line: &str,
    file_path: &Path,
    line_number: usize,
) -> Result<Option<PluginLocation>> {
    if let Some(captures) = regex.captures(line) {
        let id = captures[1].to_string();
        
        Ok(Some(create_plugin_location(
            id,
            None,
            file_path,
            line_number,
            PluginSourceType::ApplyPlugin,
        )))
    } else {
        Ok(None)
    }
}

fn parse_libs_plugin(
    regex: &Regex,
    line: &str,
    file_path: &Path,
    line_number: usize,
    version_catalogs: &HashMap<PathBuf, VersionCatalog>,
) -> Result<Option<PluginLocation>> {
    if let Some(captures) = regex.captures(line) {
        let plugin_reference = captures[1].to_string();
        
        // Try to resolve from version catalogs
        for catalog in version_catalogs.values() {
            if let Some((id, version)) = catalog.resolve_plugin_version(&plugin_reference) {
                return Ok(Some(create_plugin_location(
                    id,
                    version,
                    file_path,
                    line_number,
                    PluginSourceType::VersionCatalog(plugin_reference),
                )));
            }
        }
    }
    
    Ok(None)
}