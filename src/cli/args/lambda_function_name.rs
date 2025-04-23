use clap::Arg;

pub fn build_lambda_function_name_arg() -> Arg {
  Arg::new("lambda_function_name")
    .long("lambda")
    .value_name("LAMBDA_FUNCTION_NAME")
    .help("Enter the lambda function name")
    .required(true)
}
