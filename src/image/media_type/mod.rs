mod config;
mod layer;
mod manifest;

use serde::{Serialize, Serializer};

pub use self::config::ConfigMediaType;
pub use self::layer::LayerMediaType;
pub use self::manifest::ManifestMediaType;

#[derive(Debug, Clone)]
pub enum MediaType {
  Config(ConfigMediaType),
  Layer(LayerMediaType),
  Manifest(ManifestMediaType),
}

impl Serialize for MediaType {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    match self {
      Self::Config(mt) => serializer.serialize_newtype_struct("media_type", &String::from(mt.clone())),
      Self::Layer(mt) => serializer.serialize_newtype_struct("media_type", &String::from(mt.clone())),
      Self::Manifest(mt) => serializer.serialize_newtype_struct("media_type", &String::from(mt.clone())),
    }
  }
}

