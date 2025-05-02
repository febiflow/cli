use std::rc::Rc;

use clap::ArgMatches;

use crate::{
  features::build::actions::{BuildLambdaBinaryAction, BuildLambdaBinaryActionProvider},
  utils::CommandUtil, Result
};

pub fn handle_build(matches: &ArgMatches) -> Result<()> {
  let command_util = Rc::new(CommandUtil::new());
  let build_lambda_binary = BuildLambdaBinaryAction::new(command_util);
  let bin_name: &String = matches.get_one("bin_name").unwrap();

  build_lambda_binary.execute(&bin_name)?;

  Ok(())
}
