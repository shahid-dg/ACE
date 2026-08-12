#

````markdown
# ACE Configuration

## 1. Purpose

This document defines how I configure ACE across its CLI, analysis engine, input processing, output generation, and runtime behavior.

I use configuration to control how ACE behaves without requiring changes to source code.

My configuration system is designed to provide:

- predictable defaults
- explicit overrides
- reproducible analysis
- environment-specific settings
- validation before execution
- configuration files suitable for CI/CD
- CLI overrides for quick experiments

---

# 2. Configuration Sources

I support configuration from multiple sources.

The configuration hierarchy is:

```text
Built-in Defaults
       │
       ▼
Configuration File
       │
       ▼
Environment Variables
       │
       ▼
CLI Arguments
       │
       ▼
Final Runtime Configuration
````

Later sources override earlier sources.

Therefore:

```text
CLI > Environment > File > Defaults
```

This allows me to keep stable project configuration in version-controlled files while still overriding individual values when required.

---

# 3. Configuration File Format

I use TOML as the primary ACE configuration format.

Example:

```toml
[dataset]
strict = true
allow_empty = false

[analysis]
parallel = true
threads = 0

[analysis.consensus]
enabled = true
algorithm = "dawid-skene"
max_iterations = 25
tolerance = 0.0001

[output]
format = "json"
directory = "./output"
```

TOML provides:

* readable configuration
* comments
* nested sections
* strong compatibility with Rust tooling
* straightforward version control
* easy manual editing

---

# 4. Default Configuration

ACE always starts with a complete set of defaults.

Conceptually:

```toml
[dataset]
strict = true
allow_empty = false

[analysis]
parallel = true
threads = 0

[output]
format = "json"
directory = "./output"
```

I do not require users to specify every option.

A minimal configuration file may therefore contain only:

```toml
[analysis.consensus]
algorithm = "majority-vote"
```

All unspecified values inherit ACE defaults.

---

# 5. Configuration Structure

I organize configuration into logical sections.

```text
configuration
│
├── dataset
│
├── input
│
├── analysis
│   ├── agreement
│   ├── consensus
│   └── quality
│
├── performance
│
├── output
│
├── report
│
└── logging
```

Each section controls one area of the application.

---

# 6. Dataset Configuration

The `[dataset]` section controls validation and dataset-level behavior.

Example:

```toml
[dataset]
strict = true
allow_empty = false
duplicate_policy = "error"
```

Supported duplicate policies are:

```text
error
first
last
```

The default is:

```toml
duplicate_policy = "error"
```

I prefer failing explicitly over silently modifying annotation data.

---

# 7. Strict Validation

Strict validation is enabled by default:

```toml
[dataset]
strict = true
```

When strict validation is enabled, ACE rejects malformed or inconsistent data rather than attempting to silently repair it.

Examples include:

```text
missing item ID
missing annotator ID
missing label
invalid annotation reference
duplicate active annotation
invalid configuration value
```

---

# 8. Empty Datasets

By default:

```toml
[dataset]
allow_empty = false
```

An empty dataset causes validation to fail.

If an empty dataset is intentionally required:

```toml
[dataset]
allow_empty = true
```

This is useful for testing ingestion pipelines and validating integrations.

---

# 9. Input Configuration

I configure input behavior through `[input]`.

Example:

```toml
[input]
format = "auto"
path = "./data/annotations.csv"
```

Supported formats will include:

```text
csv
json
parquet
auto
```

When:

```toml
format = "auto"
```

I allow ACE to determine the format using the file extension and supported format detection.

---

# 10. Input Encoding

For text-based input formats, UTF-8 is the default encoding.

Example:

```toml
[input]
encoding = "utf-8"
```

If a future connector requires another encoding, I will expose it explicitly rather than performing undocumented conversions.

---

# 11. Input Validation

I validate input before statistical analysis begins.

The processing pipeline is:

```text
Input File
    │
    ▼
Parser
    │
    ▼
Schema Validation
    │
    ▼
Normalization
    │
    ▼
Dataset Validation
    │
    ▼
ACE Dataset
    │
    ▼
Analysis
```

This prevents malformed input from reaching the algorithms.

---

# 12. Analysis Configuration

The `[analysis]` section controls execution behavior.

Example:

```toml
[analysis]
parallel = true
threads = 0
```

The default:

```toml
threads = 0
```

means I allow the runtime to determine an appropriate worker count.

A fixed worker count can be specified:

```toml
[analysis]
parallel = true
threads = 4
```

---

# 13. Parallel Processing

I enable parallel execution where an algorithm can safely benefit from it.

Example:

```toml
[analysis]
parallel = true
```

To disable parallel execution:

```toml
[analysis]
parallel = false
```

This gives me a deterministic baseline for performance comparisons and debugging.

---

# 14. Agreement Configuration

I configure agreement analysis through:

```toml
[analysis.agreement]
```

Example:

```toml
[analysis.agreement]
enabled = true
metrics = [
    "fleiss-kappa",
    "krippendorffs-alpha"
]
```

I keep agreement metrics independently selectable so that users do not have to execute algorithms they do not need.

---

# 15. Agreement Metrics

I support configuration for metrics such as:

```text
pairwise-agreement
cohens-kappa
fleiss-kappa
krippendorffs-alpha
```

Example:

```toml
[analysis.agreement]
enabled = true
metrics = [
    "cohens-kappa",
    "fleiss-kappa"
]
```

The implementation will validate metric names before execution.

Unknown metrics produce configuration errors.

---

# 16. Consensus Configuration

I configure consensus estimation through:

```toml
[analysis.consensus]
```

Example:

```toml
[analysis.consensus]
enabled = true
algorithm = "dawid-skene"
max_iterations = 25
tolerance = 0.0001
```

Supported consensus strategies will include:

```text
majority-vote
weighted-vote
dawid-skene
```

---

# 17. Majority Vote

A basic consensus configuration is:

```toml
[analysis.consensus]
enabled = true
algorithm = "majority-vote"
```

Majority vote requires no iterative configuration.

When multiple labels have the same vote count, I apply a deterministic tie-breaking rule rather than relying on hash-map iteration order.

---

# 18. Weighted Vote

Weighted voting can be configured as:

```toml
[analysis.consensus]
enabled = true
algorithm = "weighted-vote"
```

Annotator reliability values are derived from the configured quality/reliability pipeline or supplied through the appropriate dataset metadata.

---

# 19. Dawid-Skene

For Dawid-Skene:

```toml
[analysis.consensus]
enabled = true
algorithm = "dawid-skene"
max_iterations = 25
tolerance = 0.0001
```

`max_iterations` limits the number of optimization iterations.

`tolerance` controls convergence.

The algorithm stops when the change between iterations falls below the configured tolerance or when the maximum iteration count is reached.

---

# 20. Consensus Defaults

My default consensus configuration is intentionally conservative.

```toml
[analysis.consensus]
enabled = true
algorithm = "majority-vote"
max_iterations = 25
tolerance = 0.0001
```

I use majority vote as the default because it is easy to interpret and provides a strong baseline before applying more computationally expensive probabilistic models.

---

# 21. Quality Configuration

I configure quality analysis through:

```toml
[analysis.quality]
```

Example:

```toml
[analysis.quality]
enabled = true
min_annotations = 2
```

Quality analysis may use:

```text
disagreement
entropy
consensus confidence
annotator reliability
anomaly scores
```

---

# 22. Minimum Annotation Requirement

Some quality calculations are meaningless when an item has only one annotation.

I therefore allow:

```toml
[analysis.quality]
min_annotations = 2
```

Items below this threshold can be excluded from disagreement-based analysis.

---

# 23. Performance Configuration

I keep performance-related options separate from algorithm configuration.

Example:

```toml
[performance]
parallel = true
threads = 0
chunk_size = 4096
```

This lets me tune execution without changing statistical behavior.

---

# 24. Chunk Size

For workloads processed in batches, I can configure:

```toml
[performance]
chunk_size = 4096
```

Chunk size affects:

* memory locality
* parallel scheduling
* allocation behavior
* throughput

I will benchmark changes to this value rather than assuming a larger or smaller value is automatically faster.

---

# 25. Memory Limits

Where supported, I can configure a soft memory target:

```toml
[performance]
memory_limit_mb = 4096
```

This value is treated as an execution constraint rather than a guarantee that the operating system will terminate the process exactly at that limit.

If a specific operation cannot operate within the configured constraints, ACE reports an explicit error.

---

# 26. Output Configuration

I configure output through `[output]`.

Example:

```toml
[output]
format = "json"
directory = "./output"
overwrite = false
```

Supported formats will include:

```text
json
csv
html
```

The selected format controls serialization only.

It does not change the underlying analysis.

---

# 27. Output Directory

The default output directory is:

```toml
[output]
directory = "./output"
```

ACE creates the directory when necessary.

I avoid writing generated reports into the source directories.

---

# 28. Overwrite Protection

By default:

```toml
[output]
overwrite = false
```

ACE does not silently replace existing output files.

To explicitly allow replacement:

```toml
[output]
overwrite = true
```

This prevents accidental destruction of previous analysis results.

---

# 29. Report Configuration

HTML reporting is configured separately:

```toml
[report]
enabled = true
title = "ACE Annotation Quality Report"
```

The report may contain:

```text
dataset summary
agreement metrics
consensus results
annotator statistics
flagged items
quality scores
visualizations
```

---

# 30. Logging Configuration

I configure logging through:

```toml
[logging]
level = "info"
format = "pretty"
```

Supported levels:

```text
trace
debug
info
warn
error
```

For machine-readable environments:

```toml
[logging]
level = "info"
format = "json"
```

This makes ACE suitable for CI systems and centralized logging platforms.

---

# 31. Environment Variables

I support environment-variable overrides for deployment environments.

The naming convention is:

```text
ACE_<SECTION>_<OPTION>
```

Examples:

```text
ACE_ANALYSIS_PARALLEL
ACE_ANALYSIS_THREADS
ACE_OUTPUT_FORMAT
ACE_OUTPUT_DIRECTORY
ACE_LOGGING_LEVEL
```

Environment variables override configuration-file values.

---

# 32. Example Environment Override

Given:

```toml
[analysis]
threads = 4
```

I can override it temporarily:

```bash
ACE_ANALYSIS_THREADS=8 ace analyze data.csv
```

The configuration file remains unchanged.

This is useful for CI/CD and server deployments.

---

# 33. CLI Overrides

CLI arguments have the highest priority.

Example:

```bash
ace analyze \
    --config ace.toml \
    --threads 8 \
    --output-format html
```

The effective configuration becomes:

```text
CLI
 │
 ├── threads = 8
 └── output-format = html
```

while unspecified values continue to come from the configuration hierarchy.

---

# 34. Configuration Discovery

When no configuration file is explicitly provided, ACE searches for the default configuration location.

Example:

```text
./ace.toml
```

I can explicitly specify another configuration file:

```bash
ace analyze --config ./configs/production.toml
```

Explicit paths always take precedence over automatic discovery.

---

# 35. Configuration Profiles

I support separate configuration files for different environments.

Example:

```text
configs/
├── development.toml
├── benchmark.toml
├── production.toml
└── ci.toml
```

A production configuration might contain:

```toml
[analysis]
parallel = true
threads = 0

[output]
format = "json"

[logging]
level = "info"
format = "json"
```

A development configuration can prioritize debugging:

```toml
[analysis]
parallel = false
threads = 1

[logging]
level = "debug"
format = "pretty"
```

---

# 36. Benchmark Configuration

I keep benchmark settings separate from normal production configuration.

Example:

```toml
[benchmark]
seed = 42
items = 100000
annotators = 20
labels = 5
annotations_per_item = 5
```

Benchmark configuration belongs to the benchmark package and should not silently alter normal ACE analysis.

---

# 37. Configuration Validation

I validate the complete configuration before starting analysis.

The validation sequence is:

```text
Load Defaults
     │
     ▼
Load File
     │
     ▼
Apply Environment
     │
     ▼
Apply CLI
     │
     ▼
Validate
     │
     ├── Invalid → Error
     │
     ▼
Execute
```

This prevents partially configured jobs from running.

---

# 38. Invalid Configuration

Example:

```toml
[analysis.consensus]
algorithm = "unknown-algorithm"
```

ACE should report a clear error:

```text
Invalid consensus algorithm:
"unknown-algorithm"

Supported algorithms:
- majority-vote
- weighted-vote
- dawid-skene
```

I prefer actionable configuration errors over generic parser failures.

---

# 39. Numeric Validation

I validate numeric configuration values before execution.

For example:

```toml
[analysis.consensus]
max_iterations = 0
```

is invalid because an iterative consensus algorithm requires at least one iteration.

Similarly:

```toml
[analysis.consensus]
tolerance = -0.1
```

is invalid.

---

# 40. Configuration Schema

The configuration model will be represented by typed Rust structures.

Conceptually:

```rust
pub struct Config {
    pub dataset: DatasetConfig,
    pub input: InputConfig,
    pub analysis: AnalysisConfig,
    pub performance: PerformanceConfig,
    pub output: OutputConfig,
    pub report: ReportConfig,
    pub logging: LoggingConfig,
}
```

Each section has its own typed configuration structure.

---

# 41. Serde Integration

I use Serde for configuration deserialization.

Conceptually:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub dataset: DatasetConfig,
    pub input: InputConfig,
    pub analysis: AnalysisConfig,
    pub performance: PerformanceConfig,
    pub output: OutputConfig,
    pub report: ReportConfig,
    pub logging: LoggingConfig,
}
```

TOML parsing is handled at the configuration boundary.

The rest of the application receives validated Rust structures.

---

# 42. Configuration Separation

I keep configuration loading separate from business logic.

The architecture is:

```text
CLI
 │
 ▼
Config Loader
 │
 ▼
Config Validator
 │
 ▼
Validated Config
 │
 ├──────────────► Input
 │
 ├──────────────► Analysis
 │
 ├──────────────► Reporting
 │
 └──────────────► Logging
```

Algorithms should not parse TOML directly.

---

# 43. Secrets

I do not store secrets in normal ACE configuration files.

For example:

```text
API keys
database passwords
cloud credentials
private tokens
```

should not be committed to Git.

If ACE later requires credentials for external integrations, I will use environment variables or an external secret-management mechanism.

---

# 44. Configuration Files in Git

Safe configuration files can be committed:

```text
configs/
├── development.toml
├── benchmark.toml
└── production.example.toml
```

Machine-specific or secret-bearing configuration files should be excluded through `.gitignore`.

Example:

```text
production.local.toml
secrets.toml
```

---

# 45. Reproducibility

For reproducible analysis, I record the effective configuration alongside important output.

Example:

```text
output/
├── analysis.json
├── report.html
└── config.resolved.toml
```

The resolved configuration allows me to determine exactly which settings produced a result.

---

# 46. Resolved Configuration

The resolved configuration represents:

```text
defaults
    +
file
    +
environment
    +
CLI overrides
```

I can expose it through the CLI:

```bash
ace config show
```

This makes configuration debugging significantly easier.

---

# 47. Configuration Precedence Example

Suppose the default is:

```toml
threads = 4
```

The configuration file specifies:

```toml
threads = 6
```

The environment specifies:

```text
ACE_ANALYSIS_THREADS=8
```

The CLI specifies:

```bash
--threads 12
```

The final value is:

```text
12
```

because:

```text
CLI > Environment > File > Default
```

---

# 48. Recommended Project Configuration

I will maintain a project-level configuration similar to:

```toml
[dataset]
strict = true
allow_empty = false
duplicate_policy = "error"

[input]
format = "auto"

[analysis]
parallel = true
threads = 0

[analysis.agreement]
enabled = true
metrics = [
    "fleiss-kappa",
    "krippendorffs-alpha"
]

[analysis.consensus]
enabled = true
algorithm = "majority-vote"
max_iterations = 25
tolerance = 0.0001

[analysis.quality]
enabled = true
min_annotations = 2

[performance]
chunk_size = 4096

[output]
format = "json"
directory = "./output"
overwrite = false

[report]
enabled = true
title = "ACE Annotation Quality Report"

[logging]
level = "info"
format = "pretty"
```

This provides a sensible production-oriented baseline without forcing users to configure every option manually.

---

# 49. Configuration Testing

I test configuration at several levels.

### Parsing tests

I verify that valid TOML is parsed correctly.

### Validation tests

I verify that invalid values are rejected.

### Precedence tests

I verify:

```text
CLI > Environment > File > Defaults
```

### Serialization tests

I verify that configuration can be serialized and reloaded without unintended changes.

### Regression tests

I preserve important configuration behavior across releases.

---

# 50. Configuration Design Principles

I follow these rules throughout the implementation:

```text
1. Defaults must be safe.
2. Invalid configuration must fail early.
3. CLI overrides must be explicit.
4. Configuration must not contain business logic.
5. Algorithms must receive typed configuration.
6. Secrets must never be committed.
7. Configuration must be reproducible.
8. Configuration changes must be testable.
9. Unknown options should be rejected when strict parsing is enabled.
10. Performance settings must never silently change statistical semantics.
```

---

# 51. Final Configuration Flow

The complete runtime flow is:

```text
                 ┌──────────────────┐
                 │ Built-in Defaults│
                 └────────┬─────────┘
                          │
                          ▼
                 ┌──────────────────┐
                 │  TOML Config     │
                 └────────┬─────────┘
                          │
                          ▼
                 ┌──────────────────┐
                 │ Environment Vars │
                 └────────┬─────────┘
                          │
                          ▼
                 ┌──────────────────┐
                 │   CLI Overrides  │
                 └────────┬─────────┘
                          │
                          ▼
                 ┌──────────────────┐
                 │ Config Validation│
                 └────────┬─────────┘
                          │
                          ▼
                 ┌──────────────────┐
                 │ Validated Config │
                 └────────┬─────────┘
                          │
          ┌───────────────┼────────────────┐
          ▼               ▼                ▼
       Input           Analysis          Output
          │               │                │
          └───────────────┼────────────────┘
                          ▼
                    ACE Results
```

I use this configuration architecture to keep ACE predictable for local development, large-scale annotation processing, automated pipelines, benchmarking, and production deployments.
```
