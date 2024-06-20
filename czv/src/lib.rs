//! # czv
//!
//! CSV operations library for data engineering/analysis tasks.
//!
//! ## Example
//!
//! ```rust
//! use czv::{count::RowCount, Result};
//!
//! fn main() -> Result<()> {
//!     let data = "\
//! fruits,price
//! apple,2.50
//! banana,3.00
//! strawberry,1.50
//! ";
//!     let output = RowCount::new().file_data(data).execute()?;
//!     println!("{output}"); // 3
//!     Ok(())
//! }
//! ```
//!
//! ## Links
//!
//! - Source code: <https://github.com/rzmk/czv>
//! - czv-wasm: <https://github.com/rzmk/czv>
//! - czv-python: <https://github.com/rzmk/czv>

/// Counting operations including row count and column count.
pub mod count;
/// Extract a section of rows.
pub mod slice;

#[allow(dead_code)]
#[derive(Debug)]
pub struct CzvError(anyhow::Error);

impl From<anyhow::Error> for CzvError {
    fn from(value: anyhow::Error) -> Self {
        value.into()
    }
}

impl From<csv::Error> for CzvError {
    fn from(value: csv::Error) -> Self {
        value.into()
    }
}

pub type Result<T, E = CzvError> = anyhow::Result<T, E>;

#[macro_export]
#[allow(unused_macros)]
macro_rules! bail {
    ($err:expr $(,)?) => {
        return Err(crate::CzvError(anyhow::anyhow!($err)))
    };
}
