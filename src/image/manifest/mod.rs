mod config;

pub use config::*;

use serde::{Serialize,Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Manifest {
  #[serde(rename(deserialize = "Config"))]
  config: String,
  #[serde(rename(deserialize = "RepoTags"))]
  repo_tags: Option<Vec<String>>,
  #[serde(rename(deserialize = "Layers"))]
  layers: Vec<String>,
}

impl Manifest {
  pub fn config(&self) -> &str {
    &self.config
  }
}
