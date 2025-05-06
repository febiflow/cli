use febiflow_cli::{cli, Result};

fn handler() -> Result<()> {
  let matches = cli::build_cli().get_matches();

  if let Some(matches) = matches.subcommand_matches("deploy") {
    cli::commands::handle_deploy(matches)?;
  } else if let Some(matches) = matches.subcommand_matches("new") {
    cli::commands::handle_new(matches)?;
  }

  Ok(())
}

fn main() {
  if let Err(error) = handler() {
    eprintln!("{:#?}", error);
  }
}

