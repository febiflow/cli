use clap::Arg;

pub fn build_binary_name_arg() -> Arg {
  Arg::new("binary_name")
    .long("bin")
    .value_name("BINARY_NAME")
    .help("Specify the binary target name")
    .required(true)
}
