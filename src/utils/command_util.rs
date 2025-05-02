//! Provides functionalities for executing external commands.
//!
//! This module defines:
//! - [`CommandUtilProvider`]: an abstraction over external command execution.
//! - [`CommandUtil`]: a concrete implementation using `std::process::Command`.
//!
//! # Examples
//!
//! ```rust,no_run
//! use crate::{CommandUtil, CommandUtilProvider};
//!
//! fn main() -> crate::Result<()> {
//!   let util = CommandUtil::new();
//!   let output = util.execute("echo", "", &["Hello, Rust!"])?;
//!   println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
//!   Ok(())
//! }
//! ```
//! Useful for build scripts, testing CLIs, or shell automation.

use std::process::{Command, Output};

use crate::Result;

/// Executes external commands via a pluggable provider.
///
/// This trait abstracts over different command‐execution backends,
/// enabling mock implementations for testing or alternative shells.
///
/// # Examples
///
/// ```rust
/// # use crate::{CommandUtil, CommandUtilProvider};
/// # fn example() -> crate::Result<()> {
/// let provider = CommandUtil::new();
/// let output = provider.execute("ls", "-la", &["/"])?;
/// assert!(output.status.success());
/// # Ok(())
/// # }
/// ```
pub trait CommandUtilProvider {
  /// Executes an external command with the specified `program`, `subcommand`, and `arguments`.
  ///
  /// If `subcommand` is an empty string, no subcommand is passed.
  ///
  /// # Parameters
  ///
  /// - `program`: Name or path of the executable (e.g., `"cargo"`).
  /// - `subcommand`: Optional primary command or action (e.g., `"build"`). Empty string means none.
  /// - `arguments`: A slice of additional arguments passed after the subcommand.
  ///
  /// # Returns
  ///
  /// - `Ok(Output)`: Captures `stdout`, `stderr`, and exit status, even if the command
  ///   returns a non-zero code.
  /// - `Err(crate::Error)`: On failure to launch the command (e.g., missing executable,
  ///   permission denied, I/O error).
  fn execute<'a>(
    &self,
    program: &str,
    subcommand: &str,
    arguments: &[&'a str],
  ) -> Result<Output>;
}

/// Concrete implementation of [`CommandUtilProvider`] using `std::process::Command`.
///
/// `CommandUtil` is stateless and lightweight: you can create multiple instances cheaply.
pub struct CommandUtil;

impl CommandUtil {
  /// Creates a new [`CommandUtil`] instance.
  ///
  /// # Examples
  ///
  /// ```rust
  /// # use crate::CommandUtil;
  /// let util = CommandUtil::new();
  /// ```
  pub fn new() -> Self {
    CommandUtil
  }
}

impl CommandUtilProvider for CommandUtil {
  /// Executes a command by constructing a `std::process::Command`, adding the
  /// optional `subcommand` and `arguments`, then capturing its output.
  ///
  /// # Examples
  ///
  /// ```rust
  /// # use crate::{CommandUtil, CommandUtilProvider};
  /// # fn example() -> crate::Result<()> {
  /// let util = CommandUtil::new();
  /// let out = util.execute("echo", "", &["Hello, world!"])?;
  /// assert!(out.status.success());
  /// # Ok(())
  /// # }
  /// ```
  ///
  /// # Errors
  ///
  /// Returns an error if `Command::output` fails, for example:
  /// - Executable not found
  /// - Permission denied
  /// - Other I/O errors
  ///
  /// # Panics
  ///
  /// This implementation never panics.
  fn execute<'a>(
    &self,
    program: &str,
    subcommand: &str,
    arguments: &[&'a str],
  ) -> Result<Output> {
    let mut cmd = Command::new(program);
    if !subcommand.is_empty() {
      cmd.arg(subcommand);
    }
    cmd.args(arguments);
    Ok(cmd.output()?)
  }
}
