#

````markdown
# Contributing to ACE

## 1. Purpose

I want ACE to remain a production-quality Rust project rather than becoming a collection of disconnected experiments.

This document defines how I structure contributions, develop features, write tests, document changes, and maintain code quality.

The goal is to make every contribution understandable, reproducible, testable, and maintainable.

---

## 2. Project Philosophy

I follow several principles when developing ACE:

- correctness before optimization
- measurable performance improvements
- explicit APIs
- strong typing
- deterministic behavior
- comprehensive testing
- minimal dependencies
- clear documentation
- separation of concerns
- reproducible benchmarks

Performance matters because ACE is designed to process large annotation datasets, but I do not sacrifice correctness merely to achieve better benchmark numbers.

---

## 3. Repository Structure

The repository is organized as a Rust workspace.

The current high-level structure is:

```text
ace/
├── crates/
│   ├── ace-cli/
│   ├── ace-core/
│   ├── ace-io/
│   └── ace-report/
│
├── benches/
│   ├── agreement.rs
│   ├── consensus.rs
│   ├── io.rs
│   └── scoring.rs
│
├── configs/
├── docs/
├── examples/
├── tests/
├── Cargo.toml
├── Cargo.lock
└── README.md
````

I keep benchmark code outside `ace-core` so performance experiments remain separate from production source code.

---

## 4. Development Environment

I develop ACE using a stable Rust toolchain unless a specific feature requires another toolchain.

The primary tools are:

```text
Rust
Cargo
rustfmt
Clippy
Git
```

I verify the installed environment with:

```bash
rustc --version
cargo --version
rustup show
```

---

## 5. Getting the Repository

I clone the repository with:

```bash
git clone <repository-url>
cd ace
```

I then verify that the workspace builds:

```bash
cargo check --workspace
```

---

## 6. Building the Project

For a normal development build:

```bash
cargo build --workspace
```

For an optimized build:

```bash
cargo build --workspace --release
```

I use release builds when evaluating actual runtime performance.

Debug builds are useful during normal development because they provide faster compilation and better debugging information.

---

## 7. Running ACE

After building the CLI, I can run:

```bash
cargo run -p ace-cli -- --help
```

For an analysis:

```bash
cargo run -p ace-cli -- analyze ./examples/data/annotations.csv
```

The exact CLI commands may evolve as the interface develops, but the CLI remains the primary integration point for users.

---

## 8. Development Workflow

I use the following workflow for changes:

```text
Issue / Feature
      │
      ▼
Understand Requirement
      │
      ▼
Design
      │
      ▼
Implement
      │
      ▼
Unit Tests
      │
      ▼
Integration Tests
      │
      ▼
Formatting
      │
      ▼
Clippy
      │
      ▼
Benchmarks
      │
      ▼
Documentation
      │
      ▼
Commit
```

I avoid making unrelated changes in the same contribution.

---

## 9. Branches

I use focused branches for development.

Examples:

```text
feature/json-input
feature/dawid-skene
feature/html-report
fix/csv-validation
perf/agreement-matrix
docs/architecture
```

A branch should represent one logical change whenever practical.

---

## 10. Small Changes

For small fixes, I keep the implementation narrow.

For example:

```text
Fix CSV missing-column validation
```

should not also introduce:

```text
new statistical algorithm
new report layout
dependency upgrades
unrelated refactoring
```

Keeping changes focused makes reviews and debugging easier.

---

## 11. Feature Development

For a new feature, I first define:

```text
purpose
input
output
API
failure behavior
performance expectations
tests
documentation
```

For example, a new agreement metric should define:

```text
Metric name
Supported label types
Required inputs
Mathematical definition
Edge cases
Expected complexity
Numerical behavior
Test cases
```

---

## 12. Statistical Implementations

Statistical algorithms require additional care.

Before implementing an algorithm, I document:

```text
mathematical definition
assumptions
input requirements
output interpretation
edge cases
numerical constraints
reference implementation
```

I then compare the implementation against known examples or trusted reference calculations.

The implementation should not be considered complete merely because it compiles.

---

## 13. Correctness First

For statistical code, correctness takes priority over raw performance.

The development order is:

```text
Correct implementation
        │
        ▼
Tests
        │
        ▼
Profiling
        │
        ▼
Optimization
        │
        ▼
Benchmark
        │
        ▼
Regression test
```

I do not introduce SIMD, unsafe code, or complex optimizations without evidence that they provide a meaningful benefit.

---

## 14. Rust Style

I format Rust code using:

```bash
cargo fmt --all
```

I expect committed Rust code to be `rustfmt` compliant.

I avoid manually formatting code in ways that fight the formatter.

---

## 15. Clippy

I run:

```bash
cargo clippy --workspace --all-targets --all-features
```

I treat Clippy warnings seriously.

Where a warning is intentionally ignored, I document why rather than suppressing warnings broadly.

---

## 16. Tests

I run the complete test suite with:

```bash
cargo test --workspace
```

Before submitting a significant change, I also run:

```bash
cargo test --workspace --all-features
```

Tests should cover both normal behavior and failure conditions.

---

## 17. Unit Tests

Unit tests belong close to the code they test.

For example:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn calculates_expected_agreement() {
        // ...
    }
}
```

Unit tests should verify individual functions and components.

---

## 18. Integration Tests

Integration tests live outside individual crates where appropriate.

The repository can contain:

```text
tests/
├── cli.rs
├── input_formats.rs
├── output_formats.rs
└── end_to_end.rs
```

Integration tests verify that multiple components work together correctly.

---

## 19. End-to-End Tests

I use end-to-end tests for important workflows.

For example:

```text
CSV
 │
 ▼
Input Adapter
 │
 ▼
ACE Dataset
 │
 ▼
Consensus
 │
 ▼
Quality Analysis
 │
 ▼
JSON Output
```

An end-to-end test should verify the complete result rather than only individual functions.

---

## 20. Test Fixtures

I keep representative datasets under:

```text
tests/
└── fixtures/
```

Example:

```text
tests/
└── fixtures/
    ├── csv/
    ├── json/
    ├── parquet/
    ├── small/
    └── large/
```

Fixtures should remain small enough to understand and deterministic enough to reproduce failures.

---

## 21. Regression Tests

Whenever I fix a reproducible bug, I add a regression test when practical.

The preferred pattern is:

```text
Bug
 │
 ▼
Minimal Reproduction
 │
 ▼
Regression Test
 │
 ▼
Implementation Fix
```

This prevents the same failure from silently returning later.

---

## 22. Property Testing

For algorithms with strong mathematical invariants, I can use property-based testing.

Examples include:

```text
agreement score remains within valid bounds
probabilities remain normalized
empty datasets do not produce invalid values
permutation of annotator ordering does not change symmetric metrics
```

Property tests complement normal example-based tests.

They do not replace them.

---

## 23. Numerical Testing

Floating-point algorithms require tolerance-aware comparisons.

I avoid tests such as:

```rust
assert_eq!(actual, expected);
```

when floating-point rounding makes exact equality inappropriate.

Instead, I compare values within a defined tolerance.

The tolerance should be justified by the algorithm rather than chosen arbitrarily.

---

## 24. Edge Cases

Every algorithm should consider edge cases such as:

```text
empty dataset
one item
one annotator
two annotators
no disagreement
complete disagreement
missing labels
single-class datasets
highly imbalanced labels
duplicate annotations
very large datasets
```

These cases often expose assumptions that are invisible in normal examples.

---

## 25. Input Testing

Input adapters must test malformed data.

Examples:

```text
missing columns
empty fields
invalid encoding
invalid JSON
invalid CSV
unexpected types
duplicate annotations
oversized records
```

I verify that failures produce useful errors instead of panics.

---

## 26. Output Testing

Output serializers must be tested independently.

For JSON:

```bash
cargo test -p ace-report json
```

For CSV:

```bash
cargo test -p ace-report csv
```

For HTML:

```bash
cargo test -p ace-report html
```

The exact test filters may change as the implementation grows.

---

## 27. No Silent Data Loss

Input and output implementations must never silently discard data.

If a record cannot be represented safely, I prefer:

```text
explicit error
```

over:

```text
silent conversion
```

or:

```text
silent omission
```

This is particularly important for annotation-quality analysis because losing a subset of annotations can alter the statistical conclusions.

---

## 28. Error Handling

Production code should use structured errors rather than `unwrap()` for recoverable failures.

For example:

```rust
let dataset = load_dataset(path)?;
```

is preferred over:

```rust
let dataset = load_dataset(path).unwrap();
```

`unwrap()` may be appropriate when an invariant is guaranteed internally, but such usage should be deliberate.

---

## 29. Panics

Library code should avoid panicking on user-controlled input.

Invalid:

```text
malformed CSV
invalid JSON
missing configuration
invalid dataset
```

should result in errors.

A panic should represent a violated internal invariant rather than normal user error.

---

## 30. Public APIs

Public APIs should be intentionally designed.

Before exposing a new public type or function, I consider:

```text
Does another crate need it?
Is the API stable enough?
Can the type be simplified?
What errors can it produce?
Will changing it create compatibility problems?
```

I avoid exposing internal implementation details unnecessarily.

---

## 31. Documentation

Public Rust APIs should contain documentation where appropriate.

Example:

```rust
/// Calculates inter-annotator agreement for the supplied dataset.
pub fn calculate_agreement(
    dataset: &Dataset,
) -> Result<AgreementResult, AgreementError> {
    // ...
}
```

Documentation should explain behavior, assumptions, and important edge cases.

---

## 32. Documentation Changes

When behavior changes, I update the relevant documentation.

Examples:

```text
new input format
        → docs/input-formats.md

new output format
        → docs/output-formats.md

algorithm change
        → docs/algorithms.md

architecture change
        → docs/architecture.md

configuration change
        → docs/configuration.md
```

Documentation is part of the implementation, not an optional final step.

---

## 33. Configuration Changes

When adding configuration, I update:

```text
configuration documentation
example configuration
validation logic
tests
CLI help
```

I avoid adding undocumented configuration options.

---

## 34. Dependencies

I add dependencies only when they provide clear value.

Before adding a crate, I consider:

```text
maintenance status
license
security history
compile-time cost
binary-size impact
performance
API quality
whether the functionality can reasonably be implemented internally
```

I do not add a dependency simply to avoid writing a small amount of straightforward code.

---

## 35. Dependency Updates

Dependency upgrades should be isolated when practical.

Instead of combining:

```text
feature implementation
+
large dependency upgrade
+
unrelated refactoring
```

I prefer separate changes.

This makes regressions easier to identify.

---

## 36. Security

Security-sensitive code receives additional review.

This includes:

```text
file handling
deserialization
network services
authentication
cryptographic operations
temporary files
path handling
external process execution
```

User-controlled input must never be trusted by default.

---

## 37. Unsafe Rust

I minimize `unsafe` code.

If `unsafe` is necessary for performance or interoperability, I document:

```text
why it is required
what invariants must hold
why safe Rust is insufficient
how it is tested
```

Unsafe blocks should remain as small and isolated as practical.

---

## 38. Performance Work

I do not optimize based on assumptions.

The preferred process is:

```text
Profile
  │
  ▼
Identify bottleneck
  │
  ▼
Implement optimization
  │
  ▼
Benchmark
  │
  ▼
Verify correctness
```

A change should demonstrate measurable improvement before I keep it solely for performance reasons.

---

## 39. Benchmarks

Benchmark code lives at the repository level:

```text
benches/
├── agreement.rs
├── consensus.rs
├── io.rs
└── scoring.rs
```

I keep benchmarks separate from production source code.

This keeps `ace-core` focused on runtime functionality while providing a dedicated environment for performance measurement.

---

## 40. Running Benchmarks

I run benchmarks with:

```bash
cargo bench
```

For a specific benchmark:

```bash
cargo bench --bench agreement
```

Benchmarks should use deterministic datasets where possible.

---

## 41. Benchmark Requirements

A meaningful benchmark should document:

```text
dataset size
number of annotators
number of labels
hardware
Rust version
optimization settings
algorithm configuration
```

Without this context, benchmark numbers are difficult to reproduce or compare.

---

## 42. Performance Claims

I do not claim:

```text
10x faster
100x faster
millions of labels per second
```

without benchmark evidence.

Performance claims in the README or documentation should be backed by reproducible benchmark results.

---

## 43. Benchmark Fixtures

Large benchmark datasets should not be committed unnecessarily.

When practical, I generate deterministic synthetic datasets.

Example:

```text
1,000 items
10 annotators
5 labels
10,000 annotations
```

The generator should use a fixed seed when reproducibility is required.

---

## 44. Git Commits

I keep commits focused.

Good:

```text
Add CSV annotation reader
```

```text
Implement Fleiss kappa calculation
```

```text
Add flagged-item JSON serialization
```

Less useful:

```text
Update stuff
```

or:

```text
Various fixes
```

Commit messages should describe the actual change.

---

## 45. Pull Requests

A pull request should explain:

```text
what changed
why it changed
how it was implemented
how it was tested
whether performance changed
whether configuration changed
whether documentation changed
```

For performance-sensitive changes, I include benchmark results.

---

## 46. Pull Request Checklist

Before opening a pull request, I run:

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets --all-features
```

For performance-sensitive changes:

```bash
cargo bench
```

I also verify that documentation and configuration examples remain correct.

---

## 47. Review Criteria

I review contributions against:

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

A feature that works but introduces unnecessary complexity may require redesign.

---

## 48. Breaking Changes

I treat breaking API changes carefully.

A breaking change may include:

```text
renaming public types
removing public functions
changing serialized schemas
changing configuration semantics
changing CLI behavior
```

Breaking changes should be clearly documented.

---

## 49. CLI Compatibility

The CLI is a public interface.

Changes to:

```text
commands
flags
configuration
exit codes
output formats
```

should be treated as compatibility-sensitive.

When changing CLI behavior, I update examples and integration tests.

---

## 50. Data Compatibility

Input compatibility is important because annotation exports may be produced by external systems.

When changing input parsing, I test existing fixtures before changing behavior.

If a previously accepted format becomes invalid, I document the reason.

---

## 51. Reproducibility

I aim for reproducible development and analysis.

Important inputs include:

```text
dataset
configuration
ACE version
Rust version
algorithm parameters
random seed
```

Where an algorithm contains randomness, I expose deterministic seeding when practical.

---

## 52. Randomized Algorithms

If an algorithm uses randomness, I avoid relying on implicit global randomness.

Instead, I use an explicit random-number generator or seed where appropriate.

Example conceptually:

```toml
[analysis]
seed = 42
```

This allows experiments to be repeated.

---

## 53. Code Review Philosophy

I prefer code that is:

```text
simple
explicit
testable
measurable
```

over code that is:

```text
clever
over-abstracted
prematurely optimized
difficult to test
```

Complexity must have a reason.

---

## 54. Adding a New Algorithm

When I add an algorithm, I follow:

```text
1. Define mathematical behavior
2. Add documentation
3. Define input requirements
4. Implement reference version
5. Add correctness tests
6. Validate against known results
7. Add benchmarks
8. Optimize if required
9. Add regression tests
10. Update README/docs
```

This keeps algorithm development disciplined.

---

## 55. Adding a New Input Format

When I add a format:

```text
1. Define canonical mapping
2. Implement reader
3. Add validation
4. Add malformed-input tests
5. Add fixtures
6. Add integration tests
7. Document the format
8. Add CLI/configuration support
```

The new format must produce the same canonical ACE data model as existing formats.

---

## 56. Adding a New Output Format

When I add an output format:

```text
1. Define serialization schema
2. Implement writer
3. Add output tests
4. Verify round-trip/readability where applicable
5. Add CLI support
6. Document the format
7. Add examples
```

The serializer must consume the existing analysis result rather than duplicating analysis logic.

---

## 57. Adding a New Report Section

For HTML reporting, I first define:

```text
purpose
data source
visual representation
interaction
fallback behavior
test requirements
```

Then I implement the section and update the report documentation.

---

## 58. Generated Files

Generated artifacts should not be committed unless they are intentionally part of the repository.

Typical ignored files include:

```text
target/
coverage/
benchmark output
generated reports
temporary datasets
local configuration
```

Source fixtures and documentation examples may be committed when they are required for reproducibility.

---

## 59. Environment Files

Secrets and machine-specific configuration must never be committed.

Examples:

```text
.env
credentials
API keys
private certificates
local datasets
```

If configuration requires secrets, I document the expected environment variables without committing their values.

---

## 60. Issue Reporting

A useful bug report should contain:

```text
ACE version
Rust version
operating system
input format
dataset characteristics
configuration
command used
expected result
actual result
error message
minimal reproduction
```

The smallest reproducible example is preferred.

---

## 61. Feature Requests

A feature request should explain:

```text
problem
current limitation
proposed behavior
expected users
expected input/output
```

For substantial features, I also consider:

```text
performance impact
API impact
maintenance cost
security implications
```

---

## 62. Reproducing a Bug

I aim to reduce reported failures to the smallest possible case.

For example:

```text
Large dataset
      │
      ▼
Small dataset
      │
      ▼
Single failing record
      │
      ▼
Minimal regression test
```

This makes the eventual fix easier to understand and maintain.

---

## 63. Release Readiness

Before a release, I verify:

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets --all-features
cargo build --workspace --release
```

I also verify:

```text
README
documentation
configuration examples
CLI help
output schemas
benchmark results
version numbers
```

---

## 64. Release Documentation

A release should document meaningful changes.

Examples:

```text
New algorithms
New input formats
New output formats
Performance improvements
Bug fixes
Breaking changes
Configuration changes
```

I avoid claiming improvements that have not been measured.

---

## 65. Quality Standard

I consider a feature complete only when:

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

are all addressed.

A feature that merely compiles is not considered finished.

---

## 66. Contribution Checklist

Before considering my own contribution complete, I verify:

```text
[ ] Implementation is focused
[ ] Public APIs are documented
[ ] Errors are handled
[ ] Unit tests exist
[ ] Integration tests exist where needed
[ ] Edge cases are covered
[ ] Formatting passes
[ ] Clippy passes
[ ] Benchmarks exist where relevant
[ ] Documentation is updated
[ ] Configuration is updated where relevant
[ ] No secrets are committed
[ ] No unnecessary dependencies were added
[ ] No unrelated files were modified
```

---

## 67. Final Development Standard

I build ACE as a system that should be understandable by another Rust developer without requiring knowledge of the original implementation process.

Every important component should therefore answer:

```text
What does it do?
Why does it exist?
What assumptions does it make?
How is it tested?
How does it fail?
How does it perform?
How does another component use it?
```

The project remains maintainable when those answers are visible in the code, tests, benchmarks, and documentation.

My standard is:

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

That is the contribution standard I use for ACE.

```
```
