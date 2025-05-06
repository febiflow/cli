use clap::ArgMatches;

use crate::{
  utils::{
    CommandUtil, CommandUtilProvider,
    InputUtil, InputUtilProvider
  }, Result
};

pub fn handle_new(_matches: &ArgMatches) -> Result<()> {
    println!("🔧 Injecting dependencies...");
    let input_util = InputUtil::new();
    let cmd_util = CommandUtil::new();

    let repo_url: &str = "https://github.com/febiflow/febiflow";
    let default_branch: &str = "master";
    let project_name = input_util.read_line("Enter project name: ")?;
    println!("📁 Creating project '{project_name}'...");

    cmd_util.execute("mkdir", "-p", &[&project_name])?;

    let archive_cmd = format!(
        "curl -L {repo_url}/archive/refs/heads/{default_branch}.tar.gz \
        | tar xz --strip-components=1 -C {project_name}");
    cmd_util.execute("sh", "-c", &[&archive_cmd])?;

    let git_init_cmd = format!("cd {} && git init", project_name);
    cmd_util.execute("sh", "-c", &[&git_init_cmd])?;

    let git_commit_cmd = format!("cd {} && git add . && git commit -m 'Initial commit'", project_name);
    cmd_util.execute("sh", "-c", &[&git_commit_cmd])?;

    println!("✅ Project '{project_name}' is ready!");
    Ok(())
}
