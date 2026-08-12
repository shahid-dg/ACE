#
````markdown
# ACE Algorithms

## 1. Purpose

ACE uses statistical and probabilistic methods to evaluate annotation quality, estimate consensus labels, and identify annotations that require human review.

This document defines:

- algorithms implemented by ACE
- mathematical definitions
- required inputs
- outputs
- assumptions
- edge cases
- computational complexity
- numerical considerations
- planned optimization strategies

The implementation should prioritize **correctness, reproducibility, and transparent behavior** before optimization.

---

# 2. Algorithm Pipeline

ACE separates analysis into three major stages:

```text
                    Dataset
                       │
                       ▼
              ┌─────────────────┐
              │ Agreement       │
              │ Analysis        │
              └────────┬────────┘
                       │
                       ▼
              ┌─────────────────┐
              │ Consensus       │
              │ Estimation      │
              └────────┬────────┘
                       │
                       ▼
              ┌─────────────────┐
              │ Quality         │
              │ Analysis        │
              └────────┬────────┘
                       │
                       ▼
              ┌─────────────────┐
              │ Review          │
              │ Prioritization  │
              └─────────────────┘
````

Agreement measures how annotators behave relative to each other.

Consensus estimates the most likely label.

Quality analysis combines these signals to identify potentially problematic data.

---

# 3. Notation

The algorithms use the following notation.

| Symbol   | Meaning                                              |
| -------- | ---------------------------------------------------- |
| (N)      | Number of items                                      |
| (M)      | Number of annotators                                 |
| (K)      | Number of possible labels                            |
| (x_i)    | Item (i)                                             |
| (a_j)    | Annotator (j)                                        |
| (y_{ij}) | Label assigned by annotator (j) to item (i)          |
| (c)      | A possible label                                     |
| (n_{ic}) | Number of annotators assigning label (c) to item (i) |
| (p_{ic}) | Estimated probability that item (i) has label (c)    |

Missing annotations are represented as missing observations rather than as a special label.

---

# 4. Pairwise Agreement

## 4.1 Definition

Pairwise agreement measures the proportion of comparable annotations on which two annotators assign the same label.

For annotators (a) and (b):

[
A_{ab} =
\frac{
\sum_{i \in I_{ab}}
\mathbf{1}(y_{ia}=y_{ib})
}{
|I_{ab}|
}
]

where:

* (I_{ab}) is the set of items annotated by both annotators
* (\mathbf{1}) is the indicator function

## 4.2 Example

```text
Item      A       B
--------------------
001       cat     cat
002       dog     dog
003       cat     dog
004       bird    bird
```

Three of four annotations agree:

[
A = \frac{3}{4}=0.75
]

## 4.3 Output

ACE should expose:

```text
agreement_score
comparable_items
matching_items
```

## 4.4 Edge Cases

If two annotators have no overlapping items:

```text
comparable_items = 0
```

The result must not silently become `0.0`.

The implementation should return an explicit undefined/insufficient-data result.

## 4.5 Complexity

For (M) annotators and (N) items:

A naive pairwise implementation has approximately:

[
O(M^2N)
]

complexity.

Sparse datasets should avoid constructing unnecessary dense matrices.

---

# 5. Agreement Matrix

ACE can represent pairwise agreement across all annotators as:

```text
             A       B       C
        ┌────────────────────────
A       │ 1.00    0.82    0.91
B       │ 0.82    1.00    0.76
C       │ 0.91    0.76    1.00
```

The matrix is symmetric:

[
A_{ab}=A_{ba}
]

and:

[
A_{aa}=1
]

for annotators with sufficient observations.

The implementation should avoid recalculating both (A_{ab}) and (A_{ba}).

---

# 6. Cohen's Kappa

## 6.1 Purpose

Cohen's kappa measures agreement between two annotators while correcting for agreement expected by chance.

[
\kappa =
\frac{P_o-P_e}
{1-P_e}
]

where:

* (P_o) = observed agreement
* (P_e) = expected agreement

---

## 6.2 Observed Agreement

[
P_o =
\frac{\text{matching annotations}}
{\text{comparable annotations}}
]

---

## 6.3 Expected Agreement

For each label (c):

[
P_e =
\sum_{c=1}^{K}
P_a(c)P_b(c)
]

where:

* (P_a(c)) is annotator A's frequency for label (c)
* (P_b(c)) is annotator B's frequency for label (c)

---

## 6.4 Interpretation

ACE should not automatically classify kappa using arbitrary labels such as "good" or "bad."

The raw statistic should be preserved.

Interpretation depends on:

* label prevalence
* dataset size
* task characteristics
* annotation policy

---

## 6.5 Degenerate Cases

If:

[
1-P_e=0
]

the statistic is mathematically undefined.

The implementation must detect this condition instead of producing:

```text
NaN
```

or:

```text
infinity
```

without explanation.

---

## 6.6 Complexity

For two annotators:

[
O(N+K)
]

assuming label frequencies can be accumulated in a single pass.

---

# 7. Fleiss' Kappa

## 7.1 Purpose

Fleiss' kappa generalizes chance-corrected agreement to multiple annotators.

For item (i):

[
P_i =
\frac{1}{n_i(n_i-1)}
\sum_{c=1}^{K}
n_{ic}(n_{ic}-1)
]

where:

* (n_i) = number of annotations for item (i)
* (n_{ic}) = number of annotations assigning label (c)

Average observed agreement:

[
\bar P =
\frac{1}{N}
\sum_{i=1}^{N}P_i
]

Overall category proportions:

[
p_c =
\frac{
\sum_i n_{ic}
}{
\sum_i n_i
}
]

Expected agreement:

[
P_e =
\sum_{c=1}^{K}p_c^2
]

Finally:

[
\kappa =
\frac{\bar P-P_e}
{1-P_e}
]

---

## 7.2 Requirements

Fleiss' kappa is intended for multiple annotators assigning categorical labels.

The implementation must account for incomplete annotation coverage.

The exact missing-data policy must be explicit in configuration.

---

## 7.3 Complexity

With (N) items and (K) labels:

[
O(NK)
]

if per-item label counts are available.

A sparse representation may reduce practical cost when (K) is large.

---

# 8. Krippendorff's Alpha

## 8.1 Purpose

Krippendorff's alpha measures agreement while supporting:

* multiple annotators
* incomplete annotation
* different measurement levels

ACE initially targets nominal categorical data.

---

## 8.2 General Definition

[
\alpha =
1-\frac{D_o}{D_e}
]

where:

* (D_o) = observed disagreement
* (D_e) = disagreement expected by chance

For nominal labels, disagreement is:

[
\delta(c,c') =
\begin{cases}
0 & c=c' \
1 & c\neq c'
\end{cases}
]

---

## 8.3 Implementation Strategy

The initial implementation should:

1. construct coincidence counts
2. calculate observed disagreement
3. calculate expected disagreement
4. calculate alpha
5. validate degenerate cases

Additional measurement levels can be introduced later without changing the public analysis architecture.

---

## 8.4 Edge Cases

The implementation must explicitly handle:

* empty datasets
* one observed category
* insufficient observations
* zero expected disagreement

---

# 9. Majority Vote

## 9.1 Purpose

Majority voting provides the simplest consensus baseline.

For item (i):

[
\hat y_i =
\arg\max_c n_{ic}
]

Example:

```text
cat   = 7
dog   = 2
bird  = 1
```

Consensus:

```text
cat
```

---

## 9.2 Ties

A tie must be deterministic.

For example:

```text
cat = 2
dog = 2
```

ACE must not rely on hash-map iteration order to determine the result.

Possible policies include:

* return multiple tied labels
* use a deterministic configured tie-breaker
* return an explicit unresolved consensus

The selected policy must be documented in configuration.

---

## 9.3 Complexity

For (N) items and (M) annotations:

[
O(M)
]

when each annotation is processed once.

Memory complexity depends on whether per-item counts are retained.

---

# 10. Weighted Voting

Weighted voting assigns different influence to annotators.

For annotator (a), define weight:

[
w_a \geq 0
]

For label (c):

[
S_{ic} =
\sum_{a \in A_i}
w_a\mathbf{1}(y_{ia}=c)
]

The selected consensus is:

[
\hat y_i =
\arg\max_c S_{ic}
]

---

## 10.1 Weight Sources

Weights may eventually be derived from:

* historical agreement
* reliability estimates
* manually assigned confidence
* model-derived reliability

Weights must not be derived from the same target item in a way that introduces circularity.

---

## 10.2 Example

```text
Annotator     Weight    Label
-----------------------------
A             0.95      cat
B             0.80      cat
C             0.40      dog
```

Scores:

```text
cat = 1.75
dog = 0.40
```

Consensus:

```text
cat
```

---

# 11. Dawid-Skene

## 11.1 Purpose

Dawid-Skene estimates:

* latent true labels
* annotator confusion behavior
* probability of each possible true label

It is more expressive than majority voting because annotators are not assumed to have identical reliability.

---

## 11.2 Model

For annotator (j), true label (c), and observed label (k):

[
\theta_{jck}
============

P(y_{ij}=k \mid z_i=c)
]

where:

* (z_i) is the latent true label
* (\theta_{jck}) represents annotator (j)'s confusion probability

The model also estimates class priors:

[
\pi_c=P(z_i=c)
]

---

# 12. Dawid-Skene Initialization

The implementation should provide a deterministic initialization strategy.

A simple baseline is majority voting:

```text
Annotations
     │
     ▼
Majority Vote
     │
     ▼
Initial latent labels
     │
     ▼
Initialize annotator confusion matrices
```

This generally provides a more useful starting point than completely random initialization.

---

# 13. Dawid-Skene E-Step

Given current model parameters, calculate:

[
P(z_i=c \mid y_i)
]

using Bayes' rule.

Conceptually:

[
P(z_i=c\mid y_i)
\propto
\pi_c
\prod_j
\theta_{j,c,y_{ij}}
]

The implementation should use logarithmic probabilities to reduce numerical underflow for large numbers of annotations.

---

# 14. Dawid-Skene M-Step

Update the class priors:

[
\pi_c =
\frac{1}{N}
\sum_i P(z_i=c\mid y_i)
]

Update annotator confusion probabilities using expected counts:

[
\theta_{jck}
============

\frac{
\sum_i
P(z_i=c\mid y_i)
\mathbf{1}(y_{ij}=k)
}{
\sum_i P(z_i=c\mid y_i)
}
]

---

# 15. Dawid-Skene Convergence

The algorithm repeats:

```text
Initialize
    │
    ▼
E-Step
    │
    ▼
M-Step
    │
    ▼
Convergence?
   / \
 No   Yes
 │     │
 └─────┘
       ▼
   Final Result
```

Convergence should be controlled by configuration.

Potential parameters:

```text
max_iterations
tolerance
minimum_probability
```

The implementation must also stop after `max_iterations` even if convergence is not reached.

---

# 16. Numerical Stability

Dawid-Skene calculations involve products of probabilities.

Direct multiplication can underflow:

[
0.001^{1000}\rightarrow0
]

Therefore the implementation should operate in log-space where appropriate.

For summing log probabilities:

[
\log(e^a+e^b)
]

should use a numerically stable log-sum-exp implementation.

---

# 17. Consensus Confidence

Consensus confidence represents the strength of evidence supporting the selected label.

For probabilistic consensus:

[
confidence_i =
\max_c P(z_i=c\mid y_i)
]

Example:

```text
cat  = 0.91
dog  = 0.07
bird = 0.02

consensus  = cat
confidence = 0.91
```

Confidence is a model output.

It must not be presented as a guaranteed probability that the label is objectively correct.

---

# 18. Annotator Reliability

Annotator reliability estimates how consistently an annotator behaves relative to the consensus or model.

A basic reliability measure can be defined as:

[
R_a =
\frac{
\text{annotations agreeing with reference}
}{
\text{comparable annotations}
}
]

The reference may be:

* majority consensus
* probabilistic consensus
* withheld expert labels
* configured ground truth

The source of the reference must always be explicit.

---

# 19. Disagreement Score

An item-level disagreement score can be derived from the label distribution.

For simple categorical voting:

[
D_i =
1-\frac{\max_c n_{ic}}{n_i}
]

Example:

```text
5 annotators

cat = 5

D = 1 - 5/5
D = 0
```

High agreement therefore produces a low disagreement score.

Another item:

```text
cat = 3
dog = 2

D = 1 - 3/5
D = 0.4
```

This is intentionally simple and should serve as a transparent baseline.

---

# 20. Entropy-Based Disagreement

For datasets where a more sensitive measure is useful, ACE can calculate label entropy.

[
H_i =
-\sum_c p_{ic}\log p_{ic}
]

where:

[
p_{ic}=\frac{n_{ic}}{n_i}
]

Maximum entropy occurs when labels are distributed evenly.

Entropy can therefore identify ambiguous items that simple majority margins may underestimate.

---

# 21. Normalized Entropy

For (K) possible labels:

[
H_i^{norm} =
\frac{H_i}{\log K}
]

This produces a value between approximately:

```text
0 = concentrated distribution
1 = maximally distributed distribution
```

The normalization is only meaningful when the label universe is well-defined.

---

# 22. Outlier Detection

ACE should not define an outlier purely as "an annotator with low agreement."

Outlier detection should consider multiple signals.

Potential features include:

```text
agreement score
label distribution
consensus disagreement
class-specific disagreement
annotation volume
reliability
```

A future composite anomaly score can be represented as:

[
S_a =
\sum_k w_k f_k(a)
]

where:

* (f_k) is a normalized feature
* (w_k) is its configured weight

The initial implementation should keep individual signals available rather than hiding everything behind one opaque score.

---

# 23. Review Prioritization

ACE's ultimate quality objective is not simply to produce statistics.

It is to identify which data should be reviewed first.

A review priority score can combine:

```text
item disagreement
+
consensus uncertainty
+
annotator reliability
+
anomaly signals
```

Conceptually:

[
Priority_i =
w_dD_i+
w_cC_i+
w_aA_i
]

where:

* (D_i) = disagreement
* (C_i) = uncertainty
* (A_i) = anomaly signal

The exact scoring model must remain configurable and documented.

---

# 24. Ground Truth

ACE must distinguish between:

```text
Consensus
```

and:

```text
Ground Truth
```

Consensus is an estimate derived from annotations.

Ground truth is an externally established reference.

A high-consensus label is not automatically correct.

For supervised evaluation, ACE should support explicit ground-truth datasets rather than treating consensus as ground truth.

---

# 25. Missing Annotations

Missing annotations must not be interpreted as incorrect labels.

For example:

```text
Item 001

Annotator A → cat
Annotator B → cat
Annotator C → missing
```

The comparison is based on the available observations.

The missing observation must not become:

```text
C → unknown
```

unless the user explicitly defines `unknown` as a legitimate label.

---

# 26. Duplicate Annotations

Duplicate records must be handled explicitly.

Example:

```text
item_001,worker_01,cat
item_001,worker_01,cat
```

The system must determine whether these represent:

* duplicate export records
* repeated annotations
* revisions
* legitimate multiple observations

ACE should not silently count duplicates as independent annotations.

---

# 27. Empty Dataset

An empty dataset must produce a valid, explicit result or an informative error.

Statistical functions must not perform operations such as:

```text
0 / 0
```

and return an unexplained `NaN`.

---

# 28. Single Annotator

Agreement metrics requiring multiple annotators cannot be meaningfully calculated with one annotator.

For example:

```text
Cohen's kappa
Fleiss' kappa
pairwise agreement
```

should return an insufficient-data result rather than fabricating agreement.

Consensus methods such as majority vote can still operate.

---

# 29. Single Label

If the entire dataset contains only one possible label, some chance-corrected metrics become degenerate.

The implementation must detect this condition and report it explicitly.

---

# 30. Algorithm Complexity

Expected baseline complexity:

| Algorithm               | Time Complexity |
| ----------------------- | --------------: |
| Majority vote           |          (O(A)) |
| Pairwise agreement      |       (O(M^2N)) |
| Cohen's kappa           |        (O(N+K)) |
| Fleiss' kappa           |         (O(NK)) |
| Nominal alpha           |         (O(NK)) |
| Weighted vote           |          (O(A)) |
| Dawid-Skene / iteration |        (O(NAK)) |
| Entropy                 |         (O(NK)) |

Where:

* (N) = items
* (M) = annotators
* (A) = annotations
* (K) = labels

Actual complexity depends on dataset sparsity and internal representation.

---

# 31. Algorithm Selection

ACE should expose algorithms independently rather than forcing one method.

Example workflow:

```text
Small dataset
    │
    ├── Majority Vote
    ├── Cohen's Kappa
    └── Manual review

Large dataset
    │
    ├── Agreement Matrix
    ├── Dawid-Skene
    ├── Reliability
    ├── Entropy
    └── Review Prioritization
```

No single algorithm should be treated as universally superior.

---

# 32. Validation Strategy

Every statistical implementation must be validated against known examples.

Tests should include:

* hand-calculated examples
* known statistical reference values
* edge cases
* missing data
* class imbalance
* ties
* degenerate distributions
* large datasets

Where appropriate, ACE should compare results against independently verified reference implementations during development.

---

# 33. Determinism Requirements

Algorithms must produce deterministic results when supplied with deterministic input.

This is particularly important for:

* tie-breaking
* sorting
* aggregation
* parallel reductions
* model initialization

Hash-map iteration order must never determine a statistical result.

---

# 34. Performance Optimization

Optimization should follow this sequence:

```text
Correct implementation
        │
        ▼
Unit tests
        │
        ▼
Reference validation
        │
        ▼
Benchmark
        │
        ▼
Profile
        │
        ▼
Optimize
        │
        ▼
Benchmark again
```

Potential optimizations include:

* Rayon parallelism
* compact representations
* precomputed indexes
* reduced allocations
* cache-friendly data layouts
* SIMD for suitable numerical operations

Optimization must never change the mathematical definition of an algorithm.

---

# 35. Algorithm Module Mapping

The planned Rust organization is:

```text
ace-core/src/
├── agreement/
│   ├── mod.rs
│   ├── pairwise.rs
│   ├── cohens_kappa.rs
│   ├── fleiss_kappa.rs
│   ├── krippendorffs_alpha.rs
│   ├── confusion_matrix.rs
│   └── agreement_matrix.rs
│
├── consensus/
│   ├── mod.rs
│   ├── majority_vote.rs
│   ├── weighted_vote.rs
│   ├── dawid_skene.rs
│   └── confidence.rs
│
└── quality/
    ├── mod.rs
    ├── reliability.rs
    ├── disagreement.rs
    ├── entropy.rs
    ├── outliers.rs
    └── prioritization.rs
```

Each module should expose focused APIs rather than a single monolithic analysis function.

---

# 36. Implementation Standard

A production algorithm implementation should contain:

```text
Algorithm
├── Mathematical definition
├── Input validation
├── Core computation
├── Numerical safeguards
├── Edge-case handling
├── Unit tests
├── Reference-value tests
├── Documentation
└── Benchmark
```

No algorithm should be considered complete merely because it produces a numerical result.

---

# 37. Guiding Principle

ACE should favor algorithms that are:

* statistically defensible
* reproducible
* explainable
* independently testable
* computationally practical
* transparent about limitations

The purpose of the engine is not to produce impressive-looking scores.

The purpose is to produce **reliable evidence about annotation quality that can be acted upon by a human reviewer or downstream data pipeline**.

```