# ACE Benchmark Data

This directory contains metadata and documentation for datasets used to
benchmark ACE.

Large datasets should not be committed to the repository.

## Dataset Requirements

Benchmark datasets should document:

- number of items
- number of annotations
- number of annotators
- number of classes
- annotation format
- dataset size
- generation method
- license

## Reproducibility

Benchmark results should record:

- ACE version
- Rust version
- operating system
- CPU
- available memory
- dataset size
- algorithm
- configuration

## Synthetic Data

Synthetic datasets may be generated using:

```bash
python scripts/generate-test-data.py