use serde::{Serialize, Serializer};

#[derive(Debug, Clone)]
pub enum ExposedPort {
  Tcp(u16),
  Udp(u16),
}

impl Serialize for ExposedPort {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    match self {
      Self::Tcp(port) => serializer.serialize_newtype_variant("exposed_port", 0, "tcp", port),
      Self::Udp(port) => serializer.serialize_newtype_variant("exposed_port", 1, "udp", port),
    }
  }
}
