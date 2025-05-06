use std::io::{self, Write};

use crate::Result;

pub trait InputUtilProvider {
  fn read_line(&self, message: &str) -> Result<String>;
}

pub struct InputUtil;

impl InputUtil {
  pub fn new() -> Self { InputUtil {} }
}

impl InputUtilProvider for InputUtil {
  fn read_line(&self, message: &str) -> Result<String> {
    let mut user_input = String::new();

    print!("{}", message);
    io::stdout().flush()?;
    io::stdin().read_line(&mut user_input)?;

    let user_input = user_input.trim();
    Ok(user_input.to_string())
  }
}
