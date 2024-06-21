//! # czv
//!
//! WASM library for [czv](https://github.com/rzmk/czv). CSV operations library for data engineering/analysis tasks.
//!
//! # Example
//!
//! Let's say we want to print the total number of non-header rows in our data:
//!
//! ```js
//! import init, * as czv from "czv-wasm";
//! // Must run `await init()` or `initSync()` first for web use
//! await init();
//!
//! const data = `fruits,price
//! apple,2.50
//! banana,3.00
//! strawberry,1.50`;
//!
//! const output = czv.rowCount({
//!     file_data: data,
//!     include_header_row: true,
//! });
//!
//! console.log(output);
//! ```
//!
//! For a full website example see the example's source code here: <https://github.com/rzmk/czv/tree/main/czv-wasm/examples/web-demo>
//!
//! # Links
//!
//! - czv GitHub repository: <https://github.com/rzmk/czv>
//! - Rust: [crates.io/crates/czv](https://crates.io/crates/czv) ([source code](https://github.com/rzmk/czv/tree/main/czv))
//! - WebAssembly (JavaScript/TypeScript): [npmjs.com/package/czv-wasm](https://www.npmjs.com/package/czv-wasm) ([source code](https://github.com/rzmk/czv/tree/main/czv-wasm))
//! - Python: [pypi.org/project/czv](https://pypi.org/project/czv/) ([source code](https://github.com/rzmk/czv/tree/main/czv-python))

#![allow(
    // https://github.com/madonoharu/tsify/issues/42
    non_snake_case,
    // https://github.com/rustwasm/wasm-bindgen/issues/3945
    clippy::empty_docs
)]

use wasm_bindgen::JsValue;

// Error-handling helpers
#[derive(thiserror::Error, Debug)]
#[error("{0}")]
pub struct CzvError(anyhow::Error);

impl From<csv::Error> for CzvError {
    fn from(value: csv::Error) -> Self {
        value.into()
    }
}

impl From<serde_wasm_bindgen::Error> for CzvError {
    fn from(value: serde_wasm_bindgen::Error) -> Self {
        value.into()
    }
}

impl From<CzvError> for JsValue {
    fn from(val: CzvError) -> Self {
        JsValue::from_str(val.to_string().as_str())
    }
}

pub type Result<T> = anyhow::Result<T, CzvError>;

#[allow(unused_macros)]
macro_rules! bail {
    ($err:expr $(,)?) => {
        return Err(crate::CzvError(anyhow::anyhow!($err)))
    };
}

// Command imports
pub mod count;
