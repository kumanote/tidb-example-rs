use diesel::{result::Error as DieselError, ConnectionError};
use r2d2::Error as R2D2Error;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("R2D2Error: {cause:?}")]
    R2D2Error { cause: R2D2Error },
    #[error("ConnectionError: {cause:?}")]
    ConnectionError { cause: ConnectionError },
    #[error("DieselError: {cause:?}")]
    DieselError { cause: DieselError },
}

impl From<R2D2Error> for Error {
    fn from(cause: R2D2Error) -> Self {
        Self::R2D2Error { cause }
    }
}

impl From<ConnectionError> for Error {
    fn from(cause: ConnectionError) -> Self {
        Self::ConnectionError { cause }
    }
}

impl From<DieselError> for Error {
    fn from(cause: DieselError) -> Self {
        Self::DieselError { cause }
    }
}
