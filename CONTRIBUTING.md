#

````markdown
# Contributing to ACE

I want ACE to remain a professional, maintainable, and technically rigorous Rust project.

Contributions should improve the project's correctness, performance, usability, security, or maintainability without introducing unnecessary complexity.

---

## 1. Before Contributing

I recommend reviewing the following documentation before making significant changes:

```text
README.md
docs/architecture.md
docs/algorithms.md
docs/data-model.md
docs/benchmarking.md
docs/configuration.md
docs/input-formats.md
docs/output-formats.md
docs/contributing.md
````

These documents define how I structure ACE and how its components interact.

---

## 2. Development Environment

I develop ACE using the stable Rust toolchain unless a specific component requires otherwise.

The primary development tools are:

```text
Rust
Cargo
rustfmt
Clippy
Git
```

I verify my environment with:

```bash
rustc --version
cargo --version
rustup show
```

---

## 3. Getting the Repository

I clone the repository with:

```bash
git clone <repository-url>
cd ace
```

I then verify the workspace:

```bash
cargo check --workspace
```

---

## 4. Development Workflow

I generally follow this workflow:

```text
Issue / Requirement
        │
        ▼
Understand the problem
        │
        ▼
Design the solution
        │
        ▼
Implement
        │
        ▼
Write tests
        │
        ▼
Update documentation
        │
        ▼
Format
        │
        ▼
Run Clippy
        │
        ▼
Benchmark where relevant
        │
        ▼
Review
```

I keep each change focused on a specific problem.

---

## 5. Branches

I use descriptive branch names.

Examples:

```text
feature/dawid-skene
feature/json-input
feature/html-report
feature/annotator-scoring
fix/csv-validation
fix/empty-dataset
perf/consensus
perf/agreement-matrix
docs/architecture
```

I avoid vague branch names such as:

```text
test
changes
new
stuff
update
```

---

## 6. Building ACE

For a normal development build:

```bash
cargo build --workspace
```

For an optimized build:

```bash
cargo build --workspace --release
```

I use release builds when evaluating production performance.

---

## 7. Running Tests

I run the complete workspace test suite with:

```bash
cargo test --workspace
```

When feature combinations are relevant:

```bash
cargo test --workspace --all-features
```

I do not consider a feature complete until its expected behavior is covered by appropriate tests.

---

## 8. Formatting

I format the complete workspace with:

```bash
cargo fmt --all
```

Before submitting a contribution, I verify formatting without modifying files:

```bash
cargo fmt --all -- --check
```

All committed Rust code should pass this check.

---

## 9. Clippy

I run:

```bash
cargo clippy --workspace --all-targets --all-features
```

I address relevant warnings rather than broadly suppressing them.

If a warning is intentionally ignored, I document the reason locally.

---

## 10. Testing Strategy

I use different levels of testing depending on the component.

### Unit Tests

I use unit tests for individual functions, algorithms, parsers, serializers, and internal components.

### Integration Tests

I use integration tests when multiple components need to work together.

### End-to-End Tests

I use end-to-end tests for complete workflows such as:

```text
Input
  ↓
Parsing
  ↓
Canonical Dataset
  ↓
Analysis
  ↓
Quality Results
  ↓
Output
```

### Regression Tests

When I fix a reproducible bug, I add a regression test whenever practical.

---

## 11. Statistical Algorithms

Statistical implementations require additional validation.

Before implementing an algorithm, I document:

```text
Mathematical definition
Assumptions
Required inputs
Expected outputs
Edge cases
Numerical considerations
```

I then validate the implementation against known examples or independently calculated results.

I prioritize correctness before optimization.

---

## 12. Numerical Testing

When testing floating-point results, I use appropriate tolerances rather than relying on exact equality where rounding makes exact comparison inappropriate.

The tolerance should be justified by the numerical behavior of the algorithm.

---

## 13. Edge Cases

I consider edge cases explicitly.

Typical cases include:

```text
Empty dataset
Single item
Single annotator
Two annotators
No disagreement
Complete disagreement
Missing labels
Duplicate annotations
Single-class dataset
Highly imbalanced labels
Large datasets
Malformed records
```

These cases should be represented in tests where they affect behavior.

---

## 14. Input Validation

Input adapters must validate external data before converting it into the canonical ACE data model.

I test cases such as:

```text
Missing columns
Invalid values
Empty fields
Malformed CSV
Malformed JSON
Unexpected types
Duplicate records
Invalid annotation identifiers
```

Invalid input should produce a useful error rather than an unexpected panic.

---

## 15. Output Validation

Output implementations should be tested independently.

I verify:

```text
Schema correctness
Required fields
Data types
Encoding
Escaping
Empty results
Large results
```

When an output format supports round-trip testing, I use it to verify that serialized data can be consumed correctly.

---

## 16. Error Handling

I avoid panics for expected runtime failures.

For example:

```rust
let dataset = load_dataset(path)?;
```

is preferred over:

```rust
let dataset = load_dataset(path).unwrap();
```

when the file can legitimately fail to load.

I use `unwrap()` or `expect()` only when an invariant guarantees that failure cannot reasonably occur.

---

## 17. Public APIs

I design public APIs deliberately.

Before exposing a new public type or function, I consider:

```text
Does another crate need it?
Is the API stable enough?
Can the interface be simplified?
What errors can it return?
Will future changes become difficult?
```

I avoid exposing implementation details unnecessarily.

---

## 18. Documentation

Public APIs should be documented where appropriate.

Example:

```rust
/// Calculates annotation agreement for the supplied dataset.
pub fn calculate_agreement(
    dataset: &Dataset,
) -> Result<AgreementResult, AgreementError> {
    // ...
}
```

Documentation should explain behavior, assumptions, errors, and important edge cases.

---

## 19. Adding an Algorithm

When I add a new algorithm, I follow:

```text
1. Define the mathematical behavior
2. Document the algorithm
3. Define input requirements
4. Implement the reference version
5. Add correctness tests
6. Validate known results
7. Add benchmarks
8. Optimize if necessary
9. Add regression tests
10. Update documentation
```

I do not optimize an algorithm before establishing that its implementation is correct.

---

## 20. Adding an Input Format

When I add a new input format, I follow:

```text
1. Define the format mapping
2. Implement the reader
3. Validate input
4. Add valid fixtures
5. Add malformed-input fixtures
6. Add parser tests
7. Add integration tests
8. Update input-format documentation
9. Update CLI/configuration where necessary
```

All supported formats should ultimately map into the same canonical ACE data model.

---

## 21. Adding an Output Format

When I add an output format, I follow:

```text
1. Define the output schema
2. Implement the writer
3. Add serialization tests
4. Test edge cases
5. Add integration tests
6. Add CLI support where required
7. Update output-format documentation
8. Add examples
```

Output implementations should consume analysis results rather than duplicate analysis logic.

---

## 22. Performance Work

I do not optimize based purely on assumptions.

I follow:

```text
Profile
   ↓
Identify bottleneck
   ↓
Implement optimization
   ↓
Benchmark
   ↓
Verify correctness
   ↓
Keep or revert
```

I only keep an optimization when the measured improvement justifies its added complexity.

---

## 23. Benchmarks

Benchmark code is maintained separately from production source code.

The benchmark directory is:

```text
benches/
```

For example:

```text
benches/
├── agreement.rs
├── consensus.rs
├── io.rs
└── scoring.rs
```

I do not place benchmark implementations inside `ace-core`.

I run benchmarks with:

```bash
cargo bench
```

or:

```bash
cargo bench --bench agreement
```

---

## 24. Benchmark Reproducibility

A meaningful benchmark should identify:

```text
Dataset size
Number of annotators
Number of labels
Algorithm configuration
Hardware
Rust version
Optimization settings
Random seed where applicable
```

I avoid making performance claims without reproducible measurements.

---

## 25. Dependencies

I add dependencies only when they provide clear value.

Before adding a crate, I consider:

```text
Maintenance status
License
Security history
Compile-time cost
Binary-size impact
Runtime performance
API quality
Existing project alternatives
```

I avoid adding a dependency simply to implement a small amount of straightforward functionality.

---

## 26. Dependency Updates

I prefer keeping dependency upgrades separate from unrelated feature work.

For example, I avoid combining:

```text
New algorithm
+
Major dependency upgrade
+
Unrelated refactoring
```

in the same change unless there is a strong reason.

---

## 27. Unsafe Rust

I minimize `unsafe` code.

When `unsafe` is necessary, I document:

```text
Why it is required
Which invariants must hold
Why safe Rust is insufficient
How it is tested
```

I keep unsafe sections as small and isolated as possible.

---

## 28. Security

I treat external input as untrusted.

This includes:

```text
Files
CSV
JSON
Parquet
CLI arguments
Configuration
Network requests
User-provided paths
```

Security-sensitive changes should also be reviewed against:

```text
SECURITY.md
```

---

## 29. Configuration Changes

When adding or changing configuration, I update:

```text
Configuration implementation
Validation
Example configuration
Tests
CLI help
docs/configuration.md
```

I do not add undocumented configuration options.

---

## 30. CLI Changes

The ACE CLI is a public interface.

Changes to:

```text
Commands
Arguments
Flags
Exit codes
Output formats
Configuration behavior
```

should be accompanied by appropriate tests and documentation.

---

## 31. Git Commits

I keep commits focused.

Good examples:

```text
Add CSV annotation reader
```

```text
Implement Fleiss kappa calculation
```

```text
Add flagged-item JSON serialization
```

I avoid vague commit messages such as:

```text
Update stuff
```

or:

```text
Various fixes
```

---

## 32. Pull Requests

A pull request should explain:

```text
What changed
Why it changed
How it was implemented
How it was tested
Performance impact
Configuration impact
Documentation impact
```

For performance-sensitive changes, I include benchmark results.

---

## 33. Pull Request Checklist

Before submitting a pull request, I verify:

```text
[ ] Implementation is focused
[ ] Code is formatted
[ ] cargo check passes
[ ] Tests pass
[ ] Clippy passes
[ ] New behavior has appropriate tests
[ ] Edge cases are covered
[ ] Documentation is updated
[ ] Configuration is updated if required
[ ] Benchmarks are updated if required
[ ] No secrets are committed
[ ] No unnecessary dependencies were added
[ ] No unrelated files were changed
```

---

## 34. Reporting Bugs

A useful bug report should include:

```text
ACE version:
Rust version:
Operating system:
Input format:
Dataset characteristics:
Configuration:
Command used:
Expected behavior:
Actual behavior:
Error message:
Steps to reproduce:
```

A minimal reproducible example is preferred.

I do not expect users to provide private or sensitive datasets.

---

## 35. Feature Requests

A feature request should explain:

```text
Problem
Current limitation
Proposed behavior
Expected users
Expected inputs
Expected outputs
```

For larger features, I also evaluate:

```text
Performance impact
API impact
Maintenance cost
Security implications
Compatibility
```

---

## 36. Breaking Changes

I clearly identify breaking changes.

Examples include:

```text
Removing public APIs
Changing public APIs
Changing CLI behavior
Changing configuration semantics
Changing input compatibility
Changing output schemas
Changing the canonical data model
```

Breaking changes must also be documented in:

```text
CHANGELOG.md
```

---

## 37. Code Review

I review contributions for:

```text
Correctness
Maintainability
Test coverage
Performance
API quality
Error handling
Security
Documentation
Dependency impact
```

I prefer simple, explicit, testable implementations over clever or unnecessarily abstract designs.

---

## 38. Reproducibility

I aim to make ACE analysis reproducible.

Important inputs include:

```text
Dataset
ACE version
Configuration
Algorithm parameters
Random seed
```

Where an algorithm uses randomness, I expose deterministic seeding when practical.

---

## 39. Generated Files

I do not commit generated or machine-specific artifacts unless they are intentionally part of the repository.

Typical generated files include:

```text
target/
coverage/
temporary reports
benchmark output
temporary datasets
local configuration
```

---

## 40. Secrets

I never commit:

```text
API keys
Passwords
Tokens
Private keys
Credentials
Personal access tokens
```

If a feature requires secrets, I document the required environment variables without committing their values.

---

## 41. Definition of Done

I consider a feature complete when the relevant work has been addressed across:

```text
Implementation
+
Tests
+
Error handling
+
Documentation
+
Performance validation where relevant
```

A feature that merely compiles is not considered complete.

---

## 42. Final Standard

I want every significant ACE component to make these questions answerable:

```text
What does it do?
Why does it exist?
What assumptions does it make?
How is it tested?
How does it fail?
How does it perform?
How is it used?
```

The standard I use is:

```text
Correct
   +
Tested
   +
Documented
   +
Measured
   +
Maintainable
```
