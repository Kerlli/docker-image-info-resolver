use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RootFs {
  #[serde(rename = "type")]
  fs_type: String,
  diff_ids: Vec<String>,
}

impl RootFs {
  pub fn diff_ids(&self) -> &Vec<String> {
    &self.diff_ids
  }
}
