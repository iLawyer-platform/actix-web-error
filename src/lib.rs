//! Error responses for actix-web made easy.
//! <!-- We don't depend on `actix_web` -->
//! This crate will make it easy implementing [`actix_web::ResponseError`](https://docs.rs/actix-web/latest/actix_web/trait.ResponseError.html) for errors.
//! It's best used in combination with [thiserror](https://docs.rs/thiserror).
//!
//! # Error Responses
//!
//! * [`Json`] will respond with JSON in the form of `{ "error": <`[`Display`](std::fmt::Display)` representation> }` (`application/json`).
//! * [`Text`] will respond with the [`Display`](std::fmt::Display) representation of the error (`text/plain`).
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
//!
//! #[derive(Debug, thiserror::Error, actix_web_error::Text)]
//! #[error("Item not found")]
//! #[status(404)]
//! struct MyOtherError;
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

    pub struct JsonErrorSerialize<'a, T> {
        pub error: &'a T,
        pub error_code: Option<&'a str>,
    }

    impl<'a, T> Serialize for JsonErrorSerialize<'a, T>
    where
        T: Display,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self.error_code {
                Some(error_code) => {
                    let mut ser = serializer.serialize_struct("_", 2)?;
                    ser.serialize_field("error", &self.error.to_string())?;
                    ser.serialize_field("error_code", &error_code.to_string())?;
                    ser.end()
                }
                None => {
                    let mut ser = serializer.serialize_struct("_", 2)?;
                    ser.serialize_field("error", &self.error.to_string())?;
                    ser.end()
                }
            }
        }
    }
}
