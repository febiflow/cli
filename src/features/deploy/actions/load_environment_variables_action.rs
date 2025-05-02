//! Load Environment Variables Action
//!
//! This module defines a trait for loading environment variables from a path
//! specified in a `FebiFlowConfigProfile`, and a file‑based implementation
//! that reads the variables via a `FileUtilProvider`.

use std::rc::Rc;

use crate::{
  features::config::dtos::actions::FebiFlowConfigProfile,
  utils::FileUtilProvider,
  Result,
};

/// Abstracts loading of environment variable lines from a configuration profile.
///
/// Extracted as a trait to decouple the loading logic from any particular I/O
/// mechanism, enabling easy testing and dependency injection.
pub trait LoadEnvironmentVariablesActionProvider {
  /// Loads environment variables specified in the given profile.
  ///
  /// # Parameters
  ///
  /// - `profile`: Contains the path to the `.env` file via `profile.env_path`.
  ///
  /// # Returns
  ///
  /// - `Ok(Vec<String>)`: The lines of the env file, each representing one
  ///   environment variable (e.g., `"KEY=VALUE"`).
  /// - `Err`: An error if reading the file fails.
  fn execute(&self, profile: &FebiFlowConfigProfile) -> Result<Vec<String>>;
}

/// File‑based implementation of [`LoadEnvironmentVariablesActionProvider`].
///
/// Uses a `FileUtilProvider` to read the `.env` file from disk (or any
/// filesystem abstraction).
pub struct LoadEnvironmentVariablesAction<F: FileUtilProvider> {
  file_util: Rc<F>,
}

impl<F: FileUtilProvider> LoadEnvironmentVariablesAction<F> {
  /// Constructs a new loader with the given file utility.
  ///
  /// # Parameters
  ///
  /// - `file_util`: A reference‑counted implementation of `FileUtilProvider`.
  pub fn new(file_util: Rc<F>) -> Self {
      Self { file_util }
  }
}

impl<F: FileUtilProvider> LoadEnvironmentVariablesActionProvider for LoadEnvironmentVariablesAction<F> {
  /// Reads and returns all lines from the environment file at `profile.env_path`.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use std::rc::Rc;
  /// use febiflow_cli::features::config::dtos::actions::FebiFlowConfigProfile;
  /// use febiflow_cli::utils::{FileUtilProvider, FileUtil};
  /// use febiflow_cli::features::deploy::actions::{
  ///   LoadEnvironmentVariablesAction, LoadEnvironmentVariablesActionProvider,
  /// };
  ///
  /// # fn example() -> febiflow_cli::Result<()> {
  /// let profile = FebiFlowConfigProfile { env_path: ".env".into() };
  /// let file_util = Rc::new(FileUtil::new());
  /// let loader = LoadEnvironmentVariablesAction::new(file_util);
  /// let variables: Vec<String> = loader.execute(&profile)?;
  /// println!("Loaded {} variables", variables.len());
  /// # Ok(())
  /// # }
  /// ```
  /// # Errors
  ///
  /// Returns an error if reading of line fails, for example:
  /// - File not found
  /// - Permission denied
  fn execute(&self, profile: &FebiFlowConfigProfile) -> Result<Vec<String>> {
      println!("🗂️ Reading environment variables from {}...", profile.env_path);
      let lines = self.file_util.read_lines(&profile.env_path)?;
      println!("✅ Environment variables loaded");
      Ok(lines)
  }
}
