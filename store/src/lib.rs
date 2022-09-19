#[macro_use]
extern crate diesel;

mod conn;
mod error;
mod pool;
mod schema;

pub mod adapters;
pub mod entities;

pub use conn::*;
pub use error::*;
pub use pool::*;

pub use diesel::connection::{Connection, TransactionManager};
pub use diesel::result::Error as DieselError;

pub type StoreConnection = diesel::mysql::MysqlConnection;
pub type StorePool = r2d2::Pool<diesel::r2d2::ConnectionManager<StoreConnection>>;
pub type Result<T> = core::result::Result<T, Error>;
