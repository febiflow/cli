use std::{fs::File, io::BufReader, path::Path};

use serde::de::DeserializeOwned;

use crate::Result;

pub trait FileUtilProvider {
  fn read_json<T>(&self, file_path: impl AsRef<Path>) -> Result<T> where T: DeserializeOwned;
}

pub struct FileUtil;

impl FileUtil {
  pub fn new() -> Self { Self {} }
}

impl FileUtilProvider for FileUtil {
  fn read_json<T>(&self, file_path: impl AsRef<Path>) -> Result<T>
  where T: DeserializeOwned {
    let file = File::open(file_path.as_ref())?;
    let reader = BufReader::new(file);

    Ok(serde_json::from_reader(reader)?)
  }
}
