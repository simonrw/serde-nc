extern crate serde;

mod error;
mod ser;

pub use error::{Error, Result};
pub use ser::{to_string, Serializer};
