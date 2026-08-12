#

````markdown
# Code of Conduct

## 1. Purpose

I want ACE to remain a professional, technically focused, and respectful open-source project.

I expect contributors to be able to:

- propose ideas
- challenge technical decisions
- review code
- report problems
- disagree about implementation
- suggest alternatives

without personal attacks, harassment, discrimination, or intimidation.

Technical disagreement is welcome. Personal hostility is not.

---

## 2. Expected Behavior

I expect everyone participating in ACE to:

- communicate respectfully
- focus criticism on code, designs, and technical decisions
- provide constructive feedback
- assume good faith
- explain technical disagreements clearly
- respect different levels of experience
- keep discussions relevant to the project
- accept decisions after reasonable technical discussion
- correct mistakes without hostility
- help maintain a productive development environment

---

## 3. Technical Disagreement

ACE is an engineering project, so disagreement is expected.

Contributors may disagree about:

```text
Architecture
Algorithms
Data structures
API design
Dependencies
Performance
Testing
Error handling
Configuration
CLI behavior
````

I encourage contributors to support technical arguments with evidence.

For example:

```text
This implementation increases memory usage on the 10M-label
benchmark and introduces a 31% regression compared with the
current implementation.
```

This is useful because it identifies a measurable technical problem.

By contrast:

```text
You don't understand Rust.
```

does not provide useful technical information and is not acceptable project communication.

---

## 4. Code Review Standards

I expect code reviews to remain professional and technically focused.

Review comments should identify concrete concerns such as:

```text
Incorrect behavior
Security problems
Performance regressions
API compatibility issues
Missing tests
Maintainability problems
Error-handling problems
Documentation gaps
```

Where practical, I explain why a requested change is necessary.

A reviewer may strongly disagree with an implementation while still communicating respectfully.

---

## 5. Acceptable Review Feedback

Useful review feedback looks like:

```text
This allocation occurs once for every annotation.
Can we reuse the existing buffer here?

The current implementation does not handle an empty
annotator set. Please add a test for that case.

This changes the public output schema and therefore
needs to be documented as a breaking change.
```

These comments are specific, actionable, and focused on the implementation.

---

## 6. Unacceptable Behavior

The following behavior is not acceptable:

* harassment
* threats
* personal attacks
* intimidation
* discrimination
* sexual harassment
* deliberate humiliation
* targeted trolling
* abusive communication
* publishing private information without permission
* deliberately disrupting project discussions
* repeatedly targeting another contributor
* hostile behavior intended to prevent someone from participating

---

## 7. Personal Information

I do not allow contributors to publish another person's private information without permission.

This includes:

```text
Private addresses
Private phone numbers
Private email addresses
Authentication information
Private account information
Other non-public personal information
```

Security-sensitive information should be reported according to:

```text
SECURITY.md
```

---

## 8. Project Discussions

I expect project discussions to remain focused on solving problems.

When disagreement occurs, contributors should prioritize:

```text
Evidence
Reproducibility
Benchmarks
Documentation
Tests
Technical reasoning
```

rather than personal arguments.

For example, when discussing performance, I prefer:

```text
Benchmark A:
Current implementation: 4.8s
Proposed implementation: 3.1s

Dataset:
10M annotations
50 annotators
```

over unsupported claims about which implementation is "better."

---

## 9. Maintainer Responsibilities

Project maintainers are responsible for maintaining a productive project environment.

Maintainers may:

* moderate discussions
* request that inappropriate behavior stop
* remove inappropriate content
* close discussions
* restrict participation
* temporarily restrict access
* permanently restrict access when necessary

Maintainers should apply these measures consistently and proportionately.

---

## 10. Reporting Problems

If someone experiences or observes behavior that violates this code of conduct, they should report it privately to the project maintainer.

A report should include, where possible:

```text
What happened
When it happened
Where it happened
Who was involved
Relevant links
Relevant screenshots or other evidence
```

Reports should not expose unnecessary private information.

---

## 11. Handling Reports

When a report is received, I aim to:

```text
1. Review the report
2. Determine whether the reported behavior violates this policy
3. Gather relevant information
4. Protect the privacy of the people involved
5. Determine an appropriate response
6. Take proportionate action
```

I do not require public discussion of personal disputes.

---

## 12. Security Reports

Security vulnerabilities should not be reported through normal public discussions.

Security reports must follow the process described in:

```text
SECURITY.md
```

This prevents potentially exploitable information from being publicly disclosed before a fix is available.

---

## 13. Conflicts of Interest

Contributors should disclose relevant conflicts of interest when they materially affect a technical decision.

Examples may include:

```text
A dependency being promoted for commercial reasons
A benchmark being performed on hardware that materially favors one implementation
A proposal directly benefiting a contributor's external product
```

Disclosure allows technical decisions to be evaluated fairly.

---

## 14. Good-Faith Participation

I encourage contributors to:

```text
Ask questions.
Challenge assumptions.
Provide evidence.
Share benchmarks.
Propose alternatives.
Admit mistakes.
Correct mistakes.
Improve implementations.
```

Nobody is expected to be correct all the time.

Changing an implementation after receiving better technical evidence is a normal part of software development.

---

## 15. Scope

This code of conduct applies to project-related participation, including:

```text
GitHub Issues
Pull Requests
GitHub Discussions
Code Reviews
Documentation
Commit discussions
Official project communication channels
Other spaces officially associated with ACE
```

---

## 16. Enforcement

Maintainers may take action when behavior violates this code of conduct.

Possible actions include:

```text
Informal warning
Request to stop the behavior
Removal of inappropriate content
Temporary restriction
Discussion closure
Temporary project access restriction
Permanent project access restriction
```

The response depends on:

```text
Severity
Frequency
Intent
Impact
Previous behavior
```

---

## 17. No Retaliation

I do not permit retaliation against someone who makes a good-faith report or participates in an investigation.

A contributor should not be targeted for:

* reporting inappropriate behavior
* reporting a security issue
* raising a legitimate technical concern
* participating in a code review
* providing evidence during an investigation

---

## 18. False Reports

Good-faith mistakes are handled differently from deliberately fabricated reports.

I encourage people to report concerns when they are uncertain about what happened.

However, deliberately fabricating allegations or manipulating evidence to harm another contributor is not acceptable.

---

## 19. Contributor Standard

I want contributors to hold themselves to this standard:

```text
Be rigorous about the code.
Be honest about evidence.
Be constructive during disagreement.
Respect other contributors.
Protect private information.
```

---

## 20. Final Principle

ACE is a technical project.

Strong disagreement, rigorous criticism, benchmarking, competing implementations, and architectural debate are all welcome.

The boundary is simple:

```text
Challenge the implementation.
Do not attack the person.
```
