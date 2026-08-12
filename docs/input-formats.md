#

````markdown
# ACE Input Formats

## 1. Purpose

This document defines the input formats that I support when importing annotation data into ACE.

My input layer converts platform-specific annotation exports into the canonical ACE data model.

The core principle is:

```text
External Format
      │
      ▼
Input Adapter
      │
      ▼
Validation
      │
      ▼
Normalization
      │
      ▼
ACE Dataset
````

Once data has been normalized, the analysis engine does not need to know whether the original data came from CVAT, Label Studio, CSV, JSON, or another source.

---

# 2. Supported Formats

I organize input support into two categories.

### Tabular formats

```text
CSV
```

### Structured formats

```text
JSON
Parquet
```

I will prioritize CSV and JSON first because they are the most practical interchange formats for annotation exports.

Parquet provides an optimized path for larger datasets.

---

# 3. Input Architecture

I keep format-specific parsing outside the statistical engine.

The intended structure is:

```text
crates/
├── ace-core/
│   └── src/
│
└── ace-io/
    └── src/
        ├── lib.rs
        ├── error.rs
        ├── format.rs
        ├── csv.rs
        ├── json.rs
        └── parquet.rs
```

`ace-core` owns the canonical data model.

`ace-io` owns external format handling.

---

# 4. Format Detection

I support explicit format selection:

```toml
[input]
format = "csv"
```

I also support automatic detection:

```toml
[input]
format = "auto"
```

When automatic detection is enabled, I primarily use the input file extension.

Examples:

```text
annotations.csv  → CSV
annotations.json → JSON
annotations.parquet → Parquet
```

I do not rely exclusively on file extensions when a format can be safely identified using file characteristics.

---

# 5. Explicit Format Selection

Users can explicitly specify the format through the CLI:

```bash
ace analyze \
    --input ./data/annotations.csv \
    --format csv
```

This is useful when:

* a file has an unusual extension
* the extension is missing
* multiple formats share an extension
* automated pipelines already know the input format

Explicit configuration takes precedence over automatic detection.

---

# 6. Canonical Input Schema

Regardless of the source format, ACE normalizes records into the following logical structure:

```text
item_id
annotator_id
label
```

Optional fields can include:

```text
timestamp
confidence
source
source_id
metadata
```

The minimum valid annotation requires:

```text
item_id
annotator_id
label
```

---

# 7. Canonical Record

Conceptually, each input record becomes:

```rust
pub struct RawAnnotation {
    pub item_id: String,
    pub annotator_id: String,
    pub label: String,
    pub metadata: Option<AnnotationMetadata>,
}
```

The input layer converts this into the strongly typed ACE representation:

```rust
Annotation {
    item_id,
    annotator_id,
    label,
}
```

---

# 8. CSV

CSV is my primary simple interchange format.

A minimal ACE-compatible CSV file looks like:

```csv
item_id,annotator_id,label
image_001,worker_01,cat
image_001,worker_02,cat
image_001,worker_03,dog
image_002,worker_01,dog
image_002,worker_02,dog
image_003,worker_01,bird
```

This structure is intentionally simple.

---

# 9. CSV Required Columns

The minimum CSV schema is:

```text
item_id
annotator_id
label
```

Column order does not matter.

For example, this is also valid:

```csv
label,item_id,annotator_id
cat,image_001,worker_01
cat,image_001,worker_02
dog,image_001,worker_03
```

The parser identifies columns by their names rather than their positions.

---

# 10. CSV Optional Columns

I allow additional columns to be preserved as metadata.

Example:

```csv
item_id,annotator_id,label,confidence,timestamp
image_001,worker_01,cat,0.95,2026-08-12T10:30:00Z
```

The required fields remain:

```text
item_id
annotator_id
label
```

Additional fields do not automatically affect statistical analysis.

---

# 11. CSV Header Validation

I require a header row by default.

For example:

```csv
item_id,annotator_id,label
```

If required columns are missing:

```csv
item,worker,class
```

ACE reports a schema error instead of guessing column mappings.

---

# 12. CSV Column Mapping

For datasets using different column names, I support explicit mappings.

Example:

```toml
[input.csv.columns]
item_id = "task_id"
annotator_id = "worker_id"
label = "category"
```

This allows:

```csv
task_id,worker_id,category
123,worker_a,cat
124,worker_b,dog
```

to become:

```text
item_id     = 123
annotator   = worker_a
label       = cat
```

---

# 13. CSV Delimiters

Comma-separated input is the default:

```toml
[input.csv]
delimiter = ","
```

I can also support common delimiters such as:

```text
;
\t
|
```

Example:

```toml
[input.csv]
delimiter = ";"
```

I will validate the configured delimiter before parsing.

---

# 14. CSV Quoting

I support standard CSV quoting.

Example:

```csv
item_id,annotator_id,label
"image,001","worker_01","cat"
```

The parser must treat:

```text
image,001
```

as one field rather than two fields.

---

# 15. CSV Escaping

Standard CSV escaping is supported.

Example:

```csv
item_id,annotator_id,label
image_001,worker_01,"animal, cat"
```

The resulting label is:

```text
animal, cat
```

---

# 16. CSV Whitespace

I do not silently alter identifiers unless normalization is explicitly configured.

For example:

```text
"worker_01 "
```

should not automatically become:

```text
"worker_01"
```

because changing identifiers can accidentally merge distinct records.

If whitespace normalization is enabled, it will occur during the normalization stage and will be documented in the resulting configuration.

---

# 17. CSV Empty Values

A missing required value is invalid.

For example:

```csv
item_id,annotator_id,label
image_001,worker_01,
```

produces an input validation error.

I do not interpret an empty label as a special category.

---

# 18. CSV Example

Complete example:

```csv
item_id,annotator_id,label,confidence
image_001,worker_01,cat,0.91
image_001,worker_02,cat,0.87
image_001,worker_03,dog,0.72
image_002,worker_01,dog,0.95
image_002,worker_02,dog,0.93
```

The resulting canonical dataset contains five annotations.

---

# 19. JSON

JSON is useful for structured annotation exports.

I support both:

```text
array-based JSON
```

and, where explicitly configured:

```text
object-based JSON
```

The preferred representation is an array of annotation records.

---

# 20. JSON Array Format

Example:

```json
[
  {
    "item_id": "image_001",
    "annotator_id": "worker_01",
    "label": "cat"
  },
  {
    "item_id": "image_001",
    "annotator_id": "worker_02",
    "label": "cat"
  },
  {
    "item_id": "image_001",
    "annotator_id": "worker_03",
    "label": "dog"
  }
]
```

Each object represents one annotation.

---

# 21. JSON Metadata

JSON allows additional structured metadata.

Example:

```json
[
  {
    "item_id": "image_001",
    "annotator_id": "worker_01",
    "label": "cat",
    "confidence": 0.95,
    "source": "label-studio"
  }
]
```

ACE preserves supported metadata without allowing it to silently influence analysis.

---

# 22. JSON Nested Metadata

Structured metadata can be preserved where supported.

Example:

```json
{
  "item_id": "image_001",
  "annotator_id": "worker_01",
  "label": "cat",
  "metadata": {
    "confidence": 0.95,
    "reviewed": true
  }
}
```

Nested metadata belongs to the metadata layer rather than the statistical core.

---

# 23. JSON Object Format

I can support a wrapped dataset representation:

```json
{
  "annotations": [
    {
      "item_id": "image_001",
      "annotator_id": "worker_01",
      "label": "cat"
    },
    {
      "item_id": "image_001",
      "annotator_id": "worker_02",
      "label": "dog"
    }
  ]
}
```

When using this format, the input adapter identifies the configured annotation collection.

---

# 24. JSON Configuration

For wrapped JSON structures, I can configure the annotation path:

```toml
[input.json]
annotations_path = "annotations"
```

For nested data:

```json
{
  "project": {
    "results": {
      "annotations": []
    }
  }
}
```

the configured path can be:

```toml
[input.json]
annotations_path = "project.results.annotations"
```

---

# 25. JSON Schema Validation

I validate required fields before normalization.

A valid record must contain:

```text
item_id
annotator_id
label
```

Invalid:

```json
{
  "item_id": "image_001",
  "label": "cat"
}
```

because `annotator_id` is missing.

---

# 26. JSON Null Values

Required fields cannot be null.

Invalid:

```json
{
  "item_id": "image_001",
  "annotator_id": null,
  "label": "cat"
}
```

ACE returns a validation error rather than converting null into a string.

---

# 27. JSON Type Validation

Required identifiers must resolve to valid identifier values.

For example:

```json
{
  "item_id": 1001,
  "annotator_id": "worker_01",
  "label": "cat"
}
```

can be supported if the adapter explicitly permits scalar identifiers and converts them deterministically.

Arbitrary objects and arrays are not accepted as identifiers.

---

# 28. Parquet

Parquet provides an efficient columnar format for large annotation datasets.

I use it primarily for:

```text
large-scale datasets
batch processing
data-engineering pipelines
high-volume annotation exports
```

A minimal Parquet schema contains:

```text
item_id
annotator_id
label
```

Optional columns can contain metadata.

---

# 29. Parquet Column Mapping

I support explicit mappings when the Parquet schema uses different names.

Example:

```toml
[input.parquet.columns]
item_id = "task_id"
annotator_id = "worker"
label = "class"
```

This keeps the canonical ACE model independent of the upstream schema.

---

# 30. Parquet Type Requirements

I prefer UTF-8-compatible string columns for:

```text
item_id
annotator_id
label
```

Numeric identifiers may be converted when explicitly supported by the adapter.

Complex nested types should be handled as metadata rather than silently converted into labels or identifiers.

---

# 31. Large Parquet Datasets

For large Parquet files, I avoid loading unnecessary columns.

If the analysis only requires:

```text
item_id
annotator_id
label
```

I should read only those columns where the Parquet implementation supports projection.

This reduces:

```text
disk I/O
memory usage
deserialization cost
```

---

# 32. Streaming Input

For sufficiently large datasets, I design the input layer to support streaming or batch processing.

The intended pipeline is:

```text
Input
 │
 ▼
Batch
 │
 ▼
Validate
 │
 ▼
Normalize
 │
 ▼
Dataset Builder
 │
 ▼
Next Batch
```

This prevents the parser itself from requiring the entire source file to fit in memory.

---

# 33. Streaming Limitations

Not every analysis algorithm can operate directly on a stream.

For example, some agreement and consensus algorithms require global information.

Therefore I distinguish between:

```text
streamable ingestion
```

and:

```text
streamable analysis
```

The input layer may stream data into an indexed or materialized dataset before analysis begins.

---

# 34. Input Size

I do not impose an arbitrary hard input-size limit.

The practical limit depends on:

```text
available memory
dataset representation
algorithm
annotation count
indexing strategy
```

For very large datasets, I recommend Parquet and memory-efficient internal representations.

---

# 35. Duplicate Handling

Input adapters detect potential duplicates.

Example:

```text
item_id       annotator_id       label
image_001     worker_01          cat
image_001     worker_01          dog
```

This produces two annotations for the same logical:

```text
item + annotator
```

pair.

The configured duplicate policy determines whether ACE:

```text
rejects
keeps first
keeps last
```

I do not silently treat conflicting duplicates as independent votes.

---

# 36. External Platform Exports

Platform-specific exports are handled through adapters.

The architecture is:

```text
CVAT Export
     │
     ▼
CVAT Adapter
     │
     ▼
Raw Annotation
     │
     ▼
Canonical Dataset

Label Studio Export
     │
     ▼
Label Studio Adapter
     │
     ▼
Raw Annotation
     │
     ▼
Canonical Dataset
```

The core algorithms remain platform-independent.

---

# 37. Adapter Design

Each adapter follows the same conceptual interface:

```rust
pub trait AnnotationReader {
    fn read(
        &mut self,
        input: &mut dyn std::io::Read,
    ) -> Result<AnnotationBatch, InputError>;
}
```

The exact interface may evolve as streaming and batch processing are implemented.

The important requirement is that every adapter produces the same normalized representation.

---

# 38. Format Registry

I can register supported input formats through a central registry.

Conceptually:

```rust
pub enum InputFormat {
    Csv,
    Json,
    Parquet,
    Auto,
}
```

The registry maps:

```text
format
   │
   ▼
reader implementation
```

This prevents format detection logic from being duplicated across the application.

---

# 39. Input Errors

I use structured input errors.

Examples include:

```text
UnsupportedFormat
FileNotFound
IoError
InvalidEncoding
MissingColumn
InvalidRecord
InvalidIdentifier
InvalidLabel
DuplicateAnnotation
SchemaMismatch
ParseError
```

Errors should contain enough context to locate the problem.

---

# 40. Error Context

For CSV input, an error should identify the row where possible.

Example:

```text
Failed to parse annotation at row 1842:
missing required field "annotator_id"
```

For JSON:

```text
Failed to parse annotation at array index 1842:
field "label" must not be null
```

For Parquet:

```text
Invalid column "annotator_id":
expected string-compatible type
```

This makes large dataset failures actionable.

---

# 41. Fail-Fast vs Error Collection

I support configurable validation behavior.

Strict mode:

```toml
[dataset]
strict = true
```

stops processing on the first fatal validation failure.

A future tolerant mode can collect multiple invalid records and report them together.

The tolerant mode must never silently discard invalid annotations.

---

# 42. Input Normalization

Normalization converts external values into ACE-compatible values.

The process is:

```text
Raw Record
    │
    ▼
Validate
    │
    ▼
Normalize
    │
    ├── Item ID
    ├── Annotator ID
    └── Label
    │
    ▼
Canonical Annotation
```

Normalization rules are explicit and deterministic.

---

# 43. Identifier Normalization

I avoid destructive normalization by default.

For example, I do not automatically:

```text
lowercase identifiers
remove punctuation
trim arbitrary characters
convert spaces
```

because these operations can change identity.

If normalization is enabled, it is controlled through configuration.

---

# 44. Label Normalization

Labels may optionally be normalized.

For example:

```text
Cat
cat
CAT
```

can be treated as distinct values by default.

If case-insensitive labels are explicitly required:

```toml
[input.normalization]
label_case = "lower"
```

they become:

```text
cat
cat
cat
```

I only enable this when the dataset semantics justify it.

---

# 45. Unicode

I preserve valid UTF-8 labels and identifiers.

For example:

```text
人
猫
voiture
café
```

are valid values.

I do not assume that labels are ASCII-only.

Unicode normalization, when required, must be explicit because different normalization forms can affect exact string equality.

---

# 46. Timestamps

If an input format contains timestamps, I preserve them as metadata.

Example:

```json
{
  "item_id": "image_001",
  "annotator_id": "worker_01",
  "label": "cat",
  "timestamp": "2026-08-12T10:30:00Z"
}
```

Timestamps do not influence agreement or consensus unless a future analysis explicitly uses them.

---

# 47. Confidence Values

Confidence values can be imported when present.

Example:

```json
{
  "item_id": "image_001",
  "annotator_id": "worker_01",
  "label": "cat",
  "confidence": 0.94
}
```

I validate confidence values according to their declared semantics.

For probabilities:

```text
0 <= confidence <= 1
```

Invalid values are rejected unless an explicit alternate scale is configured.

---

# 48. Source Information

I preserve source information where available.

Example:

```json
{
  "item_id": "image_001",
  "annotator_id": "worker_01",
  "label": "cat",
  "source": "label-studio",
  "source_id": "annotation-91827"
}
```

This allows ACE reports to trace flagged annotations back to their originating platform or record.

---

# 49. Multiple Input Files

I support processing multiple compatible input files.

Example:

```bash
ace analyze \
    ./data/batch-001.csv \
    ./data/batch-002.csv \
    ./data/batch-003.csv
```

The files must resolve to compatible schemas.

ACE combines their normalized records into the analysis dataset.

---

# 50. Directory Input

I can support directory-based ingestion:

```bash
ace analyze ./data/annotations/
```

The input layer discovers supported files according to the configured formats.

For example:

```text
data/annotations/
├── batch-001.csv
├── batch-002.csv
└── batch-003.csv
```

All discovered files are normalized into one dataset.

---

# 51. Schema Consistency

When multiple files are loaded, required fields must remain compatible.

For example:

```text
batch-001.csv
item_id,annotator_id,label

batch-002.csv
item_id,annotator_id,label
```

is valid.

But:

```text
batch-001.csv
item_id,annotator_id,label

batch-002.csv
task_id,worker,class
```

requires an explicit column mapping before the files can be combined.

---

# 52. Input Pipeline Example

A complete CSV workflow looks like:

```text
annotations.csv
      │
      ▼
CSV Reader
      │
      ▼
Header Detection
      │
      ▼
Column Mapping
      │
      ▼
Row Parsing
      │
      ▼
Record Validation
      │
      ▼
Normalization
      │
      ▼
Duplicate Handling
      │
      ▼
Dataset Builder
      │
      ▼
Validated ACE Dataset
```

---

# 53. CLI Examples

CSV:

```bash
ace analyze ./annotations.csv
```

JSON:

```bash
ace analyze ./annotations.json --format json
```

Parquet:

```bash
ace analyze ./annotations.parquet --format parquet
```

Explicit configuration:

```bash
ace analyze \
    --config ./configs/production.toml \
    ./annotations.parquet
```

---

# 54. Configuration Example

A complete input configuration can look like:

```toml
[input]
format = "csv"
path = "./data/annotations.csv"
encoding = "utf-8"

[input.csv]
delimiter = ","

[input.csv.columns]
item_id = "item_id"
annotator_id = "annotator_id"
label = "label"

[input.normalization]
label_case = "preserve"
```

This makes the input behavior explicit and reproducible.

---

# 55. Security Considerations

I treat annotation exports as untrusted input.

The input layer must therefore:

```text
validate paths
avoid unsafe deserialization
validate numeric values
validate strings
limit pathological allocations
handle malformed records safely
avoid executing input content
```

JSON, CSV, and Parquet data are data only.

They must never be interpreted as executable instructions.

---

# 56. Resource Protection

Large or malicious input can attempt to exhaust system resources.

I therefore design the input layer to support limits such as:

```toml
[input.limits]
max_file_size_mb = 4096
max_record_size_mb = 16
max_metadata_size_mb = 64
```

These limits can be adjusted for legitimate large datasets.

---

# 57. Deterministic Ingestion

Given:

```text
same input
+
same configuration
+
same ACE version
```

I aim to produce the same canonical dataset.

This is important for:

* debugging
* benchmarking
* reproducible research
* CI
* regression testing

---

# 58. Input Testing

I test every input adapter against:

```text
valid input
empty input
missing fields
invalid fields
duplicate records
Unicode values
large records
malformed records
unexpected columns
incorrect data types
```

For each supported format, I maintain representative fixtures.

---

# 59. Input Fixture Structure

I will maintain fixtures separately from production code.

Example:

```text
tests/
└── fixtures/
    ├── csv/
    │   ├── valid.csv
    │   ├── duplicate.csv
    │   ├── missing_column.csv
    │   └── malformed.csv
    │
    ├── json/
    │   ├── valid.json
    │   ├── nested.json
    │   └── malformed.json
    │
    └── parquet/
        ├── valid.parquet
        └── invalid.parquet
```

Fixtures provide deterministic regression coverage.

---

# 60. Input Compatibility Contract

Every input adapter must guarantee the following:

```text
1. Required fields are identified correctly.
2. Invalid records are reported.
3. Valid records become canonical annotations.
4. Metadata is preserved where supported.
5. Duplicate handling follows configuration.
6. No platform-specific structures leak into ace-core.
7. Errors contain useful context.
8. Processing is deterministic.
```

---

# 61. Future Formats

The architecture allows additional formats to be added without modifying the analysis engine.

Potential future adapters include:

```text
Arrow
NDJSON
XML
database queries
REST APIs
platform-specific exports
```

A new format should only need to implement the input boundary and produce the existing canonical model.

---

# 62. Format Support Priority

I will implement input support in this order:

```text
1. CSV
2. JSON
3. Parquet
4. Platform-specific adapters
```

CSV and JSON provide the broadest compatibility for the initial ACE release.

Parquet becomes particularly valuable when ACE is used against large data-engineering workloads.

---

# 63. Design Principle

I keep the input layer deliberately separate from the analysis engine.

The final architecture is:

```text
                    External Data
                         │
          ┌──────────────┼──────────────┐
          │              │              │
         CSV            JSON          Parquet
          │              │              │
          ▼              ▼              ▼
      CSV Adapter    JSON Adapter   Parquet Adapter
          │              │              │
          └──────────────┼──────────────┘
                         ▼
                 Validation Layer
                         │
                         ▼
                 Normalization Layer
                         │
                         ▼
                  ACE Data Model
                         │
                         ▼
                    ace-core
                         │
          ┌──────────────┼──────────────┐
          ▼              ▼              ▼
      Agreement       Consensus       Quality
```

My goal is to make the source format irrelevant once the data reaches `ace-core`.

That separation allows me to add new annotation platforms and file formats without rewriting the statistical algorithms.

```
```
