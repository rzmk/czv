use crate::Result;
use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

/// Options for `rowCount`.
#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct RowCountOptions {
    /// CSV file data.
    pub file_data: String,

    #[tsify(optional)]
    /// Specify whether to include the header row (first row) in the row count.
    /// Default is false.
    pub include_header_row: Option<bool>,
}

/// Returns a count of the total number of rows.
#[wasm_bindgen(skip_jsdoc, js_name = rowCount)]
pub fn row_count(options: RowCountOptions) -> Result<usize> {
    let mut rdr = ReaderBuilder::new();

    rdr.has_headers(!options.include_header_row.unwrap_or(false));

    Ok(rdr
        .from_reader(options.file_data.as_bytes())
        .records()
        .count())
}

/// Options for `columnCount`.
#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ColumnCountOptions {
    /// CSV file data.
    pub file_data: String,
}

/// Returns a count of the total number of columns (fields).
#[wasm_bindgen(skip_jsdoc, js_name = columnCount)]
pub fn column_count(options: ColumnCountOptions) -> Result<usize> {
    let rdr = ReaderBuilder::new();

    Ok(rdr
        .from_reader(options.file_data.as_bytes())
        .headers()?
        .len())
}
