use serde::{Serialize,Deserialize};

// struct ContainerConfig {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Layer {
  id: String,
  os: String,
  parent: Option<String>,
  // container_config: ContainerConfig,
  created: String,
}

impl Layer {
  pub fn id(&self) -> &str {
    &self.id
  }

  pub fn parent(&self) -> Option<&str> {
    match &self.parent {
      Some(parent_id) => Some(parent_id),
      None => None
    }
  }
}
