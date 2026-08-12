//! Weighted voting consensus strategy.

/// Compute weighted consensus using annotator reliability scores.
pub fn weighted_consensus(votes: &[(String, f64)]) -> Option<String> {
    if votes.is_empty() {
        return None;
    }

    let mut weighted_scores: std::collections::HashMap<String, f64> =
        std::collections::HashMap::new();

    for (label, weight) in votes {
        *weighted_scores.entry(label.clone()).or_insert(0.0) += weight;
    }

    weighted_scores
        .into_iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(label, _)| label)
}
