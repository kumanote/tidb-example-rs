use actix_web::http::{header, StatusCode};
use actix_web::{HttpResponse, ResponseError};
use anyhow::anyhow;
use serde_json::{json, Value as JsonValue};
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("BadRequest: {reasons:?}")]
    BadRequest { reasons: Vec<JsonValue> },

    #[error("InternalServerError: {cause}")]
    InternalServerError { cause: anyhow::Error },
}

impl Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest { reasons: _ } => StatusCode::BAD_REQUEST,
            Self::InternalServerError { cause: _ } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn body(&self) -> String {
        let json = match self {
            Self::BadRequest { reasons } => {
                json!({
                    "error": {
                        "reasons": reasons,
                    }
                })
            }
            Self::InternalServerError { cause: _ } => {
                json!({
                    "error": {
                        "detail": "oops...unknown error occurred...",
                    }
                })
            }
        };
        format!("{}", json)
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header((
                header::CONTENT_TYPE,
                header::HeaderValue::from_static("application/json; charset=utf-8"),
            ))
            .body(self.body())
    }
}

impl From<store::Error> for Error {
    fn from(cause: store::Error) -> Self {
        Self::InternalServerError {
            cause: cause.into(),
        }
    }
}

impl From<store::DieselError> for Error {
    fn from(cause: store::DieselError) -> Self {
        Self::InternalServerError {
            cause: anyhow!(cause),
        }
    }
}

impl From<store::R2D2Error> for Error {
    fn from(cause: store::R2D2Error) -> Self {
        Self::InternalServerError {
            cause: anyhow!(cause),
        }
    }
}

impl From<domain::Error> for Error {
    fn from(cause: domain::Error) -> Self {
        match cause {
            domain::Error::ValidationError { reasons } => Self::BadRequest {
                reasons: reasons.into_iter().map(|d| d.detail).collect(),
            },
            domain::Error::SystemError { cause } => Self::InternalServerError { cause },
        }
    }
}
