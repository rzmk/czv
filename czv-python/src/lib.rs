use ::czv::CzvError as OGError;
use pyo3::prelude::*;

// Error-handling helpers
#[derive(thiserror::Error, Debug)]
#[error("{0}")]
pub struct CzvError(anyhow::Error);

impl From<OGError> for CzvError {
    fn from(value: OGError) -> Self {
        Self(anyhow::anyhow!(value))
    }
}

impl From<pyo3::PyErr> for CzvError {
    fn from(value: pyo3::PyErr) -> Self {
        Self(anyhow::anyhow!(value))
    }
}

impl From<csv::Error> for CzvError {
    fn from(value: csv::Error) -> Self {
        Self(anyhow::anyhow!(value))
    }
}

impl From<CzvError> for pyo3::PyErr {
    fn from(value: CzvError) -> Self {
        anyhow::anyhow!(value).into()
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

#[pymodule]
fn czv(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(count::row_count, m)?)?;
    m.add_function(wrap_pyfunction!(count::column_count, m)?)?;
    Ok(())
}
