#[derive(Debug, Clone)]
pub enum ConfigMediaType {
  V1,
}

impl From<ConfigMediaType> for String {
  fn from(value: ConfigMediaType) -> Self {
    match value {
      ConfigMediaType::V1 => "application/vnd.oci.image.config.v1+json".to_owned(),
    }
  }
}
