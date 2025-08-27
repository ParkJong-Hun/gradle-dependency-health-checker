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

## Performance Tips

- Use specific subcommands to analyze only what you need
- Set higher thresholds to reduce noise in large projects  
- Use `--silent` mode in CI/CD pipelines to reduce log noise
- Combine `--output` with custom thresholds for automated analysis