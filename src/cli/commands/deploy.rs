use std::rc::Rc;

use clap::ArgMatches;

use crate::{
  features::{
    config::actions::{
      LoadConfigurationAction, LoadConfigurationActionProvider
    }, deploy::actions::{
      LoadEnvironmentVariablesAction, LoadEnvironmentVariablesActionProvider
    }
  }, utils::{CommandUtil, FileUtil},
  Error, ErrorKind, Result
};

pub fn handle_deploy(matches: &ArgMatches) -> Result<()> {
  println!("💉 Loading and injecting dependencies...");
  let file_util = Rc::new(FileUtil::new());
  let command_util = CommandUtil::new();
  let load_configuration = LoadConfigurationAction::new(Rc::clone(&file_util));
  let load_environment_variables = LoadEnvironmentVariablesAction::new(file_util);
  // let build_lambda_binary = BuildLambdaBinaryAction::new(&command_util);
  // let deploy_lambda = DeployLambdaAction::new(&command_util);

  let bin_name: &String = matches.get_one("bin_name").unwrap();
  let lambda_function_name: &String = matches.get_one("lambda_name").unwrap();
  let profile_name: &String = matches.get_one("profile_name").unwrap();

  let febiflow_config = load_configuration.execute()?;

  let profile = febiflow_config.profiles
    .get(profile_name)
    .ok_or_else(|| Error::new(ErrorKind::ProfileNotFound, "Profile not found"))?;

  let env_variable_lines = load_environment_variables.execute(&profile)?;

  // build_lambda_binary.execute(&bin_name)?;
  // deploy_lambda.execute(
  //   &lambda_function_name,
  //   &bin_name,
  //   &febiflow_config.aws,
  //   &env_variable_lines,
  // )?;

  Ok(())
}
