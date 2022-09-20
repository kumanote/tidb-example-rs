use anyhow::anyhow;
use serde_json::{json, Value as JsonValue};
use thiserror::Error as ThisError;

#[derive(Debug)]
pub struct ValidationErrorDetail {
    pub detail: JsonValue,
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("ValidationError: {reasons:?}")]
    ValidationError { reasons: Vec<ValidationErrorDetail> },

    #[error("SystemError: {cause}")]
    SystemError { cause: anyhow::Error },
}

impl Error {
    pub fn new_validation_error<S: core::fmt::Display>(reason: S) -> Self {
        Self::ValidationError {
            reasons: vec![ValidationErrorDetail {
                detail: json!(reason.to_string()),
            }],
        }
    }
}

impl From<store::Error> for Error {
    fn from(cause: store::Error) -> Self {
        Self::SystemError {
            cause: cause.into(),
        }
    }
}

impl From<store::DieselError> for Error {
    fn from(cause: store::DieselError) -> Self {
        Self::SystemError {
            cause: anyhow!(cause),
        }
    }
}
