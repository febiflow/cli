//! Defines the custom error types used within the application.
//!
//! This module provides a structured way to represent different kinds of errors
//! that can occure, facilitating error handling and reporting. It includes
//! an `ErrorKind` enum to categorize errors and an `Error` struct to hold
//! specific error details, along with conversions from common error types like
//! `std::io::Error` and `serde_json::Error`.

/// Represents the specific category of an application error.
///
/// This enum is used by the `Error` struct to classify the nature of the
/// problem encountered.
#[derive(Debug, PartialEq)]
pub enum ErrorKind {
  /// Indicates that a required configuration file `ferrum.json` is missing.
  MissingFerrumConfig,

  /// An error occured during a file read operation (e.g.,
  /// permission denied, file not found).
  FileRead,

  /// An error occured while deserializing a JSON data (e.g, incorrect
  /// format).
  JsonDeserialization,

  /// A specified profile was not found, typically within the configuration data.
  ProfileNotFound,

  /// Represents an error that doesn't fit into the other specific categories.
  Unhandled,
}

/// The primary error structure of the application.
///
/// It encapsulates the `ErrorKind` variant to categorize the error and a
/// `String` message providing more specific details about the error instance.
#[derive(Debug)]
pub struct Error {
  /// The category of the error.
  kind: ErrorKind,

  /// A detailed message describing the specific error encountered.
  message: String,
}

impl Error {
  /// Creates a new `Error` instance.
  ///
  /// # Arguments
  ///
  /// * `kind` - The `ErrorKind` variant categorizing this error.
  /// * `message` - A string slice containing the detailed error message.
  ///
  /// # Returns
  ///
  /// A new `Error` struct initialized with the given kind and message.
  pub fn new(kind: ErrorKind, message: &str) -> Self {
    Self { kind, message: message.to_string() }
  }

  /// Returns a reference to the `ErrorKind` of this error.
  ///
  /// This allows checking the category without consuming it.
  pub fn kind(&self) -> &ErrorKind {
    &self.kind
  }

  /// Returns a reference to the detailed error message string.
  pub fn message(&self) -> &str {
    &self.message
  }
}

/// Enables conversion from a standard I/O error (`std::io::Error`) into the
/// application's `Error` type.
///
/// This conversion automatically assigns the `ErrorKind::FileRead` and uses the
/// origin I/O error's string representation as the message. This simplifies
/// error handling when dealing with file operations using the `?` operator.
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
      Self {
        kind: ErrorKind::FileRead,
        message: error.to_string(),
      }
    }
}

/// Enables conversion from a Serde JSON error (`serde_json::Error`) into the
/// application's `Error` type.
///
/// This conversion automatically assigns the `ErrorKind::JsonDeserialization`
///  and uses the original Serde JSON error's string representation as the
/// message. This simplifies error handling when dealing with deserializing JSON
/// data using the `?` operator.
impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
      Self {
        kind: ErrorKind::JsonDeserialization,
        message: error.to_string(),
      }
    }
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde_json::Value;
  use std::fs::File;

  /// Tests the conversion from `std::io::Error` to an `Error`.
  ///
  /// Verifies that a file-not-found error, which results in `std::io::Error`,
  /// is correctly converted into an `Error` with the `ErrorKind::FileRead`.
  #[test]
  fn test_from_file_read_error() {
    // arrange
    let file_read_error = File::open("non_existing_file")
      .err().expect("Expected file open to fail");

    // act
    let app_error = Error::from(file_read_error);

    // assert
    assert_eq!(app_error.kind(), &ErrorKind::FileRead);
    assert!(app_error.message().contains("No such file or directory"));
  }

  /// Tests the conversion from `serde_json::Error` to an `Error`.
  ///
  /// Verifies the JSON parsing error, resulting in `serde_json::Error`, is
  /// correctly converted into an `Error` with the
  /// `ErrorKind::JsonDeserialization`.
  #[test]
  fn test_from_serde_json_error() {
    // arrange
    let invalid_json_data = r#"{ "key": "value } "#;
    let serde_error = serde_json::from_str::<Value>(invalid_json_data)
      .err().expect("Expected JSON parsing to fail");

    // act
    let app_error = Error::from(serde_error);

    // assert
    assert_eq!(app_error.kind(), &ErrorKind::JsonDeserialization);
  }
}
