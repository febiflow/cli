use std::rc::Rc;

use clap::ArgMatches;

use crate::{
  features::{
    build::actions::{BuildLambdaBinaryAction, BuildLambdaBinaryActionProvider},
    config::actions::{LoadConfigurationAction, LoadConfigurationActionProvider},
    deploy::actions::{
      DeployLambdaAction,
      DeployLambdaActionProvider,
      LoadEnvironmentVariablesAction,
      LoadEnvironmentVariablesActionProvider,
    },
  },
  utils::{CommandUtil, FileUtil},
  Error,
  ErrorKind,
  Result,
};

/// Handle the `deploy` subcommand by orchestrating configuration loading,
/// environment variable injection, binary build, and AWS Lambda deployment.
///
/// This function ties together the following actions:
/// 1. Load the global configuration (including AWS settings and profiles).
/// 2. Load environment variables for the chosen profile.
/// 3. Build the specified Lambda binary using `cargo lambda build`.
/// 4. Deploy the built binary to AWS Lambda.
///
/// # Parameters
///
/// - `matches`: The parsed command-line arguments containing:
///   - `bin_name`: The local binary name to build and deploy.
///   - `lambda_name`: The target AWS Lambda function name.
///   - `profile_name`: The configuration profile to use for AWS credentials.
///
/// # Errors
///
/// Returns an error if any of the following steps fail:
/// - Configuration loading (`LoadConfigurationAction`).
/// - Profile lookup in the loaded configuration.
/// - Environment variable loading (`LoadEnvironmentVariablesAction`).
/// - Binary build (`BuildLambdaBinaryAction`).
/// - Lambda deployment (`DeployLambdaAction`).
///
/// # Examples
///
/// ```bash
/// mycli deploy --bin-name my_func --lambda-name my_lambda --profile-name default
/// ```
///
pub fn handle_deploy(matches: &ArgMatches) -> Result<()> {
  println!("💉 Loading and injecting dependencies...");

  // Utilities for file and command operations
  let file_util = Rc::new(FileUtil::new());
  let command_util = Rc::new(CommandUtil::new());

  // Action providers for each step
  let load_configuration = LoadConfigurationAction::new(Rc::clone(&file_util));
  let load_environment_variables = LoadEnvironmentVariablesAction::new(file_util);
  let build_lambda_binary = BuildLambdaBinaryAction::new(Rc::clone(&command_util));
  let deploy_lambda = DeployLambdaAction::new(command_util);

  // Extract CLI arguments
  let bin_name: &String = matches.get_one("bin_name").unwrap();
  let lambda_function_name: &String = matches.get_one("lambda_name").unwrap();
  let profile_name: &String = matches.get_one("profile_name").unwrap();

  // Load the global configuration file
  let febiflow_config = load_configuration.execute()?;

  // Lookup the specified profile within the loaded configuration
  let profile = febiflow_config
    .profiles
    .get(profile_name)
    .ok_or_else(|| Error::new(ErrorKind::ProfileNotFound, "Profile not found"))?;

  // Load and format environment variables for the profile
  let env_variable_lines = load_environment_variables.execute(&profile)?;

  // Build the Lambda binary
  build_lambda_binary.execute(&bin_name)?;

  // Deploy the built binary to AWS Lambda with the retrieved config and env vars
  deploy_lambda.execute(
    &lambda_function_name,
    &bin_name,
    &febiflow_config.aws,
    &env_variable_lines,
  )?;

  Ok(())
}
