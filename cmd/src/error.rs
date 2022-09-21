use anyhow::anyhow;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("SystemError: {cause}")]
    SystemError { cause: anyhow::Error },
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

impl From<domain::Error> for Error {
    fn from(cause: domain::Error) -> Self {
        Self::SystemError {
            cause: cause.into(),
        }
    }
}
