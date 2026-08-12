#

````markdown
# Security Policy

## 1. Security Philosophy

I treat security as a core requirement of ACE.

ACE processes annotation datasets that may contain proprietary, private, or sensitive information. I therefore design the system around explicit input validation, controlled resource usage, safe filesystem operations, dependency security, and predictable failure behavior.

I treat all externally supplied data as untrusted.

Security applies across:

```text
Input parsing
Data validation
Configuration
File handling
Deserialization
CLI arguments
Report generation
Network services
Dependencies
Temporary files
Logging
FFI boundaries
Unsafe Rust
```

````
---

## 2. Supported Versions

I provide security fixes for actively maintained releases.

| Version                     | Security Support          |
| --------------------------- | ------------------------- |
| Latest stable release       | Supported                 |
| Previous maintained release | Supported where practical |
| Development branch          | Best effort               |
| Unmaintained releases       | Not supported             |

The exact supported versions are maintained as ACE begins publishing official releases.

---

## 3. Reporting a Vulnerability

I ask security researchers not to publicly disclose unpatched vulnerabilities through:

```text
GitHub Issues
GitHub Discussions
Pull Requests
Public forums
Social media
```

Security vulnerabilities should be reported privately through the repository's configured security-reporting mechanism.

If private reporting has not yet been configured, the researcher should contact the project maintainer privately using the security contact published in the repository.

---

## 4. Security Report Requirements

A useful security report should contain:

```text
Affected ACE version
Affected component
Vulnerability type
Attack scenario
Steps to reproduce
Expected behavior
Actual behavior
Potential impact
Proof of concept
Suggested mitigation
```

I prefer a minimal reproducible example.

I do not require real production datasets to reproduce security problems.

---

## 5. Sensitive Data

I do not want real private or confidential datasets included in security reports.

I prefer:

```text
Synthetic datasets
Redacted records
Minimal reproductions
Generated fixtures
```

I ask reporters to remove:

```text
Passwords
API keys
Authentication tokens
Private keys
Personal information
Private identifiers
Confidential business data
```

from reports whenever possible.

---

## 6. Trust Boundaries

I define the following as untrusted input:

```text
CSV files
JSON files
Parquet files
Configuration files
CLI arguments
File paths
Dataset metadata
Annotation labels
Annotator identifiers
HTTP requests
gRPC requests
FFI input
```

The expected processing flow is:

```text
Untrusted Input
      │
      ▼
Parsing
      │
      ▼
Validation
      │
      ▼
Canonical ACE Data Model
      │
      ▼
Analysis
      │
      ▼
Validated Result
      │
      ▼
Output
```

I do not allow unvalidated external data to bypass the canonical validation layer.

---

## 7. Input Validation

I validate external input before using it in analysis or filesystem operations.

Validation covers:

```text
Required fields
Data types
Identifier validity
Record structure
Annotation structure
Label values
Dataset consistency
Numeric ranges
File format integrity
Configuration values
```

Invalid input produces a structured ACE error.

Invalid input does not cause an unexpected panic.

---

## 8. Malformed Data

I explicitly test malformed and hostile input.

Examples include:

```text
Empty files
Truncated files
Malformed CSV
Malformed JSON
Corrupted Parquet
Missing fields
Unexpected fields
Invalid types
Invalid UTF-8
Extremely long strings
Invalid identifiers
Duplicate records
Invalid annotation relationships
```

The expected behavior is:

```text
Invalid Input
     │
     ▼
Validation Failure
     │
     ▼
Structured Error
     │
     ▼
Safe Termination
```

The system must not treat malformed data as successfully processed data.

---

## 9. Resource Limits

ACE processes potentially large annotation datasets, so I implement explicit controls for resource consumption.

The configuration system supports limits for resource-intensive operations.

Supported limits include:

```text
Maximum input file size
Maximum number of records
Maximum number of annotations
Maximum number of annotators
Maximum worker threads
Maximum concurrent analysis operations
Maximum report size where applicable
```

These limits are configured through the ACE configuration system.

The configuration behavior is documented in:

```text
docs/configuration.md
```

When ACE can determine that a workload exceeds a configured limit before processing begins, it rejects the workload before starting the expensive operation.

When the required resource usage cannot be determined in advance, ACE tracks the relevant workload counters during processing.

When a limit is reached:

```text
Processing stops
      │
      ▼
Structured resource-limit error
      │
      ▼
No successful result is returned
```

ACE does not silently continue processing after a configured limit has been exceeded.

I test each resource limit with workloads specifically designed to exceed the configured threshold.

Tests verify:

```text
[ ] The configured limit is enforced
[ ] Processing stops at the limit
[ ] A structured error is returned
[ ] No partial result is reported as successful
[ ] Temporary resources are cleaned up
[ ] Normal workloads remain unaffected
```

---

## 10. CPU and Parallelism Controls

ACE uses parallel processing for workloads where parallelism improves performance.

I do not allow unbounded worker creation.

The configured worker limit controls the maximum number of analysis workers ACE may create for supported operations.

For example:

```text
workers = 8
```

means ACE may use up to eight analysis workers for operations controlled by that setting.

I validate worker configuration values before starting processing.

Invalid values produce configuration errors.

---

## 11. Memory Consumption

ACE avoids unnecessary copies of large datasets.

For large workloads I prefer:

```text
Borrowing
Streaming
Chunked processing
Preallocated collections where appropriate
Bounded parallelism
Compact representations
```

I do not claim that ACE uses constant memory for every algorithm.

Algorithms with inherently memory-intensive operations must document their memory characteristics.

Where an algorithm requires an in-memory structure, ACE validates applicable dataset or record limits before beginning the operation.

---

## 12. Expensive Algorithms

Some annotation-analysis operations can grow rapidly with dataset size.

Examples include:

```text
Pairwise annotator comparison
Agreement matrices
Consensus computation
Similarity calculations
Graph-based analysis
Large anomaly-detection operations
```

I document the computational characteristics of these operations in:

```text
docs/algorithms.md
```

I apply dataset and workload limits before expensive operations whenever the required workload size can be calculated.

---

## 13. File Handling

ACE may read from and write to user-specified filesystem paths.

I validate paths before performing filesystem operations.

I specifically protect against:

```text
Path traversal
Unexpected absolute paths where disallowed
Unintended file overwrites
Unsafe temporary paths
Unexpected directory access
```

ACE does not interpret dataset contents as filesystem commands.

---

## 14. Output File Handling

When writing output, ACE explicitly determines the destination path and output format.

The output layer does not overwrite an existing file unless the configured behavior explicitly permits overwriting.

Where overwrite protection is enabled:

```text
Existing File
     │
     ▼
Write Attempt
     │
     ▼
Operation Rejected
```

This prevents accidental destruction of existing analysis results.

---

## 15. Temporary Files

I minimize the use of temporary files.

When temporary files are required, ACE uses operating-system-supported temporary-file mechanisms where available.

I do not create predictable temporary filenames based solely on user input.

Temporary files are removed when processing completes or when an operation terminates with an error.

Cleanup is also performed when the operation exits through an error path.

---

## 16. Deserialization

External serialized data is parsed into validated internal structures.

I do not deserialize external input directly into trusted application state without validation.

The process is:

```text
Serialized Data
      │
      ▼
Deserializer
      │
      ▼
Schema Validation
      │
      ▼
Semantic Validation
      │
      ▼
ACE Data Model
```

Schema-valid data is not automatically considered semantically valid.

---

## 17. HTML Report Security

ACE generates HTML reports containing analysis results and potentially user-controlled dataset values.

I escape user-controlled values before inserting them into HTML.

Annotation labels, annotator identifiers, metadata, and other dataset values are treated as text unless the output system explicitly defines a trusted representation.

I do not allow dataset content to become executable HTML or JavaScript merely because it contains HTML-like characters.

---

## 18. Report Data Exposure

Generated reports may contain information such as:

```text
Annotator identifiers
Annotation identifiers
Quality scores
Flagged annotations
Dataset statistics
Labels
Metadata
Error information
```

I therefore treat generated reports as potentially sensitive artifacts.

ACE does not assume that a generated report is safe to publish publicly.

Users are responsible for determining whether their generated reports can be shared.

---

## 19. Logging

I keep logs useful for diagnosing failures without unnecessarily exposing dataset contents.

I do not intentionally log:

```text
Passwords
API keys
Authentication tokens
Private keys
Complete sensitive records
Unnecessary dataset contents
```

For dataset-processing errors, I prefer reporting:

```text
File
Record location
Field
Error type
Validation failure
```

rather than dumping the complete record into logs.

---

## 20. Error Handling

ACE uses structured errors for expected failures.

Expected failures include:

```text
Invalid input
Missing files
Permission errors
Invalid configuration
Unsupported formats
Resource limits
Serialization failures
Network failures
```

These failures should not require a process panic.

I avoid `unwrap()` and `expect()` on operations that can legitimately fail at runtime.

---

## 21. CLI Security

CLI arguments are external input.

I validate CLI values before using them for:

```text
Filesystem access
Configuration loading
Dataset processing
Report generation
Network connections
```

ACE does not construct shell commands from untrusted CLI arguments.

---

## 22. Configuration Security

Configuration values are validated before they are applied.

Validation includes:

```text
Allowed ranges
Required values
Enum values
Path validity
Resource limits
Worker counts
Output settings
Input settings
```

Invalid configuration causes ACE to stop before analysis begins.

Configuration behavior is documented in:

```text
docs/configuration.md
```

---

## 23. Network Services

If a network-facing ACE service is enabled, I treat every request as untrusted.

Network endpoints validate:

```text
Request size
Request structure
Input types
Authentication state where required
Authorization where required
Timeouts
Concurrency
```

Network services should not expose internal filesystem paths, credentials, stack traces, or other sensitive implementation details through API responses.

---

## 24. Authentication and Authorization

If ACE exposes protected network functionality, authentication and authorization are separate concerns.

Authentication determines:

```text
Who is making the request?
```

Authorization determines:

```text
What is that requester allowed to access?
```

I do not consider authentication alone sufficient protection for sensitive operations.

---

## 25. Network Resource Controls

Network-facing components use bounded resources.

Where applicable, I configure:

```text
Request size limits
Connection limits
Request timeouts
Worker limits
Concurrent operation limits
```

Requests that exceed configured limits are rejected with an appropriate error.

---

## 26. Secrets

I never commit secrets to the repository.

Secrets include:

```text
API keys
Passwords
Tokens
Private keys
Cloud credentials
Database credentials
Signing keys
```

Secrets required by development or deployment environments should be supplied through appropriate environment variables or secret-management systems.

---

## 27. Repository Secret Protection

Before committing changes, I verify that sensitive local files are not included.

Examples include:

```text
.env
Credential files
Private keys
Local authentication files
Cloud configuration containing secrets
```

Repository ignore rules should protect common local secret files.

---

## 28. Dependencies

Third-party dependencies are part of ACE's security boundary.

Before adding a dependency, I evaluate:

```text
Maintenance status
Known vulnerabilities
License
Dependency tree
Unsafe code
Security history
Release practices
```

I avoid adding dependencies when the required functionality is small enough to implement safely without introducing another dependency.

---

## 29. Dependency Vulnerabilities

When a dependency vulnerability affects ACE, I:

```text
1. Identify the affected dependency
2. Determine the affected ACE versions
3. Determine whether ACE is actually exposed
4. Upgrade or replace the dependency
5. Run tests
6. Run Clippy
7. Build release artifacts
8. Add regression coverage where appropriate
9. Document the change
```

---

## 30. Unsafe Rust

I minimize `unsafe` Rust.

When `unsafe` is required, I document:

```text
Why unsafe is required
Required invariants
Memory-safety assumptions
Why safe Rust is insufficient
How the code is tested
```

I keep unsafe blocks small and isolate them from higher-level application logic.

---

## 31. FFI Boundaries

If ACE exposes or consumes foreign-function interfaces, I treat the FFI boundary as security-sensitive.

I validate:

```text
Pointers
Buffer lengths
String encoding
Ownership
Lifetimes
Nullability
Return values
Error states
```

I do not assume that foreign code satisfies Rust's safety guarantees.

---

## 32. Cryptography

ACE does not implement custom cryptographic primitives unless there is a specific, justified requirement.

When cryptography is required, I prefer established, actively maintained cryptographic libraries.

I do not consider a cryptographic implementation safe merely because its mathematical operation appears simple.

---

## 33. Randomness

I distinguish between statistical randomness and security-sensitive randomness.

For statistical algorithms, I use appropriate deterministic or non-cryptographic random generators when cryptographic security is not required.

For security-sensitive operations, I use an appropriate cryptographically secure source.

I do not use statistical randomness for authentication, secret generation, or other security-sensitive operations.

---

## 34. Data Minimization

ACE should process only the information required for the requested analysis.

I avoid unnecessary retention of:

```text
Raw datasets
Intermediate records
Metadata
Temporary files
Logs
Generated artifacts
```

Intermediate data should be released when it is no longer required.

---

## 35. Privacy

ACE's security controls do not automatically make a dataset anonymous or compliant with privacy regulations.

Users remain responsible for determining:

```text
Whether they are permitted to process the dataset
What information may be retained
Where the dataset may be stored
Who may access generated reports
Whether outputs may be shared
```

ACE provides processing and security controls; it does not replace an organization's privacy or compliance obligations.

---

## 36. Security Testing

Security-sensitive components receive targeted tests.

I test:

```text
Malformed input
Oversized input
Invalid configuration
Path handling
Resource limits
Output escaping
Deserialization
Network validation
Permission failures
Dependency behavior
```

Where appropriate, I also use:

```text
Fuzz testing
Property testing
Regression testing
Static analysis
Dependency auditing
```

---

## 37. Fuzz Testing

I use fuzz testing for parsers and other components that process complex external input when the additional testing provides meaningful coverage.

Targets may include:

```text
CSV parsing
JSON parsing
Configuration parsing
Dataset validation
Output serialization
Protocol parsing
```

A discovered parser failure should become a regression test when practical.

---

## 38. Security Regression Tests

When I fix a security vulnerability, I add a regression test whenever practical.

The test should demonstrate that:

```text
Vulnerable behavior
       ↓
No longer occurs
```

and that the intended behavior remains functional.

---

## 39. Vulnerability Assessment

When a vulnerability is reported, I determine:

```text
Affected component
Affected versions
Attack requirements
Exploitability
Confidentiality impact
Integrity impact
Availability impact
Required privileges
User interaction
```

I use this information to determine the appropriate severity and remediation priority.

---

## 40. Security Fix Process

For a confirmed vulnerability, I follow:

```text
Security Report
      │
      ▼
Verification
      │
      ▼
Impact Assessment
      │
      ▼
Fix Development
      │
      ▼
Regression Test
      │
      ▼
Security Review
      │
      ▼
Release
      │
      ▼
Security Disclosure
```

I prioritize vulnerabilities according to their actual impact on ACE users.

---

## 41. Security Releases

When a security issue requires a release, I document the affected versions and fixed version in the changelog.

Security-related changes are recorded under:

```text
CHANGELOG.md
```

where appropriate.

---

## 42. Responsible Disclosure

I appreciate responsible security research.

For a valid privately reported vulnerability, I aim to:

```text
Acknowledge the report
Verify the vulnerability
Assess its impact
Develop a fix
Add regression coverage
Release the fix
Document the resolution
```

I ask researchers to allow reasonable time for investigation and remediation before publicly disclosing an unpatched vulnerability.

---

## 43. Security Documentation

I update the relevant documentation when security-sensitive behavior changes.

Examples:

```text
Security vulnerability
→ SECURITY.md
→ CHANGELOG.md

Input validation
→ docs/input-formats.md

Resource limits
→ docs/configuration.md

Algorithmic resource usage
→ docs/algorithms.md

Architecture security boundary
→ docs/architecture.md
```

---

## 44. Security Review Checklist

Before considering a security-sensitive feature complete, I verify:

```text
[ ] External input is validated
[ ] Invalid input produces controlled errors
[ ] Resource limits are enforced where required
[ ] Filesystem operations are validated
[ ] Sensitive information is not unnecessarily logged
[ ] Secrets are not stored in source code
[ ] Dependencies have been reviewed
[ ] Unsafe code is minimized
[ ] FFI boundaries are validated
[ ] Security-sensitive outputs are escaped
[ ] Relevant tests exist
[ ] Regression tests exist for security fixes
[ ] Documentation is updated
```

---

## 45. Final Security Standard

I build ACE around the following principles:

```text
Treat external input as untrusted.
Validate before processing.
Bound expensive operations.
Fail safely.
Minimize sensitive-data exposure.
Protect filesystem operations.
Keep secrets out of source code.
Minimize unsafe code.
Review dependencies.
Test security-sensitive behavior.
Document security assumptions.
Patch vulnerabilities responsibly.
```

Security is part of ACE's implementation and review process, not an optional feature added after the system is complete.

