ace/
├── .github/
│   └── workflows/
│       ├── ci.yml
│       ├── benchmarks.yml
│       └── release.yml
│
├── crates/
│   ├── ace-core/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── error.rs
│   │   │   ├── models/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── annotation.rs
│   │   │   │   ├── annotator.rs
│   │   │   │   ├── item.rs
│   │   │   │   ├── label.rs
│   │   │   │   └── dataset.rs
│   │   │   │
│   │   │   ├── agreement/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── pairwise.rs
│   │   │   │   ├── cohens_kappa.rs
│   │   │   │   ├── fleiss_kappa.rs
│   │   │   │   ├── krippendorffs_alpha.rs
│   │   │   │   ├── confusion_matrix.rs
│   │   │   │   └── agreement_matrix.rs
│   │   │   │
│   │   │   ├── consensus/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── majority_vote.rs
│   │   │   │   ├── weighted_vote.rs
│   │   │   │   ├── dawid_skene.rs
│   │   │   │   └── confidence.rs
│   │   │   │
│   │   │   └── quality/
│   │   │       ├── mod.rs
│   │   │       ├── reliability.rs
│   │   │       ├── disagreement.rs
│   │   │       ├── outliers.rs
│   │   │       ├── anomalies.rs
│   │   │       └── prioritization.rs
│   │   │
│   │   └── Cargo.toml
│   │
│   ├── ace-io/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── error.rs
│   │   │   ├── reader.rs
│   │   │   ├── writer.rs
│   │   │   ├── csv/
│   │   │   │   ├── mod.rs
│   │   │   │   └── reader.rs
│   │   │   ├── json/
│   │   │   │   ├── mod.rs
│   │   │   │   └── reader.rs
│   │   │   ├── adapters/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── cvat.rs
│   │   │   │   └── label_studio.rs
│   │   │   └── schema.rs
│   │   └── Cargo.toml
│   │
│   ├── ace-cli/
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── cli.rs
│   │   │   ├── config.rs
│   │   │   ├── output.rs
│   │   │   └── commands/
│   │   │       ├── mod.rs
│   │   │       ├── analyze.rs
│   │   │       ├── agreement.rs
│   │   │       ├── consensus.rs
│   │   │       ├── detect.rs
│   │   │       └── report.rs
│   │   └── Cargo.toml
│   │
│   └── ace-report/
│       ├── src/
│       │   ├── lib.rs
│       │   ├── error.rs
│       │   ├── report.rs
│       │   ├── templates/
│       │   │   ├── mod.rs
│       │   │   ├── layout.rs
│       │   │   ├── overview.rs
│       │   │   ├── agreement.rs
│       │   │   ├── annotators.rs
│       │   │   ├── quality.rs
│       │   │   └── consensus.rs
│       │   └── assets/
│       │       ├── mod.rs
│       │       ├── styles.css
│       │       └── report.js
│       └── Cargo.toml
│
├── benchmarks/
│   ├── Cargo.toml
│   ├── benches/
│   │   ├── agreement.rs
│   │   ├── consensus.rs
│   │   ├── quality.rs
│   │   └── ingestion.rs
│   └── src/
│       └── lib.rs
│
├── tests/
│   ├── integration/
│   │   ├── analyze.rs
│   │   ├── agreement.rs
│   │   ├── consensus.rs
│   │   ├── detection.rs
│   │   └── reporting.rs
│   ├── fixtures/
│   │   ├── small.csv
│   │   ├── small.json
│   │   └── expected/
│   │       ├── agreement.json
│   │       └── consensus.json
│   └── common/
│       └── mod.rs
│
├── examples/
│   ├── basic_analysis.rs
│   ├── consensus.rs
│   └── custom_adapter.rs
│
│
├── docs/
│   ├── architecture.md
│   ├── algorithms.md
│   ├── data-model.md
│   ├── benchmarking.md
│   ├── configuration.md
│   ├── input-formats.md
│   ├── output-formats.md
│   └── contributing.md
│
├── data/
│   ├── sample/
│   │   ├── annotations.csv
│   │   └── annotations.json
│   └── benchmarks/
│       └── README.md
│
├── scripts/
│   ├── benchmark.sh
│   ├── generate-test-data.py
│   └── generate-report.sh
│
├── .gitignore
├── .gitattributes
├── rustfmt.toml
├── clippy.toml
├── Cargo.toml
├── Cargo.lock
├── LICENSE
├── README.md
├── CHANGELOG.md
├── CONTRIBUTING.md
├── SECURITY.md
└── CODE_OF_CONDUCT.md