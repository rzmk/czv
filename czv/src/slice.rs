use crate::{bail, Result};
use csv::ReaderBuilder;
use std::path::PathBuf;
use typed_builder::TypedBuilder;

/// Returns a slice of rows from the CSV data.
///
/// See [`czv::slice::Slice`](crate::slice::Slice) for a builder version (recommended) of this function.
///
/// ## Example
///
/// We have a file `fruits.csv` at the path `tests/resources/fruits.csv`:
///
/// ```csv
/// fruit,price
/// apple,2.50
/// banana,3.00
/// strawberry,1.50
/// ```
///
/// Let's print the first two non-header rows in the CSV file:
///
/// ```rust
/// use czv::{
///     Result,
///     slice::Slice
/// };
///
/// fn main() -> Result<()> {
///     let file_path = "tests/resources/fruits.csv";
///
///     let output: String = Slice::new()
///         .file_path(file_path)
///         .start(0)
///         .end(2) // exclusive
///         .include_header_row(false)
///         .execute()?;
///
///     println!("{output}");
///
///     Ok(())
/// }
/// ```
///
/// Printed to the terminal we get:
///
/// ```console
/// apple,2.50
/// banana,3.00
/// ```
///
/// ## Notes
///
/// - You may not use `file_path` and `file_data` together, only one may be specified.
/// - You may not use `end` and `length` together, only one may be specified.
/// - You may not use `index` with any of `start`, `end`, or `length`.
///
/// ## Arguments
///
/// * `file_path` - CSV file path (alternative to `file_data`).
/// * `file_data` - CSV file data (alternative to `file_path`).
/// * `start` - The index of the record to slice from (0-indexed).
/// If negative, starts from the last record.
/// * `end` - The index of the record to slice to.
/// * `length` - The length of the slice (alternative to `end`).
/// * `index` - Slice a single record. If negative, starts from the last record.
/// * `include_header_row` - Specify whether to include the header row (first row) in the records.
pub fn slice(
    file_path: Option<PathBuf>,
    file_data: Option<String>,
    start: Option<i32>,
    end: Option<i32>,
    length: Option<i32>,
    index: Option<i32>,
    include_header_row: bool,
) -> Result<String> {
    match (file_path, file_data) {
        (Some(_), Some(_)) => bail!("Cannot have both file_path and file_data, specify one only."),
        (None, None) => bail!("Must provide either file_path or file_data."),
        (Some(file_path), None) => {
            let mut rdr = ReaderBuilder::new()
                .has_headers(include_header_row)
                .from_path(file_path)?;
            if !include_header_row {
                rdr.byte_headers()?;
            }
            let original_pos = rdr.position().to_owned();
            let records_count = rdr.records().count();

            if index.is_some() && (start.is_some() || end.is_some() || length.is_some()) {
                bail!("Cannot use index with start, end, or length.")
            }
            if end.is_some() && length.is_some() {
                bail!("Cannot use end with length.")
            }

            let start_line = match start {
                Some(x) => {
                    if x > records_count as i32 {
                        bail!("start value {x} cannot be greater than the number of records.")
                    } else if x >= 0 {
                        Some(x as usize)
                    } else {
                        Some((records_count as i32 + x) as usize)
                    }
                }
                None => None,
            };
            let end_line = match end {
                Some(x) => {
                    if x >= records_count as i32 {
                        bail!("end value {x} cannot be greater than or equal to the number of records.")
                    } else if x >= 0 {
                        Some(x as usize)
                    } else {
                        Some((records_count as i32 + x) as usize)
                    }
                }
                None => None,
            };

            rdr.seek(original_pos)?;
            let mut res_vec: Vec<String> = vec![];

            if let Some(idx) = index {
                return Ok(rdr
                    .records()
                    .skip(idx as usize)
                    .next()
                    .unwrap()?
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(","));
            }

            for r in rdr
                .records()
                .skip(start_line.unwrap())
                .take(end_line.unwrap() - start_line.unwrap())
            {
                let record = r?;
                res_vec.push(
                    record
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                );
            }
            Ok(res_vec
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join("\n"))
        }
        (None, Some(file_data)) => {
            let mut rdr = ReaderBuilder::new()
                .has_headers(include_header_row)
                .from_reader(std::io::Cursor::new(file_data.as_str()));
            if !include_header_row {
                rdr.byte_headers()?;
            }
            let original_pos = rdr.position().to_owned();
            let records_count = rdr.records().count();

            if index.is_some() && (start.is_some() || end.is_some() || length.is_some()) {
                bail!("Cannot use index with start, end, or length.")
            }
            if end.is_some() && length.is_some() {
                bail!("Cannot use end with length.")
            }

            let start_line = match start {
                Some(x) => {
                    if x > records_count as i32 {
                        bail!("start value {x} cannot be greater than the number of records.")
                    } else if x >= 0 {
                        Some(x as usize)
                    } else {
                        Some((records_count as i32 + x) as usize)
                    }
                }
                None => None,
            };
            let end_line = match end {
                Some(x) => {
                    if x >= records_count as i32 {
                        bail!("end value {x} cannot be greater than or equal to the number of records.")
                    } else if x >= 0 {
                        Some(x as usize)
                    } else {
                        Some((records_count as i32 + x) as usize)
                    }
                }
                None => None,
            };

            rdr.seek(original_pos)?;
            let mut res_vec: Vec<String> = vec![];
            for r in rdr
                .records()
                .skip(start_line.unwrap())
                .take(end_line.unwrap() - start_line.unwrap())
            {
                let record = r?;
                res_vec.push(
                    record
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                );
            }
            Ok(res_vec
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join("\n"))
        }
    }
}

#[derive(TypedBuilder)]
#[builder(builder_method(name=new), build_method(vis="", name=__build))]
pub struct Slice {
    #[builder(default, setter(strip_option, into))]
    file_path: Option<PathBuf>,

    #[builder(default, setter(strip_option, into))]
    file_data: Option<String>,

    #[builder(default, setter(strip_option))]
    start: Option<i32>,

    #[builder(default, setter(strip_option))]
    end: Option<i32>,

    #[builder(default, setter(strip_option))]
    length: Option<i32>,

    #[builder(default, setter(strip_option))]
    index: Option<i32>,

    #[builder(default = false)]
    include_header_row: bool,
}

#[allow(non_camel_case_types)]
impl<
        __include_header_row: typed_builder::Optional<bool>,
        __index: typed_builder::Optional<Option<i32>>,
        __length: typed_builder::Optional<Option<i32>>,
        __end: typed_builder::Optional<Option<i32>>,
        __start: typed_builder::Optional<Option<i32>>,
        __file_data: typed_builder::Optional<Option<String>>,
        __file_path: typed_builder::Optional<Option<PathBuf>>,
    >
    SliceBuilder<(
        __file_path,
        __file_data,
        __start,
        __end,
        __length,
        __index,
        __include_header_row,
    )>
{
    pub fn execute(self) -> Result<String> {
        let builder = self.__build();
        slice(
            builder.file_path,
            builder.file_data,
            builder.start,
            builder.end,
            builder.length,
            builder.index,
            builder.include_header_row,
        )
    }
}
