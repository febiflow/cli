pub mod args;
pub mod commands;

use clap::Command;

use args::{
  build_binary_name_arg,
  build_config_profile_name_arg,
  build_lambda_function_name_arg,
};

pub fn build_cli() -> Command {
  Command::new("febiflow")
    .about("A simple CLI tool for febiflow projects")
    .subcommand(Command::new("new")
      .about("Create a new febiflow project"))
    .subcommand(Command::new("deploy")
      .about("Deploy febiflow project using cargo-lambda")
      .arg(build_binary_name_arg())
      .arg(build_lambda_function_name_arg())
      .arg(build_config_profile_name_arg()))
}
