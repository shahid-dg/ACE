#

````markdown
# Changelog

All notable changes to ACE are documented in this file.

I use this changelog to track the evolution of ACE across releases, including new functionality, improvements, bug fixes, performance changes, documentation updates, and breaking changes.

I follow [Semantic Versioning](https://semver.org/) where practical.

---

## [Unreleased]

This section contains changes that are implemented but have not yet been included in a released version.

### Added

- Initial Rust workspace structure.
- Core ACE project architecture.
- Annotation data model foundation.
- Input processing architecture.
- Output processing architecture.
- Initial configuration system.
- CLI foundation.
- Repository-level benchmark infrastructure.
- Initial project documentation.

### Changed

- No released changes yet.

### Fixed

- No released fixes yet.

### Performance

- No released performance changes yet.

### Documentation

- Added architecture documentation.
- Added algorithm documentation.
- Added data-model documentation.
- Added benchmarking documentation.
- Added configuration documentation.
- Added input-format documentation.
- Added output-format documentation.
- Added contribution documentation.

### Security

- Initial security policy established.

### Breaking Changes

- None.

---

## [0.1.0] - Unreleased

The first development release of ACE.

### Added

- Initial Cargo workspace.
- `ace-core` foundation.
- Canonical annotation dataset model.
- Annotation and annotator representations.
- Analysis result architecture.
- Input adapter architecture.
- Output adapter architecture.
- CLI foundation.
- Configuration infrastructure.
- Repository-level benchmark infrastructure.
- Initial test infrastructure.
- Initial documentation structure.

### Algorithms

The initial architecture provides the foundation for:

- inter-annotator agreement analysis
- consensus estimation
- annotator reliability analysis
- annotation quality scoring
- suspicious annotation detection

Algorithm-specific implementation details are documented in:

```text
docs/algorithms.md
````

### Input Formats

Initial input architecture targets:

* CSV
* JSON
* Parquet

Format-specific behavior is documented in:

```text
docs/input-formats.md
```

### Output Formats

Initial output architecture targets:

* JSON
* CSV
* HTML

Format-specific behavior is documented in:

```text
docs/output-formats.md
```

### Configuration

Initial configuration support is documented in:

```text
docs/configuration.md
```

### Benchmarking

Benchmark infrastructure is maintained separately from production source code under:

```text
benches/
```

Benchmarking methodology is documented in:

```text
docs/benchmarking.md
```

---

## Versioning

I use the following versioning rules.

### MAJOR

I increment the major version when I introduce incompatible changes to public APIs or externally visible behavior.

Examples include:

* removing public APIs
* incompatible CLI changes
* incompatible configuration changes
* incompatible output schemas
* breaking data-model changes

### MINOR

I increment the minor version when I introduce backward-compatible functionality.

Examples include:

* new algorithms
* new input formats
* new output formats
* new CLI commands
* new configuration options
* new report capabilities

### PATCH

I increment the patch version for backward-compatible fixes.

Examples include:

* incorrect calculations
* parsing fixes
* serialization fixes
* CLI bug fixes
* performance regressions
* documentation corrections

---

## Change Categories

I use these categories when applicable:

* `Added`
* `Changed`
* `Deprecated`
* `Removed`
* `Fixed`
* `Security`
* `Performance`
* `Documentation`

---

## Changelog Guidelines

I describe changes in terms of their effect on ACE rather than only describing internal implementation details.

For example:

```text
Improved consensus processing performance for large datasets.
```

is preferable to:

```text
Refactored ConsensusEngine::run().
```

When implementation details are important to developers, I include them alongside the user-facing description.

---

## Unreleased Changes

I keep active development changes under:

```text
[Unreleased]
```

When preparing a release, I move completed changes into the corresponding version section.

I do not rewrite historical release entries except to correct genuine documentation errors.

---

## Release Process

Before creating a release, I verify:

```text
[ ] Workspace builds
[ ] Tests pass
[ ] Formatting passes
[ ] Clippy passes
[ ] Release build succeeds
[ ] Relevant benchmarks pass
[ ] Documentation is updated
[ ] Configuration examples are valid
[ ] CLI behavior is verified
[ ] Output schemas are verified
[ ] Breaking changes are documented
[ ] Security considerations are reviewed
```

I then create the release entry and update the repository version.

---

## Release Comparison Links

Once ACE has tagged releases, I will maintain comparison links here.

Example:

```text
[Unreleased]: <repository-url>/compare/v0.1.0...HEAD
[0.1.0]: <repository-url>/releases/tag/v0.1.0
```
