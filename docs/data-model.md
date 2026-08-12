# 

````markdown
# ACE Data Model

## 1. Purpose

This document defines the canonical data model used internally by ACE.

The data model provides a stable boundary between:

- external annotation formats
- ingestion
- validation
- statistical analysis
- consensus estimation
- quality analysis
- reporting

External platforms may represent annotations differently, but once data enters `ace-core`, it should conform to the structures defined here.

---

# 2. Design Goals

The ACE data model is designed around the following requirements:

1. Represent annotations independently of their source platform.
2. Support multiple annotators per item.
3. Support incomplete annotation coverage.
4. Support categorical labels.
5. Preserve annotation metadata without coupling the core to a specific platform.
6. Allow deterministic analysis.
7. Avoid unnecessary data duplication.
8. Support large datasets.
9. Provide enough information for statistical analysis.
10. Keep ingestion-specific concerns outside `ace-core`.

---

# 3. Conceptual Model

The core domain consists of:

```text
Dataset
│
├── Items
│
├── Annotators
│
├── Labels
│
└── Annotations
       │
       ├── Item
       ├── Annotator
       └── Label
````

An annotation establishes the relationship:

```text
Annotator ────── annotates ──────► Item
     │                                │
     │                                │
     └──────────── Label ◄────────────┘
```

---

# 4. Core Entities

## 4.1 Dataset

A `Dataset` is the complete normalized collection of annotation information supplied to an ACE analysis.

Conceptually:

```rust
pub struct Dataset {
    pub items: Vec<Item>,
    pub annotators: Vec<Annotator>,
    pub annotations: Vec<Annotation>,
}
```

The exact implementation may use indexed or compact storage rather than these literal containers.

---

# 5. Item

An `Item` represents the entity being annotated.

Examples:

```text
image_001
document_1042
audio_019
text_883
```

A minimal representation:

```rust
pub struct Item {
    pub id: ItemId,
}
```

The core does not assume that an item is an image, document, audio file, or text sample.

---

# 6. Item ID

Every item must have a unique identifier within a dataset.

Conceptually:

```rust
pub struct ItemId(String);
```

Example:

```text
image_001
image_002
document_001
```

IDs should be:

* deterministic
* non-empty
* stable during analysis

The ID should not be interpreted semantically by the analysis engine.

---

# 7. Annotator

An `Annotator` represents the person, model, or labeling agent responsible for an annotation.

```rust
pub struct Annotator {
    pub id: AnnotatorId,
}
```

An annotator could represent:

* human worker
* expert reviewer
* automated labeling system
* model-generated annotation

ACE should not assume that every annotator is human.

---

# 8. Annotator ID

Each annotator must have a unique identifier within a dataset.

Conceptually:

```rust
pub struct AnnotatorId(String);
```

Examples:

```text
worker_001
worker_002
expert_a
model_v2
```

The identifier should not expose personal information unnecessarily.

---

# 9. Label

A `Label` represents the categorical value assigned to an item.

Examples:

```text
cat
dog
car
person
positive
negative
spam
ham
```

A canonical label representation can be:

```rust
pub struct LabelId(String);
```

ACE should treat labels as opaque identifiers.

The core should not assume that:

```text
cat > dog
```

or:

```text
positive = 1
negative = 0
```

unless an explicit ordering or mapping is provided.

---

# 10. Annotation

An `Annotation` connects an item, annotator, and label.

Conceptually:

```rust
pub struct Annotation {
    pub item_id: ItemId,
    pub annotator_id: AnnotatorId,
    pub label: LabelId,
}
```

This is the fundamental unit consumed by the agreement and consensus algorithms.

---

# 11. Annotation Metadata

External annotation systems often contain additional information.

Examples:

```text
timestamp
confidence
bounding box
review status
source
task ID
platform ID
annotation version
```

The core model should allow optional metadata without making these fields mandatory for statistical analysis.

Conceptually:

```rust
pub struct AnnotationMetadata {
    pub values: Map<String, MetadataValue>,
}
```

Possible metadata values:

```rust
pub enum MetadataValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}
```

Metadata must not influence statistical calculations unless an algorithm explicitly requests it.

---

# 12. Source Metadata

ACE may need to preserve information about where a record originated.

Example:

```rust
pub struct SourceMetadata {
    pub source: String,
    pub source_id: Option<String>,
}
```

This allows reporting such as:

```text
Source: label-studio
Task: 849201
```

without forcing the core analysis engine to understand Label Studio's internal schema.

---

# 13. Dataset Relationships

A normalized dataset can be represented as:

```text
Dataset
│
├── Item 001
│     ├── Annotation A → cat
│     ├── Annotation B → cat
│     └── Annotation C → dog
│
├── Item 002
│     ├── Annotation A → dog
│     └── Annotation C → dog
│
└── Item 003
      ├── Annotation A → bird
      ├── Annotation B → bird
      └── Annotation C → bird
```

Missing annotations are represented by the absence of an annotation.

They are not represented as a special label.

---

# 14. Normalized Representation

External data:

```text
item_id,worker,label
image_001,A,cat
image_001,B,cat
image_001,C,dog
```

becomes:

```text
Items
├── image_001

Annotators
├── A
├── B
└── C

Annotations
├── image_001 / A / cat
├── image_001 / B / cat
└── image_001 / C / dog
```

This representation is platform-independent.

---

# 15. Identifier Strategy

ACE should use strongly typed identifiers instead of passing arbitrary strings throughout the analysis engine.

Example:

```rust
pub struct ItemId(String);

pub struct AnnotatorId(String);

pub struct LabelId(String);
```

This prevents accidental interchange of:

```text
ItemId
```

with:

```text
AnnotatorId
```

even though both may internally contain strings.

---

# 16. Identifier Validation

Identifiers must satisfy:

* non-empty
* deterministic equality
* stable hashing
* no accidental whitespace normalization
* no implicit type conversion

Whether whitespace is allowed should be decided at the ingestion boundary.

The core should receive already validated identifiers.

---

# 17. Duplicate Annotations

The logical key of a categorical annotation is:

```text
(item_id, annotator_id)
```

Under the default model, an annotator should provide at most one active label for an item.

Therefore:

```text
image_001 / worker_01 / cat
image_001 / worker_01 / dog
```

is ambiguous.

ACE must not silently treat both records as independent annotations.

Possible policies:

```text
reject
deduplicate
replace
preserve-as-revisions
```

The selected policy belongs to ingestion/configuration rather than the statistical algorithms.

---

# 18. Annotation Revisions

Some annotation platforms record changes as separate events.

For example:

```text
worker_01 → cat
worker_01 → dog
worker_01 → cat
```

These should not automatically become three independent votes.

A platform adapter should resolve revisions into the active annotation state before creating the canonical dataset.

If revision history is required, it should be stored as metadata or a separate event model.

---

# 19. Missing Data

Missing annotations are valid.

Example:

```text
Item  Annotator A  Annotator B  Annotator C
--------------------------------------------
001   cat          cat          dog
002   dog          dog          -
003   bird         -            bird
```

The missing values do not represent labels.

They simply indicate that no observation exists.

Algorithms must explicitly define how missing observations affect their calculations.

---

# 20. Label Universe

The dataset contains a set of known labels:

```rust
pub struct LabelSet {
    pub labels: Vec<LabelId>,
}
```

Example:

```text
cat
dog
bird
```

The label universe can be derived from annotations.

For some algorithms, it may instead be explicitly configured.

This distinction matters for metrics such as normalized entropy.

---

# 21. Dataset Indexes

Large datasets should not repeatedly scan the complete annotation collection.

ACE should provide indexed access patterns such as:

```text
item → annotations
annotator → annotations
label → annotations
```

Conceptually:

```rust
pub struct DatasetIndex {
    pub by_item: ...,
    pub by_annotator: ...,
    pub by_label: ...,
}
```

The actual representation should be selected based on benchmark results.

---

# 22. Item-Centric View

Agreement and consensus algorithms frequently need:

```text
all annotations for item X
```

Therefore the engine should efficiently support:

```text
Item
  │
  ├── Annotator A → Label X
  ├── Annotator B → Label X
  └── Annotator C → Label Y
```

This is the primary view for consensus calculation.

---

# 23. Annotator-Centric View

Quality analysis frequently needs:

```text
all annotations produced by annotator X
```

Example:

```text
Annotator A
│
├── item_001 → cat
├── item_002 → dog
├── item_003 → cat
└── item_004 → bird
```

This view is required for:

* annotator reliability
* label distribution analysis
* agreement analysis
* anomaly detection

---

# 24. Label-Centric View

Some statistics require global label counts.

Example:

```text
cat  → 18,203
dog  → 14,291
bird →  3,882
```

The data model should allow these counts to be calculated efficiently.

---

# 25. Analysis Results

Analysis should not mutate the source dataset.

Instead:

```text
Dataset
   │
   ▼
Analysis
   │
   ▼
AnalysisResult
```

Conceptually:

```rust
pub struct AnalysisResult {
    pub agreement: Option<AgreementResult>,
    pub consensus: Option<ConsensusResult>,
    pub quality: Option<QualityResult>,
}
```

This keeps source data separate from derived information.

---

# 26. Agreement Result

An agreement result may contain:

```rust
pub struct AgreementResult {
    pub metric: AgreementMetric,
    pub score: Option<f64>,
    pub observations: usize,
}
```

For pairwise analysis:

```rust
pub struct PairwiseAgreement {
    pub annotator_a: AnnotatorId,
    pub annotator_b: AnnotatorId,
    pub score: Option<f64>,
    pub comparable_items: usize,
}
```

---

# 27. Consensus Result

Consensus results should preserve both the selected label and confidence.

```rust
pub struct ItemConsensus {
    pub item_id: ItemId,
    pub label: Option<LabelId>,
    pub confidence: Option<f64>,
}
```

For probabilistic algorithms, the full distribution may also be retained:

```rust
pub struct LabelProbability {
    pub label: LabelId,
    pub probability: f64,
}
```

---

# 28. Quality Result

Quality analysis operates primarily at two levels.

### Item level

```text
item
├── disagreement
├── entropy
├── consensus confidence
└── review priority
```

### Annotator level

```text
annotator
├── reliability
├── agreement
├── annotation count
└── anomaly score
```

Conceptually:

```rust
pub struct ItemQuality {
    pub item_id: ItemId,
    pub disagreement: Option<f64>,
    pub entropy: Option<f64>,
    pub confidence: Option<f64>,
    pub priority: Option<f64>,
}
```

---

# 29. Numeric Types

Probabilities and statistical scores should generally use:

```rust
f64
```

rather than `f32`.

Reasons include:

* statistical calculations
* iterative algorithms
* probability products
* aggregation
* numerical stability

Values representing counts should use integer types.

For example:

```text
annotation_count → usize
probability      → f64
score            → f64
```

---

# 30. Probability Validation

Probability values must satisfy:

[
0 \leq p \leq 1
]

The implementation should reject or normalize invalid probabilities according to the specific API contract.

For probability distributions:

[
\sum_i p_i \approx 1
]

Floating-point comparisons should use an appropriate tolerance rather than exact equality.

---

# 31. Dataset Validation

Before analysis begins, the dataset should be validated for:

```text
✓ unique item IDs
✓ unique annotator IDs
✓ valid labels
✓ valid annotation references
✓ non-empty identifiers
✓ duplicate annotation policy
✓ valid numeric metadata
```

Invalid input should produce structured validation errors.

---

# 32. Validation Pipeline

```text
External Input
      │
      ▼
Parse
      │
      ▼
Schema Validation
      │
      ▼
Identifier Validation
      │
      ▼
Relationship Validation
      │
      ▼
Duplicate Handling
      │
      ▼
Normalization
      │
      ▼
ACE Dataset
```

Only validated data should enter the statistical engine.

---

# 33. Serialization

The internal model should support serialization where useful.

Serde is the intended serialization framework.

Example:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    pub item_id: ItemId,
    pub annotator_id: AnnotatorId,
    pub label: LabelId,
}
```

Serialization formats should be handled primarily by `ace-io`.

`ace-core` should not contain platform-specific serialization behavior.

---

# 34. External Format Mapping

The mapping layer should follow:

```text
External Format
      │
      ▼
Platform Adapter
      │
      ▼
Normalized Record
      │
      ▼
Dataset Builder
      │
      ▼
ACE Dataset
```

For example:

```text
CVAT XML
   │
   ▼
CVAT Adapter
   │
   ▼
ACE Annotation
```

The statistical engine should never receive raw CVAT structures.

---

# 35. Metadata Isolation

Metadata must not accidentally affect statistical calculations.

For example:

```text
Annotation
├── label = cat
├── confidence = 0.91
└── timestamp = ...
```

The agreement engine should use:

```text
label = cat
```

unless an algorithm explicitly declares that metadata is part of its input.

---

# 36. Dataset Immutability

Analysis functions should prefer read-only access:

```rust
pub fn analyze(
    dataset: &Dataset,
) -> Result<AnalysisResult, CoreError>
```

rather than:

```rust
pub fn analyze(
    dataset: &mut Dataset,
)
```

Analysis should not alter:

* annotations
* labels
* annotator identities
* item identities

Derived results belong in separate structures.

---

# 37. Ownership and Memory

The Rust implementation should avoid unnecessary cloning of identifiers and annotations.

Where appropriate, internal storage can use integer indexes:

```text
ItemId
   │
   ▼
ItemIndex = 42

AnnotatorId
   │
   ▼
AnnotatorIndex = 7

LabelId
   │
   ▼
LabelIndex = 2
```

Annotations can then store compact indexes:

```text
item_index
annotator_index
label_index
```

This approach can significantly reduce memory usage for large datasets.

The decision should be benchmark-driven.

---

# 38. Canonical Internal Representation

A large dataset may eventually be represented internally as:

```text
Dataset
│
├── ItemTable
│     ├── ItemId
│     └── Item metadata
│
├── AnnotatorTable
│     ├── AnnotatorId
│     └── Annotator metadata
│
├── LabelTable
│     └── LabelId
│
└── AnnotationTable
      ├── item_index
      ├── annotator_index
      └── label_index
```

This is preferable to repeatedly storing long string identifiers in every annotation.

---

# 39. Sparse Data

ACE should assume annotation datasets can be sparse.

Example:

```text
100,000 items
20,000 annotators
3 annotations per item
```

A dense:

```text
100,000 × 20,000
```

matrix would be unnecessarily large.

The canonical annotation representation should therefore be sparse.

---

# 40. Dataset Statistics

The dataset model should make common statistics accessible without forcing repeated full scans.

Useful statistics include:

```text
item_count
annotator_count
label_count
annotation_count
average_annotations_per_item
average_annotations_per_annotator
minimum_annotations_per_item
maximum_annotations_per_item
```

These statistics can be exposed through a dataset summary structure.

---

# 41. Dataset Summary

Conceptually:

```rust
pub struct DatasetSummary {
    pub items: usize,
    pub annotators: usize,
    pub labels: usize,
    pub annotations: usize,
    pub average_annotations_per_item: f64,
}
```

The summary should be derived from the validated dataset.

---

# 42. Versioning

The serialized ACE representation may evolve.

If persisted datasets or analysis results become a supported feature, they should include an explicit schema version.

Example:

```json
{
  "schema_version": 1,
  "items": [],
  "annotators": [],
  "annotations": []
}
```

Schema changes should be backward-compatible where practical.

---

# 43. Ground Truth Model

Ground truth should remain distinct from normal annotations.

Conceptually:

```rust
pub struct GroundTruth {
    pub item_id: ItemId,
    pub label: LabelId,
}
```

This allows:

```text
Annotations
     │
     ├── Annotator A
     ├── Annotator B
     └── Annotator C
     
Ground Truth
     │
     └── Expert / validated label
```

This distinction allows ACE to evaluate consensus against an external reference.

---

# 44. Label Taxonomy

The initial ACE model assumes flat categorical labels.

Example:

```text
cat
dog
bird
```

Hierarchical labels may be added later:

```text
animal
├── mammal
│   ├── cat
│   └── dog
└── bird
```

Hierarchical semantics should not be simulated by string parsing.

They require an explicit taxonomy model.

---

# 45. Multi-Label Annotations

The initial model is primarily designed for single categorical labels.

A future multi-label representation could be:

```text
image_001
    ├── person
    ├── vehicle
    └── building
```

This is a fundamentally different statistical problem from single-label classification.

Multi-label support should therefore be implemented as an explicit model extension rather than silently treating multiple labels as independent categorical votes.

---

# 46. Continuous Annotations

Continuous annotations such as:

```text
rating = 4.7
score = 82.3
```

require different agreement metrics.

The current model focuses on categorical annotation quality.

Support for:

* ordinal labels
* interval measurements
* continuous measurements

should be introduced with explicit measurement-level semantics.

---

# 47. Data Model Invariants

The following invariants must hold after dataset construction:

```text
1. Every item ID is unique.
2. Every annotator ID is unique.
3. Every annotation references a valid item.
4. Every annotation references a valid annotator.
5. Every annotation references a valid label.
6. Missing annotations are represented by absence.
7. Duplicate active annotations are resolved before analysis.
8. Analysis does not mutate source data.
9. IDs remain stable during analysis.
10. Statistical results are derived from validated data.
```

---

# 48. Example Complete Dataset

```text
Dataset
│
├── Items
│   ├── image_001
│   ├── image_002
│   └── image_003
│
├── Annotators
│   ├── worker_01
│   ├── worker_02
│   └── worker_03
│
├── Labels
│   ├── cat
│   ├── dog
│   └── bird
│
└── Annotations
    │
    ├── image_001 / worker_01 / cat
    ├── image_001 / worker_02 / cat
    ├── image_001 / worker_03 / dog
    │
    ├── image_002 / worker_01 / dog
    ├── image_002 / worker_02 / dog
    ├── image_002 / worker_03 / dog
    │
    ├── image_003 / worker_01 / bird
    └── image_003 / worker_03 / bird
```

Analysis can derive:

```text
image_001
    consensus = cat
    disagreement = 0.333
    confidence = 0.667

image_002
    consensus = dog
    disagreement = 0.000
    confidence = 1.000

image_003
    consensus = bird
    annotator_02 = missing
```

---

# 49. Implementation Location

The primary domain model belongs in:

```text
crates/ace-core/src/
├── model/
│   ├── mod.rs
│   ├── dataset.rs
│   ├── item.rs
│   ├── annotator.rs
│   ├── annotation.rs
│   ├── label.rs
│   ├── ids.rs
│   └── metadata.rs
```

Indexes and optimized representations can be introduced separately:

```text
crates/ace-core/src/
└── index/
    ├── mod.rs
    ├── item_index.rs
    ├── annotator_index.rs
    └── label_index.rs
```

The exact module layout may evolve as implementation progresses.

---

# 50. Design Principle

The ACE data model follows one central rule:

> **Normalize once, analyze many times.**

External annotation formats should be converted into a stable canonical representation.

After normalization, every algorithm should operate on the same domain model regardless of whether the original annotations came from:

```text
CVAT
Label Studio
CSV
JSON
Parquet
Custom Annotation Platform
```

This keeps the statistical engine independent, testable, and reusable.

```