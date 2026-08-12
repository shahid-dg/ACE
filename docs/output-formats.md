#

````markdown
# ACE Output Formats

## 1. Purpose

This document defines the output formats that I use to expose ACE analysis results.

My output layer converts the internal analysis results into formats suitable for:

- humans
- automated pipelines
- dashboards
- CI/CD systems
- data analysis
- archival
- downstream applications

The output architecture follows:

```text
ACE Analysis
     │
     ▼
Analysis Result
     │
     ▼
Output Serializer
     │
     ├── JSON
     ├── CSV
     └── HTML
````

The analysis engine remains independent of the output format.

---

# 2. Output Design

I keep the internal result model separate from serialization.

The intended flow is:

```text
Dataset
   │
   ▼
Analysis Engine
   │
   ▼
Canonical Analysis Result
   │
   ├──────────────┐
   ▼              ▼
JSON/CSV         HTML
```

This means changing the output format does not change how ACE calculates agreement, consensus, or quality scores.

---

# 3. Supported Output Formats

The initial ACE release supports:

```text
JSON
CSV
HTML
```

Each format serves a different purpose.

| Format | Primary Use                 |
| ------ | --------------------------- |
| JSON   | APIs, automation, pipelines |
| CSV    | Spreadsheets, data analysis |
| HTML   | Human-readable reports      |

---

# 4. JSON Output

JSON is the primary machine-readable output format.

Example:

```bash
ace analyze ./data/annotations.csv --output-format json
```

The result can be written to:

```text
output/results.json
```

JSON is designed to preserve the complete structure of the analysis result.

---

# 5. JSON Result Structure

A typical result follows this structure:

```json
{
  "schema_version": "1.0",
  "dataset": {
    "items": 1000,
    "annotations": 4200,
    "annotators": 12,
    "labels": 8
  },
  "agreement": {},
  "consensus": {},
  "quality": {},
  "flagged_items": []
}
```

The exact fields evolve with the ACE result model, but the structure remains organized around major analysis categories.

---

# 6. Schema Version

Every JSON result contains a schema version:

```json
{
  "schema_version": "1.0"
}
```

I use this to distinguish output-schema changes from ACE application versions.

This is important because downstream systems may consume ACE output automatically.

---

# 7. Dataset Summary

JSON contains a dataset summary.

Example:

```json
{
  "dataset": {
    "items": 1000,
    "annotations": 4200,
    "annotators": 12,
    "labels": 8
  }
}
```

This allows consumers to understand the scope of the analysis without processing the original dataset.

---

# 8. Agreement Results

Agreement results are grouped under:

```json
{
  "agreement": {
    "fleiss_kappa": 0.81,
    "krippendorffs_alpha": 0.78
  }
}
```

Only metrics requested by configuration need to appear in the result.

---

# 9. Consensus Results

Consensus information is represented separately.

Example:

```json
{
  "consensus": {
    "algorithm": "majority-vote",
    "items_processed": 1000
  }
}
```

Per-item consensus can be represented as:

```json
{
  "item_id": "image_001",
  "consensus_label": "cat",
  "confidence": 0.87
}
```

---

# 10. Quality Results

Quality analysis contains information about annotation reliability and suspicious items.

Example:

```json
{
  "quality": {
    "mean_score": 0.84,
    "flagged_items": 37
  }
}
```

The quality section can contain:

```text
item quality
annotator reliability
disagreement
entropy
confidence
anomaly scores
```

---

# 11. Flagged Items

Flagged items are one of the most important ACE outputs.

Example:

```json
{
  "flagged_items": [
    {
      "item_id": "image_00421",
      "reason": "high_disagreement",
      "score": 0.93
    }
  ]
}
```

Each flagged item should provide enough information for a data manager to investigate the annotation.

---

# 12. Flag Reasons

I use explicit machine-readable reason identifiers.

Examples:

```text
high_disagreement
low_consensus
low_confidence
annotator_outlier
duplicate_annotation
statistical_anomaly
```

Human-readable explanations can be included separately.

---

# 13. Annotator Results

JSON can contain per-annotator statistics.

Example:

```json
{
  "annotators": [
    {
      "annotator_id": "worker_01",
      "annotations": 840,
      "agreement": 0.91,
      "reliability": 0.94
    }
  ]
}
```

This makes it possible to identify systematic annotator behavior.

---

# 14. Annotator Bias

Where supported by the selected model, ACE can expose annotator-specific statistics.

Example:

```json
{
  "annotator_id": "worker_07",
  "label_distribution": {
    "cat": 0.81,
    "dog": 0.12,
    "bird": 0.07
  }
}
```

These statistics help distinguish genuine disagreement from systematic labeling tendencies.

---

# 15. JSON Pretty Printing

Human-readable JSON can be enabled:

```toml
[output.json]
pretty = true
```

Example:

```bash
ace analyze data.csv --output-format json --pretty
```

For large automated pipelines, compact JSON is preferable:

```toml
[output.json]
pretty = false
```

---

# 16. JSON Streaming

For large result sets, I design the JSON writer to support streaming where practical.

Instead of constructing the entire serialized document in memory:

```text
Analysis Result
      │
      ▼
Streaming Serializer
      │
      ▼
results.json
```

This reduces memory pressure for datasets containing millions of annotations or flagged items.

---

# 17. JSON Compatibility

JSON output must remain valid UTF-8.

Labels and identifiers containing Unicode characters are preserved.

Example:

```json
{
  "label": "猫"
}
```

No ASCII-only restriction is imposed on annotation labels.

---

# 18. CSV Output

CSV is intended for tabular analysis.

Example:

```bash
ace analyze ./data/annotations.csv --output-format csv
```

ACE can generate separate CSV files for different result categories.

Example:

```text
output/
├── summary.csv
├── item-results.csv
├── annotator-results.csv
└── flagged-items.csv
```

---

# 19. Summary CSV

Example:

```csv
metric,value
items,1000
annotations,4200
annotators,12
labels,8
fleiss_kappa,0.81
krippendorffs_alpha,0.78
```

This format is convenient for spreadsheets and simple scripts.

---

# 20. Item Results CSV

Example:

```csv
item_id,consensus_label,confidence,quality_score,flagged
image_001,cat,0.91,0.94,false
image_002,dog,0.87,0.88,false
image_003,bird,0.52,0.41,true
```

Each row represents one analyzed item.

---

# 21. Flagged Items CSV

Example:

```csv
item_id,reason,score
image_003,high_disagreement,0.93
image_041,low_confidence,0.88
image_127,annotator_outlier,0.82
```

This file can be directly imported into a review workflow.

---

# 22. Annotator Results CSV

Example:

```csv
annotator_id,annotations,agreement,reliability
worker_01,840,0.91,0.94
worker_02,812,0.87,0.89
worker_03,901,0.76,0.72
```

This allows data managers to analyze annotator performance using standard spreadsheet or BI tooling.

---

# 23. CSV Delimiter

Comma is the default delimiter:

```toml
[output.csv]
delimiter = ","
```

I can support alternative delimiters when required:

```toml
[output.csv]
delimiter = ";"
```

The output serializer ensures fields are correctly quoted when necessary.

---

# 24. CSV Quoting

Values containing commas, quotes, or line breaks are quoted according to standard CSV rules.

For example:

```csv
item_id,label
image_001,"animal, cat"
```

This ensures output remains parseable by standard CSV readers.

---

# 25. HTML Output

HTML is the primary human-facing output.

Example:

```bash
ace analyze ./data/annotations.csv --output-format html
```

The generated report may contain:

```text
output/
└── report.html
```

---

# 26. HTML Report Structure

The report is organized into sections:

```text
ACE Annotation Quality Report
│
├── Executive Summary
│
├── Dataset Overview
│
├── Agreement Analysis
│
├── Consensus Analysis
│
├── Annotator Analysis
│
├── Quality Analysis
│
├── Flagged Items
│
└── Configuration
```

This gives users a progression from high-level results to detailed investigation.

---

# 27. Executive Summary

The report begins with the most important metrics.

Example:

```text
Dataset
1,000 items
4,200 annotations
12 annotators

Agreement
Fleiss' κ = 0.81

Flagged
37 items
```

The purpose is to let a reviewer understand dataset quality without reading the entire report.

---

# 28. Dataset Overview

The dataset section contains:

```text
items
annotations
annotators
unique labels
annotations per item
annotations per annotator
```

Where appropriate, ACE can include distributions and summary statistics.

---

# 29. Agreement Visualization

HTML reports can visualize agreement metrics.

Example conceptual representation:

```text
Fleiss' Kappa
████████████████░░░░ 0.81

Krippendorff's Alpha
███████████████░░░░░ 0.78
```

The final implementation can use browser-native HTML/CSS and JavaScript where required.

---

# 30. Annotator Visualization

The report can display annotator-level performance.

Example:

```text
worker_01   ██████████████████ 0.94
worker_02   █████████████████  0.89
worker_03   ██████████████     0.72
```

This provides an immediate visual indication of potential outliers.

---

# 31. Label Distribution

I can display label distributions.

Example:

```text
cat     42%
dog     31%
bird    18%
other    9%
```

This is useful because severe class imbalance can influence agreement metrics and consensus behavior.

---

# 32. Flagged Item Table

The HTML report includes a searchable flagged-item table.

Example:

| Item      | Reason            | Score | Consensus |
| --------- | ----------------- | ----: | --------- |
| image_003 | High disagreement |  0.93 | cat       |
| image_041 | Low confidence    |  0.88 | dog       |
| image_127 | Annotator outlier |  0.82 | bird      |

The table allows reviewers to prioritize manual investigation.

---

# 33. Filtering

I design the HTML report to support filtering by:

```text
reason
score
annotator
label
item
```

For example:

```text
Show only:
High disagreement
Score > 0.80
```

This makes large reports practical to review.

---

# 34. Sorting

Tables can be sorted by:

```text
item ID
quality score
confidence
disagreement
annotator
```

Sorting allows reviewers to move directly to the most suspicious records.

---

# 35. Search

The flagged-item section supports text search.

A reviewer can search:

```text
image_00421
worker_07
cat
```

without manually scanning the complete report.

---

# 36. Static HTML

The default HTML report is designed to be usable as a static file.

Example:

```bash
file://.../report.html
```

No backend server should be required for normal report viewing.

This makes the report easy to:

```text
archive
email
upload
attach to tickets
store as CI artifacts
```

---

# 37. Self-Contained Reports

Where practical, ACE generates self-contained HTML.

The report can contain:

```text
HTML
CSS
JavaScript
data required for visualization
```

inside the generated report.

This avoids external runtime dependencies.

---

# 38. External Assets

If a user explicitly enables external assets, the report may reference external resources.

However, the default report should avoid depending on third-party CDNs.

This improves:

```text
offline usability
reproducibility
security
long-term archival
```

---

# 39. Report Configuration

HTML reporting is controlled through:

```toml
[report]
enabled = true
title = "ACE Annotation Quality Report"
```

Additional options can include:

```toml
[report]
include_annotators = true
include_distributions = true
include_flagged_items = true
self_contained = true
```

---

# 40. Output Directory Structure

For a complete analysis, I use:

```text
output/
├── results.json
├── summary.csv
├── item-results.csv
├── annotator-results.csv
├── flagged-items.csv
└── report.html
```

Users can select only the formats they need.

---

# 41. Output Selection

I support explicit output selection.

JSON only:

```bash
ace analyze data.csv --output-format json
```

HTML only:

```bash
ace analyze data.csv --output-format html
```

CSV only:

```bash
ace analyze data.csv --output-format csv
```

Multiple formats can be generated when requested:

```bash
ace analyze data.csv \
    --output-format json \
    --output-format html
```

---

# 42. Output Naming

I use predictable filenames.

Examples:

```text
results.json
report.html
summary.csv
item-results.csv
annotator-results.csv
flagged-items.csv
```

Predictable names make the output easy to integrate into automation.

---

# 43. Output Overwrite Protection

By default:

```toml
[output]
overwrite = false
```

ACE refuses to overwrite existing files.

To explicitly enable overwriting:

```toml
[output]
overwrite = true
```

This prevents accidental loss of previous analysis results.

---

# 44. Output Atomicity

For important output files, I design the writer to avoid leaving partially written results when possible.

The intended process is:

```text
Generate Temporary File
        │
        ▼
Flush
        │
        ▼
Validate / Finalize
        │
        ▼
Atomic Rename
        │
        ▼
Final Output
```

This is particularly useful when ACE runs inside automated pipelines.

---

# 45. Failed Output

If serialization fails, ACE reports the failure and avoids presenting an incomplete output file as a successful result.

Example:

```text
Failed to write JSON output:
permission denied: ./output/results.json
```

The CLI should return a non-zero exit status.

---

# 46. Output Errors

I use structured output errors.

Examples:

```text
OutputDirectoryError
FileExists
PermissionDenied
SerializationError
UnsupportedOutputFormat
WriteError
InvalidOutputConfiguration
```

Errors should identify the affected output where possible.

---

# 47. Machine-Readable Output

For CI/CD and automation, JSON is the preferred format.

Example:

```bash
ace analyze data.csv \
    --output-format json \
    --output ./artifacts/results.json
```

A pipeline can then process:

```text
agreement
quality
flagged_items
annotator_statistics
```

without parsing human-readable text.

---

# 48. Exit Codes

Output generation is independent from process success.

A successful analysis should return:

```text
exit code 0
```

Configuration, input, analysis, or output failures should return a non-zero exit code.

This allows:

```bash
ace analyze data.csv && echo "Analysis succeeded"
```

to work correctly in automation.

---

# 49. Quality-Gate Output

ACE can support CI quality gates.

Example configuration:

```toml
[quality_gate]
enabled = true
min_agreement = 0.80
max_flagged_ratio = 0.05
```

A dataset that fails the configured thresholds can cause ACE to return a non-zero exit code.

Example:

```text
Agreement:       0.76
Required:        0.80

Quality gate: FAILED
```

This makes ACE useful beyond manual analysis.

---

# 50. JSON Quality-Gate Result

The machine-readable result can expose:

```json
{
  "quality_gate": {
    "passed": false,
    "checks": [
      {
        "name": "minimum_agreement",
        "passed": false,
        "actual": 0.76,
        "required": 0.80
      }
    ]
  }
}
```

This allows CI systems to determine exactly why a dataset failed.

---

# 51. Output Reproducibility

Output should be deterministic where the underlying analysis is deterministic.

Given:

```text
same dataset
+
same configuration
+
same ACE version
```

I aim to generate equivalent results.

Where timestamps or execution metadata are included, they should be clearly separated from analytical values.

---

# 52. Execution Metadata

I can include execution metadata:

```json
{
  "metadata": {
    "ace_version": "0.1.0",
    "schema_version": "1.0",
    "generated_at": "2026-08-12T10:30:00Z"
  }
}
```

This helps with debugging and auditability.

Execution timestamps should not alter analytical results.

---

# 53. Configuration Metadata

The final output can include the effective configuration.

Example:

```json
{
  "configuration": {
    "analysis": {
      "parallel": true,
      "threads": 8
    },
    "consensus": {
      "algorithm": "majority-vote"
    }
  }
}
```

This allows an output file to explain how it was produced.

---

# 54. Sensitive Data

ACE output may contain the same identifiers and labels present in the input dataset.

Therefore I treat generated reports as potentially sensitive.

The output layer should not:

```text
upload results
send results externally
embed credentials
expose environment secrets
```

unless an explicitly configured integration requires it.

---

# 55. Large Output Sets

Large datasets can produce extremely large result files.

I therefore allow users to control whether detailed per-item results are generated.

Example:

```toml
[output]
include_item_results = true
include_annotator_results = true
include_flagged_items = true
```

For summary-only workloads:

```toml
[output]
include_item_results = false
include_annotator_results = false
include_flagged_items = true
```

This can significantly reduce output size.

---

# 56. Output Compression

Compression can be added for machine-readable output.

Example:

```text
results.json.gz
```

This is particularly useful for large datasets.

Compression belongs to the output transport layer and does not change the JSON schema.

---

# 57. Output Schema Stability

I treat the output schema as a public interface.

Changes to:

```text
field names
field types
required fields
nested structure
```

must be treated carefully.

Breaking schema changes require a schema-version change.

---

# 58. Backward Compatibility

Where practical, I maintain compatibility between minor ACE versions.

For example:

```text
schema 1.0
schema 1.1
```

may add optional fields without breaking existing consumers.

A future:

```text
schema 2.0
```

may introduce breaking structural changes.

---

# 59. Output Testing

I test each serializer independently.

Tests include:

```text
empty result
single item
large result
Unicode labels
special characters
missing optional fields
flagged items
multiple annotators
multiple metrics
```

I also verify that generated files can be read by standard parsers.

---

# 60. JSON Tests

For JSON I verify:

```text
valid JSON syntax
correct field names
correct numeric values
correct schema version
correct nested structures
UTF-8 preservation
```

I also use snapshot-style tests where appropriate to detect accidental schema changes.

---

# 61. CSV Tests

For CSV I verify:

```text
headers
row counts
quoting
escaping
Unicode
empty values
delimiter behavior
```

I also parse generated CSV files again during testing to ensure they are actually consumable.

---

# 62. HTML Tests

For HTML I verify:

```text
valid document structure
required sections
flagged-item rendering
metric rendering
Unicode preservation
static asset availability
```

Where possible, generated reports should be tested without requiring an internet connection.

---

# 63. Output Module Structure

I keep serializers isolated from the analysis engine.

The intended structure is:

```text
crates/
└── ace-report/
    └── src/
        ├── lib.rs
        ├── error.rs
        ├── json.rs
        ├── csv.rs
        ├── html.rs
        ├── schema.rs
        └── templates/
```

The exact crate structure may evolve as implementation progresses.

---

# 64. Output Trait

I use a common abstraction for serializers.

Conceptually:

```rust
pub trait OutputWriter {
    fn write(
        &self,
        result: &AnalysisResult,
        destination: &std::path::Path,
    ) -> Result<(), OutputError>;
}
```

Each format implements this interface.

For example:

```text
JsonWriter
CsvWriter
HtmlWriter
```

This prevents format-specific logic from spreading through the CLI.

---

# 65. Result Model Separation

The analysis engine produces:

```rust
AnalysisResult
```

The output layer consumes it.

The relationship is:

```text
ace-core
    │
    ▼
AnalysisResult
    │
    ▼
ace-report
    │
    ├── JsonWriter
    ├── CsvWriter
    └── HtmlWriter
```

The output layer does not recalculate metrics.

---

# 66. Performance Considerations

Output generation can become a bottleneck for very large datasets.

I therefore prioritize:

```text
streaming serialization
buffered writes
minimal allocations
column projection
lazy HTML rendering
```

where appropriate.

The goal is to prevent serialization from dominating total analysis time.

---

# 67. Benchmarking Output

I benchmark output generation independently from algorithm performance.

For example:

```text
Input Analysis
      │
      ▼
Analysis Time
      │
      ▼
Result Serialization
      │
      ▼
Output Time
```

This lets me distinguish:

```text
algorithm bottleneck
```

from:

```text
serialization bottleneck
```

---

# 68. Recommended Production Output

For automated pipelines, I recommend:

```toml
[output]
format = "json"
overwrite = false
```

For human review:

```toml
[output]
format = "html"
```

For spreadsheet-based investigation:

```toml
[output]
format = "csv"
```

For complete reporting:

```text
JSON + CSV + HTML
```

---

# 69. Complete Example

A complete analysis command might be:

```bash
ace analyze \
    --config ./configs/production.toml \
    ./data/annotations.parquet \
    --output ./artifacts/
```

The resulting directory can contain:

```text
artifacts/
├── results.json
├── summary.csv
├── item-results.csv
├── annotator-results.csv
├── flagged-items.csv
└── report.html
```

---

# 70. Final Output Architecture

The complete output pipeline is:

```text
                       ACE Analysis
                            │
                            ▼
                    AnalysisResult
                            │
                ┌───────────┼───────────┐
                │           │           │
                ▼           ▼           ▼
              JSON         CSV         HTML
                │           │           │
                ▼           ▼           ▼
           Automation    Analysis    Human Review
                │           │           │
                └───────────┼───────────┘
                            ▼
                         Artifacts
```

I keep the output layer independent, deterministic, testable, and suitable for both human investigation and automated data-quality pipelines.

```
```
