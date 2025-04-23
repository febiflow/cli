use clap::ArgMatches;

use crate::{
  utils::file_util::FileUtil,
  Result,
};

pub fn handle_deploy(matches: &ArgMatches) -> Result<()> {
  println!("💉 Loading and injecting dependencies...");
  let file_util = FileUtil::new();
  

  Ok(())
}
