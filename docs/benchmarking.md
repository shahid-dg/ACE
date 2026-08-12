#

````markdown
# ACE Benchmarking

## 1. Purpose

ACE treats performance as a measurable engineering requirement.

The benchmark suite exists to answer concrete questions:

- How quickly can ACE process annotation datasets?
- How does performance change as dataset size increases?
- Which algorithms consume the most CPU time?
- How much memory is required?
- Does an optimization actually improve performance?
- Does parallel processing provide measurable gains?
- Does performance regress between releases?

Benchmarks are maintained separately from production source code.

---

# 2. Benchmark Architecture

ACE uses a dedicated benchmark package:

```text
benchmarks/
└── ace-benchmarks/
    ├── Cargo.toml
    └── benches/
        ├── agreement.rs
        ├── consensus.rs
        ├── quality.rs
        └── ingestion.rs
````

Production code remains under:

```text
crates/
├── ace-core/
├── ace-io/
├── ace-cli/
└── ace-report/
```

The benchmark package may depend on production crates, but production crates must never depend on the benchmark package.

```text
             ┌──────────────────┐
             │ ace-benchmarks   │
             └────────┬─────────┘
                      │
             ┌────────┴────────┐
             ▼                 ▼
        ┌─────────┐       ┌─────────┐
        │ace-core │       │ ace-io  │
        └─────────┘       └─────────┘
```

---

# 3. Benchmarking Principles

Benchmarks must be:

1. Reproducible.
2. Representative of real workloads.
3. Isolated from unrelated I/O.
4. Large enough to expose meaningful differences.
5. Run using release-quality compiler optimizations.
6. Compared using consistent hardware where possible.
7. Interpreted using distributions rather than a single timing.
8. Accompanied by dataset characteristics.

A benchmark result without workload information is incomplete.

---

# 4. Benchmark Tool

ACE uses Criterion for statistical benchmarking.

The benchmark package should contain the dependency:

```toml
[dev-dependencies]
criterion = { version = "0.7", features = ["html_reports"] }
```

The exact version is controlled by the workspace dependency configuration.

---

# 5. Running Benchmarks

Run the complete benchmark suite:

```bash
cargo bench -p ace-benchmarks
```

Run a specific benchmark:

```bash
cargo bench -p ace-benchmarks -- agreement
```

Run consensus benchmarks:

```bash
cargo bench -p ace-benchmarks -- consensus
```

Run quality benchmarks:

```bash
cargo bench -p ace-benchmarks -- quality
```

Run ingestion benchmarks:

```bash
cargo bench -p ace-benchmarks -- ingestion
```

---

# 6. Release Builds

Benchmark execution must use optimized builds.

Cargo's benchmark profile is optimized by default.

The benchmark environment should therefore be treated as:

```text
Debug development
      │
      │ not suitable for performance claims
      ▼
Release benchmark
      │
      ▼
Criterion measurement
```

Performance claims must never be based on:

```bash
cargo run
```

or:

```bash
cargo test
```

timings.

---

# 7. Benchmark Categories

ACE benchmarks are divided into four primary categories.

```text
Benchmarks
│
├── Agreement
│   ├── Pairwise agreement
│   ├── Cohen's kappa
│   ├── Fleiss' kappa
│   ├── Krippendorff's alpha
│   └── Agreement matrix
│
├── Consensus
│   ├── Majority vote
│   ├── Weighted vote
│   └── Dawid-Skene
│
├── Quality
│   ├── Disagreement
│   ├── Entropy
│   ├── Reliability
│   └── Review prioritization
│
└── Ingestion
    ├── CSV
    ├── JSON
    └── Other supported formats
```

---

# 8. Dataset Dimensions

Synthetic benchmark datasets should vary across several dimensions.

Important variables include:

```text
item count
annotator count
label count
annotations per item
annotation density
label distribution
annotator reliability
```

Example benchmark sizes:

```text
Small
10,000 items

Medium
100,000 items

Large
1,000,000 items

Stress
10,000,000 items
```

These numbers are targets rather than mandatory requirements for every benchmark.

---

# 9. Dataset Generation

Benchmarks should use deterministic synthetic datasets.

A benchmark generator should accept a seed:

```text
seed = 42
```

This ensures that:

```text
same generator
+
same parameters
+
same seed
=
same dataset
```

This makes performance comparisons reproducible.

---

# 10. Synthetic Dataset Characteristics

A useful synthetic dataset should simulate realistic annotation behavior.

Example:

```text
Items:             100,000
Annotators:             20
Labels:                  5
Annotations/item:       5
Total annotations: 500,000
```

Annotator reliability can vary:

```text
worker_01 → 0.98
worker_02 → 0.94
worker_03 → 0.87
worker_04 → 0.61
```

This allows quality algorithms to operate on realistic disagreement patterns.

---

# 11. Benchmark Fixtures

Benchmark fixtures should be generated rather than manually embedded into benchmark source files.

Recommended structure:

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

Large generated datasets should not be committed to Git.

Instead:

```text
scripts/
└── generate-benchmark-data.py
```

can generate controlled datasets.

---

# 12. Benchmark Setup vs Measurement

Benchmark setup must not be included in the measured section unless the purpose of the benchmark is specifically to measure setup.

Incorrect:

```rust
group.bench_function("majority_vote", |b| {
    b.iter(|| {
        let dataset = generate_dataset();
        majority_vote(&dataset)
    });
});
```

This measures both:

```text
dataset generation
+
algorithm
```

Correct:

```rust
let dataset = generate_dataset();

group.bench_function("majority_vote", |b| {
    b.iter(|| {
        majority_vote(&dataset)
    });
});
```

This measures the algorithm itself.

---

# 13. Preventing Compiler Elimination

Benchmark results are invalid if the compiler can eliminate unused calculations.

Criterion should consume the result through `black_box` where appropriate.

Example:

```rust
use std::hint::black_box;

b.iter(|| {
    let result = majority_vote(black_box(&dataset));
    black_box(result);
});
```

The exact placement of `black_box` should reflect what is actually being measured.

---

# 14. Agreement Benchmarks

Agreement benchmarks should measure individual algorithms separately.

Example:

```text
agreement.rs
│
├── pairwise_agreement
├── cohens_kappa
├── fleiss_kappa
├── krippendorffs_alpha
└── agreement_matrix
```

This allows performance regressions to be isolated.

---

# 15. Pairwise Agreement Benchmark

A representative benchmark should vary:

```text
annotator count
item count
annotation density
```

Example cases:

```text
20 annotators × 10,000 items
20 annotators × 100,000 items
50 annotators × 100,000 items
```

The benchmark should report time per operation rather than only an aggregate suite duration.

---

# 16. Agreement Matrix Scaling

Agreement matrix computation has approximately:

[
O(M^2N)
]

complexity.

Therefore the benchmark should specifically test scaling with annotator count.

Example:

```text
10 annotators
20 annotators
50 annotators
100 annotators
```

while keeping the item count controlled.

This identifies the point at which pairwise analysis becomes computationally expensive.

---

# 17. Consensus Benchmarks

Consensus benchmarks should compare:

```text
majority vote
weighted vote
Dawid-Skene
```

Example workload:

```text
100,000 items
20 annotators
5 labels
```

Dawid-Skene should additionally vary iteration count.

```text
5 iterations
10 iterations
25 iterations
50 iterations
```

---

# 18. Dawid-Skene Benchmark

Dawid-Skene has iterative computational cost.

A useful measurement is:

```text
dataset size
×
number of labels
×
number of annotators
×
iterations
```

Benchmark results should therefore record configuration alongside timing.

Example:

```text
Items:       100,000
Annotators:       20
Labels:             5
Iterations:        25
```

---

# 19. Quality Benchmarks

Quality analysis should benchmark individual components.

```text
quality.rs
│
├── disagreement
├── entropy
├── annotator reliability
├── anomaly detection
└── review prioritization
```

This prevents a single composite benchmark from hiding which component is responsible for performance changes.

---

# 20. Ingestion Benchmarks

I/O benchmarks answer a different question from algorithm benchmarks.

They measure:

```text
file
 │
 ▼
parser
 │
 ▼
normalized dataset
```

They should not accidentally include statistical analysis.

Example:

```text
CSV file
   │
   ▼
CSV parser
   │
   ▼
Dataset
```

---

# 21. Parsing vs Normalization

Where practical, ingestion benchmarks should distinguish:

```text
parse-only
```

from:

```text
parse + normalize
```

This identifies whether performance limitations originate from:

* serialization
* validation
* allocation
* normalization
* indexing

---

# 22. Memory Benchmarking

Execution time alone is insufficient.

For large datasets, ACE should track:

```text
resident memory
peak allocation
dataset representation size
```

Memory measurements may use external operating-system tooling where Criterion alone is insufficient.

Example Linux tooling:

```bash
/usr/bin/time -v cargo bench -p ace-benchmarks
```

For detailed profiling, platform-specific tools may be used.

---

# 23. Benchmark Environment

A benchmark report should record:

```text
ACE version
Git commit
Rust version
Cargo version
operating system
CPU
CPU core count
RAM
benchmark dataset
dataset seed
compiler profile
```

Example:

```text
ACE version:      0.1.0
Rust:             1.85.0
OS:               Linux
CPU:              8-core x86_64
RAM:              16 GB
Dataset seed:     42
```

Without environment information, historical comparisons should be treated cautiously.

---

# 24. CPU Parallelism

ACE may use Rayon for parallel computation.

Benchmarks should compare:

```text
single-threaded
```

against:

```text
parallel
```

where the algorithm supports both.

Example:

```text
1 thread
2 threads
4 threads
8 threads
```

The objective is to measure scaling rather than assume parallelism automatically improves performance.

---

# 25. Parallel Scaling

A useful parallel scaling measurement is:

[
Speedup(p)=
\frac{T_1}{T_p}
]

where:

* (T_1) is single-thread execution time
* (T_p) is execution time using (p) threads

Parallel efficiency:

[
Efficiency(p)=
\frac{Speedup(p)}{p}
]

These measurements identify diminishing returns.

---

# 26. Benchmark Baselines

Each important algorithm should have a baseline.

Example:

```text
majority_vote/100k
baseline: 8.4 ms
```

After optimization:

```text
majority_vote/100k
new: 5.9 ms
```

The improvement should be evaluated statistically by Criterion rather than by comparing two individual runs.

---

# 27. Regression Detection

Performance regressions should be investigated when:

```text
execution time increases significantly
```

or:

```text
memory usage increases significantly
```

Possible causes include:

* unnecessary cloning
* changed data structures
* additional allocations
* loss of parallelism
* algorithmic complexity changes
* serialization overhead

A performance regression should not be ignored simply because functional tests still pass.

---

# 28. Benchmarking Before Optimization

The optimization process is:

```text
Implement
   │
   ▼
Validate correctness
   │
   ▼
Benchmark
   │
   ▼
Profile
   │
   ▼
Identify bottleneck
   │
   ▼
Optimize
   │
   ▼
Benchmark again
```

Optimization should target measured bottlenecks.

---

# 29. Profiling

When a benchmark identifies a bottleneck, profiling tools can be used.

Potential tools include:

```text
cargo-flamegraph
perf
Instruments
Windows Performance Analyzer
Valgrind
heaptrack
```

Tool selection depends on the development platform.

Profiling should be performed against representative workloads.

---

# 30. Statistical Interpretation

Criterion provides distributions rather than relying on one timing.

Important outputs include:

```text
mean
median
standard deviation
confidence intervals
outlier analysis
```

A benchmark should not be considered improved solely because one run was faster.

---

# 31. Benchmark Naming

Benchmark names should describe the operation and workload.

Good:

```text
majority_vote/100k_items/20_annotators
dawid_skene/100k_items/20_annotators/25_iterations
fleiss_kappa/1m_items/10_labels
```

Poor:

```text
test1
fast
benchmark
new
```

Consistent naming makes historical results easier to interpret.

---

# 32. Benchmark Parameters

Where possible, benchmark groups should use parameterized workloads.

Conceptually:

```rust
for items in [10_000, 100_000, 1_000_000] {
    // benchmark workload
}
```

This allows scaling behavior to be observed directly.

---

# 33. Avoiding Unrealistic Benchmarks

ACE should not optimize exclusively for artificially favorable workloads.

A useful benchmark suite should include:

```text
balanced labels
imbalanced labels
high agreement
low agreement
sparse annotation
dense annotation
few annotators
many annotators
few labels
many labels
```

This prevents performance claims from being based on one convenient dataset.

---

# 34. Correctness During Benchmarking

Benchmarks must still produce valid results.

A performance optimization that changes:

```text
consensus label
agreement score
quality score
```

incorrectly is not an optimization.

Benchmark code should therefore validate representative results against known expectations during development.

---

# 35. Benchmark Data Policy

Large benchmark datasets should not normally be committed to the repository.

Instead, store:

```text
dataset generator
configuration
random seed
schema
```

This provides reproducibility without unnecessarily increasing repository size.

---

# 36. Benchmark Reproducibility

A benchmark should be reproducible using:

```text
source commit
+
Rust version
+
benchmark configuration
+
dataset seed
+
hardware information
```

Example:

```text
Commit: 8d91f4a
Seed: 42
Items: 100000
Annotators: 20
Labels: 5
```

---

# 37. Benchmark Output

Criterion generates benchmark reports under Cargo's target directory.

Typical output:

```text
target/
└── criterion/
    ├── agreement/
    ├── consensus/
    ├── quality/
    └── ingestion/
```

HTML reports can be inspected locally after running:

```bash
cargo bench -p ace-benchmarks
```

---

# 38. CI Benchmarking

CI should initially focus on correctness and compilation.

Performance benchmarking in CI can be noisy because hosted runners vary in:

* CPU allocation
* background load
* virtualization
* memory
* thermal conditions

Therefore automated CI performance thresholds should only be introduced once a sufficiently stable environment is available.

---

# 39. Benchmark Release Process

Before a performance-focused release:

```text
1. Run complete test suite.
2. Run benchmark suite.
3. Record environment.
4. Compare against previous baseline.
5. Investigate significant regressions.
6. Record meaningful improvements.
7. Include benchmark results in release notes where appropriate.
```

---

# 40. Benchmark Checklist

Before accepting a benchmark:

```text
[ ] Production implementation is correct.
[ ] Workload is representative.
[ ] Dataset generation is deterministic.
[ ] Benchmark setup is outside measured code.
[ ] Result is consumed with black_box where required.
[ ] Release optimizations are enabled.
[ ] Dataset dimensions are recorded.
[ ] Environment is recorded.
[ ] Memory considerations are documented.
[ ] Results are reproducible.
```

---

# 41. Benchmark Directory Contract

The benchmark directory must remain separate from production source code:

```text
ace/
├── crates/
│   ├── ace-core/
│   ├── ace-io/
│   ├── ace-cli/
│   └── ace-report/
│
└── benchmarks/
    └── ace-benchmarks/
        ├── Cargo.toml
        └── benches/
            ├── agreement.rs
            ├── consensus.rs
            ├── quality.rs
            └── ingestion.rs
```

Do not place benchmark implementations inside:

```text
crates/ace-core/src/
```

or other production source directories.

---

# 42. Performance Goal

ACE should optimize for practical throughput rather than an arbitrary benchmark number.

The long-term objective is to make operations such as:

```text
large annotation ingestion
large-scale agreement analysis
consensus estimation
quality scoring
review prioritization
```

fast enough to be useful in real annotation pipelines.

Performance claims must always be tied to:

```text
workload
hardware
algorithm
configuration
measurement method
```

A statement such as:

> "ACE processes millions of annotations quickly"

is incomplete without the workload and benchmark conditions that produced the result.

```
