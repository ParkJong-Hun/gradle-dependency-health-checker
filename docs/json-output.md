# JSON Output

## Basic Usage

### Output to File
```bash
# Output results to JSON file instead of console
gradle-dependency-health-checker --output analysis.json

# Output specific analysis to JSON
gradle-dependency-health-checker conflicts --output conflicts.json
gradle-dependency-health-checker dependencies --output deps.json
gradle-dependency-health-checker bundles --output bundles.json

# Combine with other options
gradle-dependency-health-checker all \
  --path ./my-project \
  --output results.json \
  --min-version-conflicts 3
```

### Silent Mode
```bash
# Generate JSON file without console output (useful for CI/CD)
gradle-dependency-health-checker --output analysis.json --silent

# Completely silent execution (no output at all)
gradle-dependency-health-checker --path ./my-project --silent

# Silent mode with subcommands
gradle-dependency-health-checker conflicts \
  --output conflicts.json \
  --silent
```

## JSON Structure

The JSON output contains three main sections:

### Complete Structure Overview
```json
{
  "duplicate_analysis": {
    "regular_duplicates": { ... },
    "version_conflicts": { ... }
  },
  "plugin_analysis": {
    "duplicate_plugins": { ... }
  },
  "bundle_analysis": {
    "recommended_bundles": [ ... ],
    "total_bundles_found": 8
  }
}
```

### Detailed Examples

#### Version Conflicts
```json
{
  "duplicate_analysis": {
    "version_conflicts": {
      "com.squareup.okhttp3:okhttp": [
        {
          "dependency": {
            "group": "com.squareup.okhttp3",
            "artifact": "okhttp",
            "version": "4.12.0"
          },
          "file_path": "app/build.gradle",
          "line_number": 12,
          "configuration": "implementation",
          "source_type": "Direct"
        },
        {
          "dependency": {
            "group": "com.squareup.okhttp3",
            "artifact": "okhttp", 
            "version": "4.10.0"
          },
          "file_path": "feature/build.gradle",
          "line_number": 8,
          "configuration": "implementation",
          "source_type": "VersionCatalog(libs.okhttp)"
        }
      ]
    }
  }
}
```

#### Duplicate Dependencies
```json
{
  "duplicate_analysis": {
    "regular_duplicates": {
      "com.squareup.retrofit2:retrofit": [
        {
          "dependency": {
            "group": "com.squareup.retrofit2",
            "artifact": "retrofit",
            "version": "2.9.0"
          },
          "file_path": "app/build.gradle",
          "line_number": 15,
          "configuration": "implementation",
          "source_type": "Direct"
        },
        {
          "dependency": {
            "group": "com.squareup.retrofit2",
            "artifact": "retrofit",
            "version": "2.9.0"
          },
          "file_path": "feature/build.gradle",
          "line_number": 10,
          "configuration": "implementation",
          "source_type": "Direct"
        }
      ]
    }
  }
}
```

#### Duplicate Plugins
```json
{
  "plugin_analysis": {
    "duplicate_plugins": {
      "java": [
        {
          "plugin": {
            "id": "java",
            "version": null
          },
          "file_path": "app/build.gradle",
          "line_number": 3,
          "source_type": "PluginsBlock"
        },
        {
          "plugin": {
            "id": "java",
            "version": null
          },
          "file_path": "feature/build.gradle",
          "line_number": 2,
          "source_type": "ApplyPlugin"
        }
      ]
    }
  }
}
```

#### Bundle Recommendations
```json
{
  "bundle_analysis": {
    "recommended_bundles": [
      {
        "dependencies": [
          "com.squareup.retrofit2:retrofit",
          "com.squareup.okhttp3:okhttp"
        ],
        "modules": [
          "app/build.gradle",
          "feature/build.gradle"
        ],
        "bundle_size": 2,
        "module_count": 2,
        "configurations": ["implementation"],
        "priority_score": 0.8
      }
    ],
    "total_bundles_found": 5
  }
}
```

## Field Definitions

### Source Types
- `"Direct"`: Directly declared dependency/plugin
- `"VersionCatalog(reference)"`: From version catalog (e.g., `libs.retrofit`)

### Plugin Source Types
- `"PluginsBlock"`: Declared in `plugins { }` block
- `"ApplyPlugin"`: Declared with `apply plugin:` statement  
- `"VersionCatalog(reference)"`: From version catalog plugins section

### Bundle Fields
- `dependencies`: List of dependencies in `group:artifact` format
- `modules`: Build files that use these dependencies
- `bundle_size`: Number of dependencies in the bundle
- `module_count`: Number of modules sharing the dependencies
- `configurations`: Gradle configurations used (implementation, api, etc.)
- `priority_score`: Calculated recommendation priority (0.0 to 1.0)

## Integration Examples

### Python Processing
```python
import json

with open('analysis.json', 'r') as f:
    data = json.load(f)

# Process version conflicts
for artifact, conflicts in data['duplicate_analysis']['version_conflicts'].items():
    print(f"Conflict in {artifact}:")
    for conflict in conflicts:
        print(f"  - {conflict['file_path']}:{conflict['line_number']} = {conflict['dependency']['version']}")
```

### Shell Script Processing
```bash
# Extract bundle recommendations count
BUNDLE_COUNT=$(cat analysis.json | jq '.bundle_analysis.total_bundles_found')

# List all conflicted artifacts
cat analysis.json | jq -r '.duplicate_analysis.version_conflicts | keys[]'

# Get high-priority bundles
cat analysis.json | jq '.bundle_analysis.recommended_bundles[] | select(.priority_score > 0.7)'
```