//! Lambda build utilities for compiling Rust binaries for AWS Lambda deployment.
//!
//! This module provides functionality to build Rust binaries specifically formatted
//! for AWS Lambda deployment using the `cargo lambda` toolchain. It handles the
//! necessary cross-compilation settings and target architecture configuration.

use std::rc::Rc;

use crate::{utils::command_util::CommandUtilProvider, Result};

/// Defines the interface for building Lambda-compatible binaries.
///
/// This trait provides a standardized interface for building Rust binaries
/// that can be deployed to AWS Lambda. It's extracted as a trait to facilitate
/// testing through dependency injection.
pub trait BuildLambdaBinaryActionProvider {
  /// Builds the specified binary for AWS Lambda deployment.
  ///
  /// This method handles the necessary cross-compilation steps to create
  /// a binary that's compatible with the AWS Lambda runtime environment.
  ///
  /// # Parameters
  ///
  /// * `bin_name` - The name of the binary target to build, as defined in Cargo.toml
  ///
  /// # Returns
  ///
  /// * `Ok(())` - If the build process completes successfully
  /// * `Err(Error)` - If any part of the build process fails, with details in the error
  ///
  /// # Example
  ///
  /// ```
  /// let provider = get_build_provider();
  /// match provider.execute("my_lambda_function") {
  ///     Ok(_) => println!("Build successful"),
  ///     Err(e) => eprintln!("Build failed: {}", e),
  /// }
  /// ```
  fn execute(&self, bin_name: &str) -> Result<()>;
}

/// Concrete implementation for building Lambda-compatible binaries.
///
/// This struct encapsulates the functionality needed to build a Rust binary
/// that can be deployed to AWS Lambda, using the cargo-lambda tool.
pub struct BuildLambdaBinaryAction<C: CommandUtilProvider> {
  command_util: Rc<C>,
}

impl<C: CommandUtilProvider> BuildLambdaBinaryAction<C> {
  /// Creates a new instance with the specified command utility provider.
  ///
  /// # Parameters
  ///
  /// * `command_util` - A reference-counted provider for executing shell commands
  ///
  /// # Returns
  ///
  /// A new `BuildLambdaBinaryAction` instance configured with the provided command utility
  ///
  /// # Example
  ///
  /// ```
  /// use std::rc::Rc;
  /// use febiflow_cli::utils::CommandUtil;
  /// use febiflow_cli::actions::BuildLambdaBinaryAction;
  ///
  /// let command_util = Rc::new(CommandUtil::new());
  /// let action = BuildLambdaBinaryAction::new(command_util);
  /// ```
  pub fn new(command_util: Rc<C>) -> Self {
      BuildLambdaBinaryAction { command_util }
  }
}

impl<C: CommandUtilProvider> BuildLambdaBinaryActionProvider
for BuildLambdaBinaryAction<C> {
  /// Builds the specified binary for AWS Lambda deployment.
  ///
  /// This method uses cargo-lambda to build a release binary with the
  /// appropriate target architecture for AWS Lambda (x86_64-unknown-linux-gnu).
  ///
  /// # Parameters
  ///
  /// * `bin_name` - The name of the binary target to build
  ///
  /// # Returns
  ///
  /// * `Ok(())` - If the build completes successfully
  /// * `Err(Error)` - If the build process encounters an error
  ///
  /// # Example
  ///
  /// ```
  /// use std::rc::Rc;
  /// use febiflow_cli::utils::CommandUtil;
  /// use febiflow_cli::actions::{BuildLambdaBinaryAction, BuildLambdaBinaryActionProvider};
  ///
  /// let command_util = Rc::new(CommandUtil::new());
  /// let action = BuildLambdaBinaryAction::new(command_util);
  ///
  /// // This will execute:
  /// // cargo lambda build --release --bin create_user_bin --target x86_64-unknown-linux-gnu
  /// let result = action.execute("create_user_bin");
  /// ```
  ///
  /// # Notes
  ///
  /// Requires the cargo-lambda tool to be installed. If not installed,
  /// this method will return an error.
  fn execute(&self, bin_name: &str) -> Result<()> {
      println!("🛠️ Building lambda binary...");
      self.command_util.execute("cargo", "lambda", &[
          "build", "--release",
          "--bin", bin_name,
          "--target", "x86_64-unknown-linux-gnu",
      ])?;
      println!("✅ Build successful");

      Ok(())
  }
}
