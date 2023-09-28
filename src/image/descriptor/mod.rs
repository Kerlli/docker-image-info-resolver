use std::collections::HashMap;
use serde::Serialize;

use super::media_type::MediaType;

#[derive(Debug, Serialize, Clone)]
pub struct Descriptor {
  pub media_type: MediaType,
  pub digest: String,
  pub size: u64,
  pub urls: Option<Vec<String>>,
  /// Annotations Rules
  /// * Annotations MUST be a key-value map where both the key and value MUST be strings.
  /// * While the value MUST be present, it MAY be an empty string.
  /// * Keys MUST be unique within this map, and best practice is to namespace the keys.
  /// * Keys SHOULD be named using a reverse domain notation - e.g. com.example.myKey.
  /// * The prefix org.opencontainers is reserved for keys defined in Open Container Initiative (OCI) specifications and MUST NOT be used by other specifications and extensions.
  /// * Keys using the org.opencontainers.image namespace are reserved for use in the OCI Image Specification and MUST NOT be used by other specifications and extensions, including other OCI specifications.
  /// * If there are no annotations then this property MUST either be absent or be an empty map.
  /// * Consumers MUST NOT generate an error if they encounter an unknown annotation key.
  pub annotations: Option<HashMap<String, String>>,
}
