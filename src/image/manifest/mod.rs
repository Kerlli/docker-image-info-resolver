mod config;

pub use config::*;

use serde::{Serialize, Deserialize, Deserializer, de::Error};
use serde_json::Value;

use super::{
  media_type::{MediaType, ManifestMediaType, LayerMediaType},
  descriptor::Descriptor,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Manifest {
  #[serde(rename(deserialize = "Config"))]
  config: String,
  #[serde(skip_deserializing, default = "default_media_type")]
  media_type: MediaType,
  #[serde(rename(deserialize = "RepoTags"))]
  repo_tags: Option<Vec<String>>,
  #[serde(default = "default_schema_version")]
  schema_version: u8,
  #[serde(rename(deserialize = "Layers"), deserialize_with = "deserialize_layers")]
  layers: Vec<Descriptor>,
}

fn default_media_type() -> MediaType {
  MediaType::Manifest(ManifestMediaType::V1)
}

fn default_schema_version() -> u8 { 2 }

fn deserialize_layers<'de, D>(deserializer: D) -> Result<Vec<Descriptor>, D::Error>
where
  D: Deserializer<'de>,
{
  let v: Value = Deserialize::deserialize(deserializer)?;
  if v.is_array() {
    let layers = v.as_array().unwrap();
    let mut descriptors: Vec<Descriptor> = vec![];

    for layer in layers {
      if layer.is_string() {
        let layer_name = layer.as_str().unwrap();
        descriptors.push(Descriptor {
          media_type: MediaType::Layer(LayerMediaType::V1Tar),
          digest: layer_name.to_string(),
          size: 0,
          urls: None,
          annotations: None,
        });
      } else {
        return Err(Error::custom("Unrecognized layer data type, expect String"));
      }
    }

    Ok(descriptors)
  } else {
    return Err(Error::custom("Resolve layers failed"));
  }
}

impl Manifest {
  pub fn config(&self) -> &str {
    &self.config
  }
  pub fn layers_mut(&mut self) -> &mut Vec<Descriptor> {
    &mut self.layers
  } 
}
