//! CSV reader for annotation data.

use crate::error::Result;
use serde::Deserialize;
use std::io::Read;

/// Annotation record from CSV.
#[derive(Debug, Clone, Deserialize)]
pub struct AnnotationRecord {
    pub item_id: String,
    pub annotator_id: String,
    pub label: String,
}

/// Read annotations from CSV format.
pub fn read_csv<R: Read>(reader: R) -> Result<Vec<AnnotationRecord>> {
    let mut csv_reader = csv::Reader::from_reader(reader);
    let mut records = Vec::new();

    for result in csv_reader.deserialize() {
        let record: AnnotationRecord = result?;
        records.push(record);
    }

    Ok(records)
}
