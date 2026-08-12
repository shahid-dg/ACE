# ACE — Annotation Consensus Engine

> **High-performance annotation quality assurance and consensus analysis for large-scale datasets, built in Rust.**



\

ACE is a high-performance, platform-agnostic toolkit for analyzing **multi-annotator datasets**.

It identifies annotation disagreements, estimates annotator reliability, computes consensus labels, and surfaces suspicious or low-confidence annotations for human review.

Built from the ground up in **Rust**, ACE is designed for large datasets where memory efficiency, parallel processing, reproducibility, and command-line automation matter.

---

## Why ACE?

Large AI datasets are often labeled by multiple people.

The difficult part isn't collecting the labels.

It's determining:

* Which labels are trustworthy?
* Which annotators consistently disagree?
* Which items are genuinely ambiguous?
* Where are the likely labeling errors?
* Which samples should humans review first?
* How confident are we in the final consensus?

Manually answering those questions doesn't scale.

ACE turns annotation exports into an automated quality-analysis pipeline.

```text
             Annotation Data
                    │
                    ▼
          ┌──────────────────┐
          │     Ingestion    │
          │  CSV / JSON      │
          └────────┬─────────┘
                   │
                   ▼
          ┌──────────────────┐
          │ Normalization    │
          │ & Validation     │
          └────────┬─────────┘
                   │
          ┌────────┴─────────┐
          ▼                  ▼
   Agreement Analysis   Consensus Engine
          │                  │
          └────────┬─────────┘
                   ▼
          Quality Analysis
                   │
          ┌────────┴─────────┐
          ▼                  ▼
     Machine Output      HTML Report
```

---

## Core Capabilities

### Annotation Agreement

Measure how consistently annotators label the same items.

Planned metrics include:

* Pairwise agreement
* Cohen's κ
* Fleiss' κ
* Krippendorff's α
* Agreement matrices
* Confusion matrices

---

### Consensus Estimation

Determine the most likely label for each item using multiple consensus strategies.

Supported/planned methods include:

* Majority voting
* Weighted voting
* Dawid-Skene
* Confidence estimation

Example:

```text
Item: image_48291

Annotator A    → cat
Annotator B    → cat
Annotator C    → dog
Annotator D    → cat
Annotator E    → cat

Consensus      → cat
Confidence     → 0.84

Review status  → not required
```

---

### Annotator Reliability

ACE analyzes annotation behavior to estimate annotator reliability.

Example output:

```text
ANNOTATOR        RELIABILITY     LABELS
────────────────────────────────────────
annotator_07       0.94          182,401
annotator_12       0.91          175,293
annotator_03       0.87          190,812
annotator_18       0.64          161,204
```

The goal isn't simply to rank annotators.

ACE can identify patterns such as:

* consistently low agreement
* systematic disagreement with consensus
* class-specific disagreement
* unusually high/low label distributions

---

### Suspicious Annotation Detection

ACE surfaces annotations and items that deserve human attention.

For example:

```text
⚠ HIGH PRIORITY

Item: image_73182

Consensus confidence: 0.51
Annotator disagreement: HIGH
Annotators: 5

Labels:
  cat    2
  dog    2
  fox    1

Recommendation:
  Human review
```

This allows QA teams to focus their limited review capacity on the most uncertain samples.

---

## Input

ACE is designed around a normalized annotation model rather than a specific annotation platform.

This makes it possible to import exports from different systems through adapters.

Initial formats:

* CSV
* JSON

Planned:

* JSONL
* Parquet
* custom adapter API
* annotation-platform-specific importers

Example CSV:

```csv
item_id,annotator_id,label
image_001,worker_01,cat
image_001,worker_02,cat
image_001,worker_03,dog
image_002,worker_01,dog
image_002,worker_02,dog
```

---

## Output

ACE can produce machine-readable output for pipelines:

```bash
ace analyze annotations.csv --format json
```

Example:

```json
{
  "items": 100000,
  "annotations": 487231,
  "annotators": 42,
  "agreement": 0.873,
  "flagged_items": 4217
}
```

It can also generate a human-readable HTML report:

```bash
ace report annotations.csv --output report.html
```

The report is designed to provide:

* dataset statistics
* annotator statistics
* agreement matrices
* consensus confidence
* suspicious items
* disagreement patterns
* review priorities

---

# CLI

Analyze a dataset:

```bash
ace analyze annotations.csv
```

Calculate agreement:

```bash
ace agreement annotations.csv
```

Generate consensus labels:

```bash
ace consensus annotations.csv --method dawid-skene
```

Find suspicious annotations:

```bash
ace detect annotations.csv
```

Generate an HTML report:

```bash
ace report annotations.csv --output report.html
```

Export machine-readable results:

```bash
ace analyze annotations.csv --format json > results.json
```

---

# Architecture

ACE is structured as a modular Rust workspace.

```text
ace/
├── crates/
│   ├── ace-core/
│   │   ├── models/
│   │   ├── consensus/
│   │   ├── agreement/
│   │   └── quality/
│   │
│   ├── ace-io/
│   │   ├── csv/
│   │   ├── json/
│   │   └── adapters/
│   │
│   ├── ace-cli/
│   │   └── commands/
│   │
│   └── ace-report/
│       └── html/
│
├── benchmarks/
├── tests/
├── examples/
├── docs/
├── Cargo.toml
└── README.md
```

The core library remains independent from the CLI so ACE can eventually be embedded into other applications and data pipelines.

---

# Design Goals

ACE is being developed around several principles.

### Performance

Large annotation datasets should not require a heavyweight runtime.

ACE uses Rust's:

* zero-cost abstractions
* efficient memory management
* data-parallel execution
* streaming I/O
* optimized data structures

Parallel workloads can use [Rayon](https://github.com/rayon-rs/rayon).

---

### Reproducibility

Analysis should produce deterministic and reproducible results whenever the underlying algorithm permits it.

Configuration should be explicit.

```text
dataset
+
configuration
+
algorithm
↓
reproducible result
```

---

### Platform Independence

ACE should operate on annotation exports rather than being tightly coupled to one annotation platform.

The goal is:

```text
CVAT ───────┐
Label Studio ├──→ ACE → Analysis
Custom CSV ──┤
JSON ────────┘
```

---

### Automation First

ACE is designed to work inside automated workflows.

For example:

```text
Annotation Export
       ↓
      ACE
       ↓
Quality Report
       ↓
CI / Data Pipeline
       ↓
Human Review
```

This makes the tool useful beyond interactive analysis.

---

# Algorithms

ACE is designed to support multiple levels of annotation analysis.

### Agreement

* Pairwise agreement
* Cohen's κ
* Fleiss' κ
* Krippendorff's α

### Consensus

* Majority vote
* Weighted majority vote
* Dawid-Skene

### Quality Analysis

* annotator reliability
* disagreement scoring
* consensus confidence
* outlier detection
* uncertainty ranking

Algorithms are implemented and benchmarked in Rust rather than delegating the computation to a Python runtime.

---

# Performance

Performance is a first-class part of ACE.

Rather than making unsupported claims about speed, the project publishes reproducible benchmarks.

Benchmarks will measure:

| Dataset | Annotations | Runtime | Peak RAM |
| ------- | ----------: | ------: | -------: |
| Small   |        100K |     TBD |      TBD |
| Medium  |          1M |     TBD |      TBD |
| Large   |         10M |     TBD |      TBD |

Benchmark methodology will be documented so results can be independently reproduced.

---

# Example Workflow

Suppose a dataset contains:

```text
250,000 items
1,200,000 annotations
35 annotators
12 possible classes
```

Run:

```bash
ace report annotations.csv --output qa-report.html
```

ACE analyzes the dataset and produces:

```text
Dataset
────────────────────────────
Items:              250,000
Annotations:      1,200,000
Annotators:              35
Classes:                 12

Agreement
────────────────────────────
Mean agreement:      0.871
Fleiss κ:            0.792

Quality
────────────────────────────
Low-confidence items:  8,421
High-disagreement:     3,182
Potential outliers:      914

Consensus
────────────────────────────
High confidence:     218,391
Medium confidence:    23,688
Low confidence:        7,921
```

The resulting HTML report allows a QA manager to investigate the highest-priority items instead of manually reviewing the entire dataset.

---

# Technology

ACE is built primarily with:

* **Rust**
* Cargo
* Serde
* Rayon
* Criterion
* Clap
* Tokio where asynchronous I/O is required

Planned components may include:

* Arrow / Parquet support
* SIMD optimizations
* Python bindings
* WASM-based report components

---

# Installation

## From source

```bash
git clone https://github.com/shahid-dg/ace.git
cd ace

cargo build --release
```

Run:

```bash
cargo run --release -- analyze data/annotations.csv
```

Once releases are published:

```bash
ace --version
```

---

# Development

Clone the repository:

```bash
git clone https://github.com/shahid-dg/ace.git
cd ace
```

Run tests:

```bash
cargo test --workspace
```

Run benchmarks:

```bash
cargo bench
```

Check formatting:

```bash
cargo fmt --all -- --check
```

Run Clippy:

```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

---

# Roadmap

## Phase 1 — Foundation

* [x] Project architecture
* [ ] Annotation schema
* [ ] CSV ingestion
* [ ] JSON ingestion
* [ ] Validation
* [ ] CLI

## Phase 2 — Agreement

* [ ] Pairwise agreement
* [ ] Cohen's κ
* [ ] Fleiss' κ
* [ ] Confusion matrices
* [ ] Agreement matrices

## Phase 3 — Consensus

* [ ] Majority voting
* [ ] Weighted voting
* [ ] Dawid-Skene
* [ ] Confidence scoring

## Phase 4 — Quality Analysis

* [ ] Annotator reliability
* [ ] Suspicious annotation detection
* [ ] Difficult-item detection
* [ ] Outlier analysis
* [ ] Review prioritization

## Phase 5 — Performance

* [ ] Parallel processing
* [ ] Streaming ingestion
* [ ] Memory optimization
* [ ] Benchmark suite
* [ ] SIMD experimentation

## Phase 6 — Reporting

* [ ] HTML reports
* [ ] Interactive agreement matrix
* [ ] Annotator dashboards
* [ ] Confidence distribution
* [ ] Review queue export

## Phase 7 — Integrations

* [ ] Parquet
* [ ] Python bindings
* [ ] Annotation platform adapters
* [ ] CI integration

---

# Use Cases

ACE is designed for teams working with:

* computer vision datasets
* NLP datasets
* content moderation datasets
* speech datasets
* classification datasets
* human preference datasets
* AI evaluation datasets
* crowdsourced annotation projects

---

# Project Philosophy

ACE is not intended to replace human reviewers.

It is designed to answer a more useful question:

> **Where should humans spend their limited QA time?**

Instead of manually reviewing an entire dataset, teams can prioritize:

```text
High confidence
       ↓
Automatically accepted

Low confidence
       ↓
Human review

Strong annotator disagreement
       ↓
Investigate

Systematic annotator deviation
       ↓
QA investigation
```

---

# Limitations

ACE does not determine ground truth by itself.

Consensus algorithms can still produce incorrect results when:

* all annotators make the same mistake
* the dataset contains ambiguous examples
* annotators lack domain expertise
* labels are poorly defined
* the annotation process is systematically biased

For that reason, ACE reports confidence and disagreement rather than presenting every consensus result as absolute truth.

---

# Contributing

Contributions are welcome.

Areas that would be particularly useful include:

* additional agreement metrics
* consensus algorithms
* annotation-format adapters
* performance improvements
* benchmark datasets
* report visualizations
* documentation
* testing

Please open an issue before implementing major architectural changes.

---

# License

ACE is released under the MIT License.

See LICENSE for details.

---

# Status

🚧 **ACE is currently under active development.**

The architecture and APIs may change before the first stable release.

The project prioritizes:

**correctness → reproducibility → performance → convenience**

---

## Built with Rust

ACE exists to explore what a modern, high-performance annotation quality system can look like when its core is designed around Rust's strengths:

**safe memory management.**

**parallel computation.**

**predictable performance.**

**portable binaries.**

**composable tooling.**

> **Turn millions of annotations into actionable quality signals.**
