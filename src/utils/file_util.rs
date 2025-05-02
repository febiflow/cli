//! Provides functionality for file operations, including reading of JSON files
//! and text files line by line.
//!
//! This module defines a trait `FileUtilProvider` for abstracting file reading
//! operations and provides concrete implementation `FileUtil` that interacts
//! with the local file system.

use std::{
  fs::File,
  io::{self, BufRead, BufReader},
  path::Path
};

use serde::de::DeserializeOwned;

use crate::Result;

/// A trait defining contract for file utility providers.
///
/// This abstraction allow for different implementations of file reading
/// utilities, facilitating mocking for tests or supporting various file
/// sources beyond the standard file system.
pub trait FileUtilProvider {
  /// Reads a JSON file from the specified path and deserializes it into a
  /// given type `T`.
  ///
  /// # Type Parameters
  ///
  /// * `T` - The target type to deserialize the JSON data into. This type must
  ///   implement `serde::de::DeserializeOwned`.
  ///
  /// # Arguments
  ///
  /// * `file_path` - A type that can be referenced as a `Path` (e.g., `&str`,
  ///   `String`, `PathBuf`)
  ///
  /// # Returns
  ///
  /// * `Result<T>` - A `Result` containing the deserialized data type of
  ///   type `T` on success. On failure, it returns an error, typically
  ///   wrapping `std::io::Error` (e.g., file not found) or `serde_json::Error`
  ///   (e.g., malformed JSON), contained within the crate's custom `Result`
  ///   type.
  fn read_json<T>(&self, file_path: impl AsRef<Path>) -> Result<T>
    where T: DeserializeOwned;

  /// Reads a file from the specified path line by line into a vector of strings.
  ///
  /// Each string in the resulting vector corresponds to one line from the file,
  /// with newline characters typically stripped.
  ///
  /// # Arguments
  ///
  /// * `file_path`: A type that can be referenced as a `Path` (e.g., `&str`, `String`, `PathBuf`)
  ///   representing the location of the text file.
  ///
  /// # Returns
  ///
  /// * `Result<Vec<String>>`: A `Result` containing a `Vec<String>` where each element
  ///   is a line from the file on success.
  ///   On failure, it returns an error, typically wrapping `std::io::Error` (e.g., file not found,
  ///   read error, invalid UTF-8 data), contained within the crate's custom `Result` type.
  fn read_lines(&self, file_path: impl AsRef<Path>) -> Result<Vec<String>>;
}

/// A concrete implementation of `FileUtilProvider` for standard file system
/// operations.
///
/// This struct provides the actual logic for reading files from the local disk
/// using standard Rust file I/O operations. It is a zero-sized struct as it
/// doesn't need to hold any state.
pub struct FileUtil;

impl FileUtil {
  /// Creates a new instance of `FileUtil`.
  ///
  /// As `FileUtil` is a zero-sized struct (contains no data), this function
  /// simply constructs and returns a default instance.
  pub fn new() -> Self { Self {} }
}

/// Implements the `FileUtilProvider` trait for the `FileUtil` struct, providing
/// concrete file reading functionality based on the standard library.
impl FileUtilProvider for FileUtil {
  /// Reads a JSON file from the specified path and deserializes it into type `T`.
  ///
  /// This implementation performs the following steps:
  /// 1. Opens the file at the given `file_path` using `std::fs::File::open`.
  /// 2. Wraps the file handle in a `std::io::BufReader` for efficient, buffered
  ///    reading.
  /// 3. Uses `serde_json::from_reader` to parse the JSON data from the reader
  ///    directly into an instance of the requested type `T`.
  ///
  /// # Type Parameters
  ///
  /// * `T`: The target type for deserialization, constrained by
  ///   `serde::de::DeserializeOwned`.
  ///
  /// # Arguments
  ///
  /// * `file_path`: An object convertible to a `Path` reference pointing to
  ///   the JSON file.
  ///
  /// # Returns
  ///
  /// * `Result<T>`: `Ok(T)` with the deserialized data if successful.
  /// * `Err`: An error if the file cannot be opened (`std::io::Error`) or if
  ///    the JSON parsing fails (`serde_json::Error`), wrapped within the
  ///    crate's `Result` type via the `?` operator.
  fn read_json<T>(&self, file_path: impl AsRef<Path>) -> Result<T>
  where T: DeserializeOwned {
    let file = File::open(file_path.as_ref())?;
    let reader = BufReader::new(file);

    Ok(serde_json::from_reader(reader)?)
  }

  /// Reads a file from the specified path line by line into a vector of strings.
  ///
  /// This implementation performs the following steps:
  /// 1. Opens the file at the given `file_path` using `std::fs::File::open`.
  /// 2. Wraps the file handle in a `std::io::BufReader`.
  /// 3. Uses the `lines()` method (from the `BufRead` trait) to get an iterator
  ///    over the lines of the file. Each item yielded by `lines()` is an
  ///    `io::Result<String>`.
  /// 4. Collects the results from the iterator into a
  ///    `io::Result<Vec<String>>`. This collection process stops early if any
  ///    line read results in an `io::Error`.
  /// 5. Uses the `?` operator to propagate any `io::Error` encountered during
  ///    file opening or line reading, wrapping it in the crate's `Result` type.
  ///
  /// # Arguments
  ///
  /// * `file_path`: An object convertible to a `Path` reference pointing to the text file.
  ///
  /// # Returns
  ///
  /// * `Result<Vec<String>>`: `Ok(Vec<String>)` containing all lines from the file if successful.
  /// * `Err`: An error if the file cannot be opened or if any line fails to read
  ///   (e.g., due to I/O issues or invalid UTF-8 data), wrapped within the crate's `Result` type.
  fn read_lines(&self, file_path: impl AsRef<Path>) -> Result<Vec<String>> {
    let file = File::open(file_path.as_ref())?;
    let reader = BufReader::new(file);

    Ok(reader.lines().collect::<io::Result<Vec<String>>>()?)
  }
}
