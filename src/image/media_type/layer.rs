#[derive(Debug, Clone)]
pub enum LayerMediaType {
  V1Tar,
  V1Tgz,
  NonDistV1Tar,
  NonDistV1Tgz,
}

impl From<LayerMediaType> for String {
  fn from(value: LayerMediaType) -> Self {
    match value {
      LayerMediaType::V1Tar => "application/vnd.oci.image.layer.v1.tar".to_owned(),
      LayerMediaType::V1Tgz => "application/vnd.oci.image.layer.v1.tar+gzip".to_owned(),
      LayerMediaType::NonDistV1Tar => "application/vnd.oci.image.layer.nondistributable.v1.tar".to_owned(),
      LayerMediaType::NonDistV1Tgz => "application/vnd.oci.image.layer.nondistributable.v1.tar+gzip".to_owned(),
    }
  }
}
