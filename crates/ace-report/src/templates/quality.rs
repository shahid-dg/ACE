//! Templates for quality analysis reports.

/// Render a quality analysis section.
pub fn render_quality_analysis(item_id: &str, confidence: f64, annotation_count: usize) -> String {
    format!(
        r#"<section class="quality-analysis">
  <h3>Quality Analysis: {}</h3>
  <dl>
    <dt>Confidence</dt>
    <dd>{:.2}%</dd>
    <dt>Annotations</dt>
    <dd>{}</dd>
  </dl>
</section>"#,
        item_id,
        confidence * 100.0,
        annotation_count
    )
}
