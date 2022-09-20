pub mod examples;
pub mod use_case;

mod error;

pub use error::Error;
pub type Result<T> = core::result::Result<T, Error>;
