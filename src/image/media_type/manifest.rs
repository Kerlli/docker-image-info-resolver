#[derive(Debug, Clone)]
pub enum ManifestMediaType {
  // application/vnd.oci.image.manifest.v1+json
  V1,
}

impl From<ManifestMediaType> for String {
  fn from(value: ManifestMediaType) -> Self {
    match value {
      ManifestMediaType::V1 => "application/vnd.oci.image.manifest.v1+json".to_owned(),
    }
  }
}
