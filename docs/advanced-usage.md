# Advanced Usage

## Custom Thresholds

### Basic Threshold Configuration
```bash
# Run all checks with custom thresholds
gradle-dependency-health-checker all \
  --path ./my-project \
  --min-version-conflicts 3 \
  --min-duplicate-dependencies 2 \
  --min-duplicate-plugins 2 \
  --min-bundle-size 2 \
  --min-bundle-modules 3 \
  --max-bundle-recommendations 10

# Run only conflict analysis with custom threshold
gradle-dependency-health-checker conflicts \
  --path ./my-project \
  --min-version-conflicts 5
```

### Available Commands

| Command | Description | Available Options |
|---------|-------------|-------------------|
| **(default)** | Run all checks | All options available |
| `all` | Run all checks explicitly | All options available |
| `conflicts` | Check version conflicts only | `--min-version-conflicts` |
| `dependencies` | Check duplicate dependencies only | `--min-duplicate-dependencies` |
| `plugins` | Check duplicate plugins only | `--min-duplicate-plugins` |
| `duplicates` | Check both dependency and plugin duplicates | `--min-duplicate-dependencies`, `--min-duplicate-plugins` |
| `bundles` | Generate bundle recommendations only | `--min-bundle-size`, `--min-bundle-modules`, `--max-bundle-recommendations` |

### Threshold Options (defaults)

| Option | Default | Description |
|--------|---------|-------------|
| `--min-version-conflicts` | `2` | Minimum number of version conflicts to display |
| `--min-duplicate-dependencies` | `2` | Minimum number of duplicate dependencies to display |
| `--min-duplicate-plugins` | `2` | Minimum number of duplicate plugins to display |
| `--min-bundle-size` | `2` | Minimum number of dependencies for bundle recommendation |
| `--min-bundle-modules` | `2` | Minimum number of modules for bundle recommendation |
| `--max-bundle-recommendations` | `5` | Maximum number of bundle recommendations to display |

## Global Options

| Option | Default | Description |
|--------|---------|-------------|
| `--path` | `.` | Path to the Gradle project to analyze |
| `--output` | *none* | Output results to JSON file instead of console |
| `--silent` | `false` | Suppress all output messages (useful with --output) |

## Kotlin Multiplatform Projects

### Project Structure Support
```bash
# Analyze Kotlin Multiplatform project with complex sourceSets
gradle-dependency-health-checker --path ./my-kmp-project

# Focus on bundle recommendations for shared dependencies
gradle-dependency-health-checker bundles \
  --path ./my-kmp-project \
  --min-bundle-modules 3 \
  --min-bundle-size 2
```

### SourceSet-Specific Analysis
The tool automatically recognizes and properly categorizes dependencies from different sourceSets:

- **`commonMain.dependencies`**: Tagged as `implementation-commonMain`
- **`androidMain { dependencies }`**: Tagged as `implementation-androidMain` 
- **`commonMainImplementation()`**: Recognized as sourceSet-specific configuration
- **Mixed patterns**: All supported in the same project

### Version Catalog Integration
```bash
# Analyze project using version catalogs with Kotlin Multiplatform
gradle-dependency-health-checker --path ./kmp-with-catalog

# Example project structure:
# ├── gradle/libs.versions.toml
# ├── core/
# │   └── build.gradle.kts (uses libs.xxx)
# └── feature/
#     └── build.gradle.kts (mixed libs.xxx and direct strings)
```

## CI/CD Integration

### GitHub Actions
```yaml
name: Dependency Analysis
on: [pull_request]

jobs:
  dependency-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Build Dependency Checker
        run: |
          git clone https://github.com/ParkJong-Hun/gradle-dependency-health-checker.git
          cd gradle-dependency-health-checker
          cargo build --release
          
      - name: Run Dependency Analysis
        run: |
          ./gradle-dependency-health-checker/target/release/gradle-dependency-health-checker \
            --path . \
            --output dependency-analysis.json \
            --silent \
            --min-version-conflicts 1 \
            --max-bundle-recommendations 10
            
      - name: Upload Results
        uses: actions/upload-artifact@v3
        with:
          name: dependency-analysis
          path: dependency-analysis.json
```

### Jenkins Pipeline
```groovy
pipeline {
    agent any
    
    stages {
        stage('Dependency Analysis') {
            steps {
                script {
                    sh '''
                        gradle-dependency-health-checker \
                            --path $WORKSPACE \
                            --output analysis.json \
                            --silent \
                            --min-version-conflicts 1
                    '''
                    
                    // Process results
                    def analysis = readJSON file: 'analysis.json'
                    def conflicts = analysis.duplicate_analysis.version_conflicts.size()
                    
                    if (conflicts > 0) {
                        unstable("Found ${conflicts} version conflicts")
                    }
                }
            }
        }
    }
}
```

## Performance Tips

### For Large Projects
- Use specific subcommands to analyze only what you need
- Set higher thresholds to reduce noise: `--min-duplicate-dependencies 3`
- Use `--silent` mode in CI/CD pipelines to reduce log noise
- Combine `--output` with custom thresholds for automated analysis

### For Multiplatform Projects  
- Bundle recommendations are especially valuable for identifying common sourceSet dependencies
- Focus on `--min-bundle-modules 3` to find dependencies shared across multiple sourceSets
- Version conflicts are rare in well-managed KMP projects due to version catalogs