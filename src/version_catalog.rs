/*
 * Copyright (c) 2025 ParkJong-Hun
 *
 * Licensed under the MIT License.
 * See LICENSE file in the project root for full license information.
 */

use crate::config::file_patterns;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Deserialize, Serialize)]
pub struct VersionCatalog {
    pub versions: Option<HashMap<String, String>>,
    pub libraries: Option<HashMap<String, LibraryDefinition>>,
    pub plugins: Option<HashMap<String, PluginDefinition>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LibraryDefinition {
    pub group: Option<String>,
    pub name: Option<String>,
    pub version: Option<VersionRef>,
    #[serde(rename = "version.ref")]
    pub version_ref: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum VersionRef {
    Direct(String),
    Reference { r#ref: String },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PluginDefinition {
    pub id: String,
    pub version: Option<VersionRef>,
    #[serde(rename = "version.ref")]
    pub version_ref: Option<String>,
}

pub fn find_version_catalog_files(root_path: &Path) -> Result<Vec<PathBuf>> {
    let mut catalog_files = Vec::new();
    
    for entry in WalkDir::new(root_path) {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                if file_patterns::VERSION_CATALOG_FILES.contains(&filename) {
                    catalog_files.push(path.to_path_buf());
                }
            }
        }
    }
    
    Ok(catalog_files)
}

pub fn parse_version_catalog(file_path: &Path) -> Result<VersionCatalog> {
    let content = fs::read_to_string(file_path)?;
    let catalog: VersionCatalog = toml::from_str(&content)?;
    Ok(catalog)
}

impl VersionCatalog {
    pub fn resolve_library_version(&self, library_name: &str) -> Option<(String, String, String)> {
        let libraries = self.libraries.as_ref()?;
        let library_def = libraries.get(library_name)?;
        
        let group = library_def.group.as_ref()?.clone();
        let name = library_def.name.as_ref()?.clone();
        
        // Try to get version from version_ref first, then from version field
        let version = if let Some(version_ref) = &library_def.version_ref {
            self.versions.as_ref()?.get(version_ref)?.clone()
        } else if let Some(version) = &library_def.version {
            match version {
                VersionRef::Direct(v) => v.clone(),
                VersionRef::Reference { r#ref } => {
                    self.versions.as_ref()?.get(r#ref)?.clone()
                }
            }
        } else {
            return None;
        };
        
        Some((group, name, version))
    }
    
    pub fn resolve_plugin_version(&self, plugin_name: &str) -> Option<(String, Option<String>)> {
        let plugins = self.plugins.as_ref()?;
        let plugin_def = plugins.get(plugin_name)?;
        
        let id = plugin_def.id.clone();
        
        // Try to get version from version_ref first, then from version field
        let version = if let Some(version_ref) = &plugin_def.version_ref {
            self.versions.as_ref()?.get(version_ref).cloned()
        } else if let Some(version) = &plugin_def.version {
            match version {
                VersionRef::Direct(v) => Some(v.clone()),
                VersionRef::Reference { r#ref } => {
                    self.versions.as_ref()?.get(r#ref).cloned()
                }
            }
        } else {
            None
        };
        
        Some((id, version))
    }
}