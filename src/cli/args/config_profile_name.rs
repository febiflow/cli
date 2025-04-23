use clap::Arg;

pub fn build_config_profile_name_arg() -> Arg {
  Arg::new("config_profile_name")
    .long("profile")
    .value_name("CONFIG_PROFILE_NAME")
    .help("Enter the profile name to use for deployment")
    .required(true)
}
