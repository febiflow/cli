//! Configuration utilities for reading a config on a febiflow project.
//!
//! This module provides functionality to read febiflow configuration files stored in the
//! `.febiflow` folder using the file utilities provided by `FileUtilProvider`. It handles
//! reading and parsing JSON config into the `FebiFlowConfig` structure.

use std::rc::Rc;

use crate::{
  features::config::dtos::actions::FebiFlowConfig,
  utils::FileUtilProvider,
  Result
};

/// Defines the interface for loading febiflow configuration.
///
/// This trait provides a standardized interface for loading febiflow configuration.
/// It's extracted as a trait to facilitate testing through dependency injection.
pub trait LoadConfigurationActionProvider {
    /// Loads the febiflow configuration file.
    ///
    /// # Returns
    ///
    /// * `Ok(FebiFlowConfig)` - The parsed configuration if loading is successful
    /// * `Err(Error)` - If loading or parsing fails, with details in the error
    ///
    /// # Example
    ///
    /// ```
    /// let provider = get_load_config_provider();
    /// match provider.execute() {
    ///     Ok(config) => println!("Loaded configuration: {:#?}", config),
    ///     Err(e) => eprintln!("Loading of config failed: {}", e),
    /// }
    /// ```
    fn execute(&self) -> Result<FebiFlowConfig>;
}

/// Concrete implementation for loading febiflow configuration.
///
/// This action reads and parses the febiflow configuration file from the
/// standard location in the `.febiflow` directory.
pub struct LoadConfigurationAction<F: FileUtilProvider> {
    file_util: Rc<F>,
}

impl<F: FileUtilProvider> LoadConfigurationAction<F> {
    /// Creates a new instance with the specified file utility provider.
    ///
    /// # Parameters
    ///
    /// * `file_util` - A reference-counted provider for file utilities
    ///
    /// # Returns
    ///
    /// A new `LoadConfigurationAction` instance configured with the provided file utility
    ///
    /// # Example
    ///
    /// ```
    /// use std::rc::Rc;
    /// use febiflow_cli::utils::FileUtil;
    /// use febiflow_cli::actions::LoadConfigurationAction;
    ///
    /// let file_util = Rc::new(FileUtil::new());
    /// let action = LoadConfigurationAction::new(file_util);
    /// ```
    pub fn new(file_util: Rc<F>) -> Self {
        Self { file_util }
    }
}

impl<F: FileUtilProvider> LoadConfigurationActionProvider for LoadConfigurationAction<F> {
    /// Loads and parses the febiflow configuration file from the standard location.
    ///
    /// This implementation reads the JSON configuration from `.febiflow/config.json`
    /// and parses it into a `FebiFlowConfig` structure.
    ///
    /// # Returns
    ///
    /// * `Ok(FebiFlowConfig)` - The parsed configuration if loading is successful
    /// * `Err(Error)` - If reading or parsing fails
    ///
    /// # Example
    ///
    /// ```
    /// use std::rc::Rc;
    /// use febiflow_cli::utils::FileUtil;
    /// use febiflow_cli::actions::{LoadConfigurationAction, LoadConfigurationActionProvider};
    ///
    /// let file_util = Rc::new(FileUtil::new());
    /// let action = LoadConfigurationAction::new(file_util);
    ///
    /// // Load the configuration
    /// match action.execute() {
    ///    Ok(config) => {
    ///        // Use the configuration
    ///        println!("Project name: {}", config.project_name);
    ///    },
    ///    Err(e) => eprintln!("Failed to load config: {}", e),
    /// }
    /// ```
    fn execute(&self) -> Result<FebiFlowConfig> {
        let file_path = ".febiflow/config.json";

        println!("🗂️ Reading configuration from {}...", file_path);
        let febiflow_config = self.file_util.read_json(file_path)?;
        println!("✅ Configuration loaded successfully");

        Ok(febiflow_config)
    }
}
