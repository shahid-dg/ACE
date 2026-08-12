#
````markdown

# ACE Architecture

## 1. Purpose

ACE is designed as a modular annotation-quality analysis engine.

The architecture separates:

- annotation data representation
- data ingestion
- statistical analysis
- consensus estimation
- quality analysis
- report generation
- command-line interaction
- performance benchmarking

The primary architectural objective is to keep the analysis engine reusable independently of the interface used to invoke it.

---

## 2. System Architecture

```text
                         ┌─────────────────────┐
                         │       User          │
                         └──────────┬──────────┘
                                    │
                                    ▼
                         ┌─────────────────────┐
                         │      ace-cli        │
                         │                     │
                         │ Commands            │
                         │ Configuration       │
                         │ Orchestration       │
                         └──────────┬──────────┘
                                    │
                    ┌───────────────┼────────────────┐
                    │               │                │
                    ▼               ▼                ▼
             ┌────────────┐  ┌────────────┐  ┌────────────┐
             │   ace-io   │  │ ace-core   │  │ ace-report │
             │            │  │            │  │            │
             │ Parsers    │  │ Data Model │  │ HTML       │
             │ Adapters   │  │ Agreement  │  │ JSON       │
             │ Validation │  │ Consensus  │  │ Reports    │
             │ Export     │  │ Quality    │  │            │
             └─────┬──────┘  └──────┬─────┘  └──────┬─────┘
                   │                 │               │
                   └─────────────────┼───────────────┘
                                     │
                                     ▼
                              Analysis Results
````

---

## 3. Workspace Structure

```text
ace/
├── crates/
│   ├── ace-core/
│   ├── ace-io/
│   ├── ace-cli/
│   └── ace-report/
│
├── benchmarks/
│   └── ace-benchmarks/
│
├── tests/
├── examples/
├── docs/
├── data/
├── scripts/
└── Cargo.toml
```

---

## 4. Crate Responsibilities

### 4.1 `ace-core`

`ace-core` contains ACE's domain model and analysis algorithms.

It owns:

```text
Models
├── Dataset
├── Item
├── Annotation
├── Annotator
└── Label

Agreement
├── Pairwise agreement
├── Cohen's kappa
├── Fleiss' kappa
├── Krippendorff's alpha
└── Agreement matrices

Consensus
├── Majority vote
├── Weighted vote
├── Dawid-Skene
└── Confidence scoring

Quality
├── Annotator reliability
├── Disagreement
├── Outlier detection
├── Anomaly detection
└── Review prioritization
```

`ace-core` must remain independent of:

* CLI implementation
* filesystem access
* HTTP servers
* HTML rendering
* platform-specific annotation formats

The core crate should operate entirely on validated domain structures.

---

### 4.2 `ace-io`

`ace-io` provides the translation layer between external annotation formats and ACE's internal data model.

Responsibilities:

```text
External Data
     │
     ▼
Parser
     │
     ▼
Validation
     │
     ▼
Normalization
     │
     ▼
ace-core::Dataset
```

Supported and planned sources include:

* CSV
* JSON
* JSONL
* Parquet
* CVAT exports
* Label Studio exports
* custom adapters

The I/O layer must not contain statistical analysis logic.

---

### 4.3 `ace-cli`

`ace-cli` is the application boundary.

Its responsibilities are:

* command parsing
* configuration loading
* input selection
* invoking `ace-io`
* invoking `ace-core`
* invoking `ace-report`
* formatting terminal output
* returning process exit codes

The CLI should remain thin.

A command should orchestrate existing library functionality rather than implement algorithms directly.

---

### 4.4 `ace-report`

`ace-report` converts analysis results into external presentation formats.

Responsibilities include:

* HTML reports
* machine-readable JSON
* CSV exports
* report summaries
* visualizations
* review tables

Report generation must consume analysis results produced by `ace-core`.

It should not independently recalculate statistical metrics.

---

### 4.5 `ace-benchmarks`

Performance benchmarks live in a dedicated workspace package.

```text
benchmarks/
└── ace-benchmarks/
    ├── Cargo.toml
    └── benches/
        ├── agreement.rs
        ├── consensus.rs
        ├── quality.rs
        └── ingestion.rs
```

The benchmark package may depend on:

```text
ace-core
ace-io
```

Production crates must never depend on the benchmark package.

---

## 5. Dependency Rules

The intended dependency graph is:

```text
                    ┌───────────┐
                    │ ace-cli   │
                    └─────┬─────┘
                          │
              ┌───────────┼───────────┐
              ▼           ▼           ▼
         ┌────────┐  ┌─────────┐  ┌────────────┐
         │ ace-io │  │ace-core │  │ ace-report │
         └───┬────┘  └─────────┘  └──────┬─────┘
             │             ▲             │
             └─────────────┘             │
                           └─────────────┘


             ┌───────────────────┐
             │  ace-benchmarks   │
             └─────────┬─────────┘
                       │
                 ┌─────┴─────┐
                 ▼           ▼
             ace-core      ace-io
```

### Dependency constraints

```text
ace-core
    └── must not depend on ace-cli

ace-core
    └── must not depend on ace-report

ace-core
    └── must not depend on ace-io

ace-io
    └── may depend on ace-core

ace-report
    └── may depend on ace-core

ace-cli
    └── may depend on all production crates

ace-benchmarks
    └── may depend on production crates
```

This keeps the dependency graph acyclic.

---

## 6. Data Flow

A normal analysis operation follows this pipeline:

```text
             Annotation Export
                    │
                    ▼
             ┌──────────────┐
             │    ace-io    │
             └──────┬───────┘
                    │
                    ▼
              Raw Records
                    │
                    ▼
               Validation
                    │
                    ▼
              Normalization
                    │
                    ▼
             ACE Dataset
                    │
                    ▼
             ┌──────────────┐
             │   ace-core   │
             └──────┬───────┘
                    │
          ┌─────────┼─────────┐
          ▼         ▼         ▼
      Agreement Consensus  Quality
          │         │         │
          └─────────┼─────────┘
                    ▼
             Analysis Result
                    │
                    ▼
             ┌──────────────┐
             │ ace-report   │
             └──────┬───────┘
                    │
              ┌─────┴─────┐
              ▼           ▼
            JSON         HTML
```

---

## 7. Domain Boundary

The normalized dataset is the boundary between ingestion and analysis.

External formats may differ significantly:

```text
CVAT
Label Studio
CSV
JSON
Custom Export
```

They must eventually become the same conceptual representation:

```text
Dataset
├── items
├── annotators
└── annotations
```

This prevents platform-specific assumptions from entering the statistical engine.

---

## 8. Analysis Pipeline

Analysis is performed in defined stages.

### Stage 1 — Dataset Validation

The dataset is checked for structural validity.

Examples:

* missing identifiers
* malformed records
* invalid labels
* duplicate records
* inconsistent schema

### Stage 2 — Agreement

Annotator agreement is calculated.

Possible metrics:

```text
Pairwise Agreement
Cohen's Kappa
Fleiss' Kappa
Krippendorff's Alpha
```

### Stage 3 — Consensus

ACE estimates the most likely label.

Possible methods:

```text
Majority Vote
Weighted Vote
Dawid-Skene
```

### Stage 4 — Quality Analysis

The engine derives quality signals from:

* disagreement
* consensus confidence
* annotator reliability
* label distributions
* anomalous behavior

### Stage 5 — Prioritization

Items are ranked according to their estimated review value.

The output should allow a reviewer to answer:

> Which annotations should humans inspect first?

---

## 9. Error Boundaries

Each crate owns errors associated with its responsibility.

```text
ace-core
    CoreError

ace-io
    IoError

ace-report
    ReportError

ace-cli
    ApplicationError
```

Library crates return errors rather than terminating the process.

For example:

```rust
pub fn analyze(dataset: &Dataset) -> Result<AnalysisResult, CoreError>
```

The CLI converts library errors into user-facing messages and process exit codes.

---

## 10. Performance Architecture

Performance-sensitive operations should be isolated from presentation and I/O concerns.

```text
Input
  │
  ▼
Parse
  │
  ▼
Normalize
  │
  ▼
Compute ──────────► Parallel execution
  │
  ▼
Aggregate
  │
  ▼
Report
```

Parallelism should primarily be applied to computationally expensive operations.

Potential targets include:

* pairwise agreement
* agreement matrices
* item-level quality scoring
* consensus calculations
* large dataset transformations

Optimization decisions must be supported by benchmark measurements.

---

## 11. Memory Strategy

ACE is intended to process datasets substantially larger than typical interactive examples.

Memory-sensitive components should therefore prefer:

* compact domain structures
* indexed lookup tables
* reusable allocations
* batch processing
* streaming where appropriate
* avoiding unnecessary cloning

Large datasets should not automatically require converting every intermediate representation into another full copy.

---

## 12. Determinism

ACE analysis should be reproducible.

Given:

```text
same dataset
+
same configuration
+
same ACE version
```

the analysis should produce equivalent results.

Algorithms involving randomness must expose deterministic seed configuration.

Parallel execution must not unnecessarily change statistical results.

---

## 13. Security Boundary

ACE can be used with sensitive annotation datasets.

The architecture therefore treats imported data as untrusted input.

Input boundaries must perform validation before data enters analysis.

The application should avoid:

* logging raw annotation contents
* leaking input data through errors
* executing input-provided content
* trusting external HTML
* writing arbitrary user-controlled filesystem paths without validation

Generated HTML must be treated as an output boundary and appropriately escaped.

---

## 14. Testing Architecture

Testing follows the dependency structure.

```text
Unit Tests
    │
    ├── ace-core
    ├── ace-io
    └── ace-report
          │
          ▼
Integration Tests
          │
          ▼
CLI / End-to-End Tests
          │
          ▼
Benchmark Suite
```

### Unit tests

Validate individual functions and algorithms.

### Integration tests

Validate interactions between crates.

### End-to-end tests

Validate complete workflows.

### Benchmarks

Measure performance independently of correctness tests.

---

## 15. Extension Points

The architecture is designed to allow new functionality without modifying unrelated layers.

### New input format

Add a parser/adapter in `ace-io`.

```text
New Format
    │
    ▼
New Adapter
    │
    ▼
ACE Dataset
```

No changes should be required in the statistical algorithms.

### New agreement metric

Add an implementation under `ace-core`.

```text
ace-core/
└── agreement/
    └── new_metric.rs
```

### New consensus algorithm

Add an implementation under:

```text
ace-core/
└── consensus/
```

### New output format

Implement the output layer in `ace-report`.

---

## 16. Architectural Invariants

The following rules should remain true as ACE grows:

1. `ace-core` contains domain and analysis logic.
2. `ace-core` does not know about CLI or presentation.
3. `ace-io` translates external data into ACE structures.
4. `ace-report` presents results but does not own analysis.
5. `ace-cli` orchestrates rather than implements algorithms.
6. Benchmarks remain outside production crates.
7. Production dependencies remain acyclic.
8. Sensitive input is not logged by default.
9. Statistical algorithms are independently testable.
10. Performance claims are supported by reproducible benchmarks.

---

## 17. Architectural Goal

ACE should be usable in several environments without changing its core algorithms:

```text
                         ┌───────────────┐
                         │   ace-core    │
                         │               │
                         │ Analysis      │
                         │ Consensus     │
                         │ Quality       │
                         └───────┬───────┘
                                 │
              ┌──────────────────┼──────────────────┐
              │                  │                  │
              ▼                  ▼                  ▼
           CLI App          Python Binding      Future API
              │                  │                  │
              ▼                  ▼                  ▼
          Pipelines          Notebooks         Services
```

The core principle is simple:

> **Data enters through adapters, analysis happens in the core, and results leave through presentation layers.**

```
