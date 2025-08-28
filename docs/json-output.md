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
          "file_path": "feature/build.gradle.kts",
          "line_number": 8,
          "configuration": "implementation",
          "source_type": {
            "VersionCatalog": "libs.okhttp"
          }
        }
      ],
      "androidx.core:core-ktx": [
        {
          "dependency": {
            "group": "androidx.core",
            "artifact": "core-ktx",
            "version": "1.13.0"
          },
          "file_path": "core/build.gradle.kts",
          "line_number": 15,
          "configuration": "implementation-androidMain",
          "source_type": "Direct"
        },
        {
          "dependency": {
            "group": "androidx.core",
            "artifact": "core-ktx",
            "version": "1.12.0"
          },
          "file_path": "feature/session/build.gradle.kts",
          "line_number": 22,
          "configuration": "androidMainImplementation",
          "source_type": {
            "VersionCatalog": "libs.androidx.core.ktx"
          }
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
      "org.jetbrains.compose:runtime": [
        {
          "dependency": {
            "group": "org.jetbrains.compose",
            "artifact": "runtime",
            "version": null
          },
          "file_path": "core/model/build.gradle.kts",
          "line_number": 13,
          "configuration": "implementation-commonMain",
          "source_type": {
            "VersionCatalog": "compose.runtime"
          }
        },
        {
          "dependency": {
            "group": "org.jetbrains.compose",
            "artifact": "runtime",
            "version": null
          },
          "file_path": "core/common/build.gradle.kts",
          "line_number": 13,
          "configuration": "commonMainImplementation",
          "source_type": {
            "VersionCatalog": "compose.runtime"
          }
        },
        {
          "dependency": {
            "group": "org.jetbrains.compose",
            "artifact": "runtime",
            "version": null
          },
          "file_path": "app-android/build.gradle.kts",
          "line_number": 72,
          "configuration": "implementation",
          "source_type": {
            "VersionCatalog": "compose.runtime"
          }
        }
      ],
      "io.ktor:ktor-client-core": [
        {
          "dependency": {
            "group": "io.ktor",
            "artifact": "ktor-client-core",
            "version": "2.3.5"
          },
          "file_path": "core/network/build.gradle.kts",
          "line_number": 15,
          "configuration": "api-commonMain",
          "source_type": {
            "VersionCatalog": "libs.ktor.client.core"
          }
        },
        {
          "dependency": {
            "group": "io.ktor",
            "artifact": "ktor-client-core",
            "version": "2.3.5"
          },
          "file_path": "feature/session/build.gradle.kts",
          "line_number": 12,
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
          "org.jetbrains.compose:runtime",
          "org.jetbrains.compose:ui",
          "androidx.compose.material3:material3",
          "androidx.compose.ui:ui-tooling"
        ],
        "modules": [
          "core/designsystem/build.gradle.kts",
          "feature/session/build.gradle.kts",
          "feature/timetable/build.gradle.kts",
          "app-android/build.gradle.kts"
        ],
        "bundle_size": 4,
        "module_count": 4,
        "configurations": [
          "api-commonMain",
          "implementation",
          "implementation-commonMain"
        ],
        "priority_score": 65.0
      },
      {
        "dependencies": [
          "io.ktor:ktor-client-core",
          "io.ktor:ktor-client-json", 
          "org.jetbrains.kotlinx:kotlinx-serialization-json"
        ],
        "modules": [
          "core/network/build.gradle.kts",
          "feature/session/build.gradle.kts",
          "feature/sponsors/build.gradle.kts"
        ],
        "bundle_size": 3,
        "module_count": 3,
        "configurations": [
          "api",
          "implementation"
        ],
        "priority_score": 32.5
      }
    ],
    "total_bundles_found": 8
  }
}
```

## Field Definitions

### Source Types
- **`"Direct"`**: Directly declared dependency/plugin (e.g., `implementation("group:artifact:version")`)
- **`{"VersionCatalog": "reference"}`**: From version catalog (e.g., `libs.retrofit`, `compose.runtime`)
  - `libs.xxx` references: Standard version catalog entries
  - `compose.xxx` references: Compose BOM-managed dependencies

### Plugin Source Types
- **`"PluginsBlock"`**: Declared in `plugins { }` block
- **`"ApplyPlugin"`**: Declared with `apply plugin:` statement
- **`{"VersionCatalog": "reference"}`**: From version catalog plugins section

### Configuration Types
- **Standard configurations**: `implementation`, `api`, `compileOnly`, `testImplementation`, etc.
- **SourceSet-specific**: `commonMainImplementation`, `androidMainApi`, etc.
- **SourceSet-suffixed**: `implementation-commonMain`, `api-androidMain`, etc.

### Bundle Fields
- **`dependencies`**: List of dependencies in `group:artifact` format
- **`modules`**: Build files that use these dependencies  
- **`bundle_size`**: Number of dependencies in the bundle
- **`module_count`**: Number of modules sharing the dependencies
- **`configurations`**: Gradle configurations used (implementation, api, etc.)
- **`priority_score`**: Calculated recommendation priority (higher = more recommended)

### Version Information
- **`version: "1.0.0"`**: Explicit version from dependency declaration
- **`version: null`**: Version managed by BOM or version catalog without explicit version

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

# Get high-priority bundles (score > 50)
cat analysis.json | jq '.bundle_analysis.recommended_bundles[] | select(.priority_score > 50)'
```