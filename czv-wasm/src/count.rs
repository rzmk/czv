use crate::Result;
use csv::ReaderBuilder;
use wasm_bindgen::prelude::*;

/// Returns a count of the total number of rows.
///
/// @param {string} `file_data` CSV file data.
/// @param {boolean | undefined} `include_header_row` Specify whether to include the header row (first row) in the row count. Default is false.
/// @returns {number}
#[wasm_bindgen(skip_jsdoc, js_name = rowCount)]
pub fn row_count(file_data: String, include_header_row: Option<bool>) -> Result<usize> {
    let mut rdr = ReaderBuilder::new();

    rdr.has_headers(!include_header_row.unwrap_or(false));
    return Ok(rdr.from_reader(file_data.as_bytes()).records().count());
}

/// Returns a count of the total number of columns (fields).
///
/// ## Arguments
///
/// @param {string} `file_data` CSV file data.
#[wasm_bindgen(skip_jsdoc, js_name = columnCount)]
pub fn column_count(file_data: Option<String>) -> Result<usize> {
    let rdr = ReaderBuilder::new();

    if let Some(file_data) = file_data {
        return Ok(rdr.from_reader(file_data.as_bytes()).headers()?.len());
    } else {
        bail!("Could not determine a file path or file data for column_count_builder.");
    }
}
