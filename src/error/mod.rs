use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub enum ResolverError {
  IoError(std::io::Error),
  SerdeJsonError(serde_json::Error),
}

impl Error for ResolverError {}

impl Display for ResolverError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
    match self {
      Self::IoError(err) => write!(f, "IO error: {}", err),
      Self::SerdeJsonError(err) => write!(f, "Serde error: {}", err)
    }
  }
}

impl From<std::io::Error> for ResolverError {
  fn from(err: std::io::Error) -> Self {
    Self::IoError(err)
  }
}

impl From<serde_json::Error> for ResolverError {
  fn from(err: serde_json::Error) -> Self {
    Self::SerdeJsonError(err)
  }
}
