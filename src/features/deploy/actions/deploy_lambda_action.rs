use std::rc::Rc;

use crate::{
  features::config::dtos::actions::FebiFlowConfigAws,
  utils::CommandUtilProvider,
  Result,
};

/// A provider for deploying AWS Lambda functions.
///
/// This trait defines a single high‑level operation for packaging and
/// deploying a Lambda function using provided configuration and
/// environment variables.
pub trait DeployLambdaActionProvider {
  /// Execute the deployment of a Lambda function.
  ///
  /// # Parameters
  ///
  /// - `lambda_function_name`: The name of the Lambda function to deploy.
  /// - `bin_name`: The local binary name to package for AWS Lambda.
  /// - `febiflow_config_aws`: Configuration struct containing AWS region and IAM role.
  /// - `env_variable_lines`: A list of `"KEY=VALUE"` strings to set as environment variables.
  ///
  /// # Returns
  ///
  /// Returns `Ok(())` if deployment succeeds, or an error `Result` on failure.
  fn execute(
    &self,
    lambda_function_name: &str,
    bin_name: &str,
    febiflow_config_aws: &FebiFlowConfigAws,
    env_variable_lines: &Vec<String>,
  ) -> Result<()>;
}

/// Concrete implementation of [`DeployLambdaActionProvider`] using a
/// command‑execution utility.
///
/// Holds an `Rc`‑wrapped provider that actually runs the external
/// `cargo lambda deploy` command.
pub struct DeployLambdaAction<C: CommandUtilProvider> {
  command_util: Rc<C>,
}

impl<C: CommandUtilProvider> DeployLambdaAction<C> {
  /// Create a new `DeployLambdaAction` with the given command util.
  ///
  /// # Parameters
  ///
  /// - `command_util`: A reference‑counted implementation of
  ///   [`CommandUtilProvider`] which will be used to invoke commands.
  pub fn new(command_util: Rc<C>) -> Self {
    Self { command_util }
  }
}

impl<C: CommandUtilProvider> DeployLambdaActionProvider for DeployLambdaAction<C> {
  /// Deploys the specified binary as an AWS Lambda function.
  ///
  /// This will:
  /// 1. Print a preparation message.
  /// 2. Build the `cargo lambda deploy` command arguments, including region,
  ///    IAM role, and environment variables.
  /// 3. Invoke the `cargo lambda` command via the provided
  ///    [`CommandUtilProvider`].
  /// 4. Print success confirmation upon completion.
  ///
  /// # Errors
  ///
  /// Returns an error if the underlying command execution fails.
  fn execute(
    &self,
    lambda_function_name: &str,
    bin_name: &str,
    febiflow_config_aws: &FebiFlowConfigAws,
    env_variable_lines: &Vec<String>,
  ) -> Result<()> {
    println!("⚙️ Preparing deployment for lambda function {}...", lambda_function_name);

    let mut deploy_args: Vec<&str> = vec![
      "deploy",
      "--binary-name", bin_name,
      lambda_function_name,
      "--region", &febiflow_config_aws.region,
      "--iam-role", &febiflow_config_aws.iam_role,
    ];

    deploy_args.extend(
      env_variable_lines
        .iter()
        .flat_map(|line| ["--env-var", line.as_str()]),
    );

    println!(
      "🚀 Deploying lambda function {} to AWS region {}...",
      lambda_function_name, &febiflow_config_aws.region
    );

    self.command_util.execute("cargo", "lambda", &deploy_args)?;
    println!("✅ Deployment successful!");

    Ok(())
  }
}
