//! A simple input utility module.
//!
//! This module defines the `InputUtilProvider` trait for reading user input
//! with a prompt message, and provides a concrete `InputUtil` implementation
//! that reads from standard input (`stdin`) and flushes to standard output (`stdout`).
//!
//! # Examples
//!
//! ```no_run
//! use your_crate::{InputUtil, InputUtilProvider};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let input = InputUtil::new();
//!     let name = input.read_line("Enter your name: ")?;
//!     println!("Hello, {}!", name);
//!     Ok(())
//! }
//! ```
//!

use std::io::{self, Write};

use crate::Result;

/// A provider for reading lines of input from the user.
///
/// This trait abstracts over any type that can prompt the user and read
/// a line of text, returning it as a `String`.
pub trait InputUtilProvider {
    /// Prompts the user with `message`, reads a line of input, trims it,
    /// and returns the result as a `String`.
    ///
    /// # Arguments
    ///
    /// * `message` - The prompt message to display to the user.
    ///
    /// # Returns
    ///
    /// A `Result<String>` containing the trimmed user input, or an error
    /// if flushing stdout or reading from stdin fails.
    fn read_line(&self, message: &str) -> Result<String>;
}

/// A concrete implementation of [`InputUtilProvider`] that reads from `stdin`.
///
/// This struct has no state and can be constructed via [`InputUtil::new`].
pub struct InputUtil;

impl InputUtil {
    /// Creates a new `InputUtil` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// # use your_crate::InputUtil;
    /// let util = InputUtil::new();
    /// ```
    pub fn new() -> Self {
        InputUtil {}
    }
}

impl InputUtilProvider for InputUtil {
    /// Reads a line of input from the user after displaying `message`.
    ///
    /// This method:
    /// 1. Prints the prompt message without a newline.
    /// 2. Flushes stdout to ensure the prompt is shown immediately.
    /// 3. Reads a line from stdin into an internal buffer.
    /// 4. Trims leading and trailing whitespace.
    /// 5. Returns the trimmed string.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Flushing stdout fails.
    /// - Reading from stdin fails.
    fn read_line(&self, message: &str) -> Result<String> {
        let mut user_input = String::new();

        print!("{}", message);
        io::stdout().flush()?;
        io::stdin().read_line(&mut user_input)?;

        let user_input = user_input.trim();
        Ok(user_input.to_string())
    }
}
