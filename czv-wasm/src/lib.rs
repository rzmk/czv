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

impl Into<JsValue> for CzvError {
    fn into(self) -> JsValue {
        JsValue::from_str(self.to_string().as_str())
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
