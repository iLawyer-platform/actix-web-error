//! Error responses for actix-web made easy.
//! This crate will make it easy implementing [`actix_web::ResponseError`] for errors.
//! It's best used in combination with [thiserror](https://docs.rs/thiserror).
//!
//! Currently, only the [`Json`] error is supported.
//!
//! # Example
//!
//! ```
//! #[derive(Debug, thiserror::Error, actix_web_error::Json)]
//! #[status(BAD_REQUEST)] // default status for all variants
//! enum MyError {
//!     #[error("Missing: {0}")]
//!     MissingField(&'static str),
//!     #[error("Malformed Date")]
//!     MalformedDate,
//!     #[error("Internal Server Error")]
//!     #[status(500)] // specific override
//!     Internal,
//! }
//! # fn main() {}
//! ```
//!
#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]

pub use actix_web_error_derive::*;

#[doc(hidden)]
pub mod __private {
    use serde::{ser::SerializeStruct, Serialize, Serializer};
    use std::fmt::Display;

    #[repr(transparent)]
    pub struct JsonErrorSerialize<'a, T>(pub &'a T);

    impl<'a, T> Serialize for JsonErrorSerialize<'a, T>
    where
        T: Display,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut ser = serializer.serialize_struct("_", 1)?;
            ser.serialize_field("error", &self.0.to_string())?;
            ser.end()
        }
    }
}
