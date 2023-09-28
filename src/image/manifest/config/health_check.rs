use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Healthcheck {
  #[serde(rename(deserialize = "Test"))]
  test: Vec<String>,
  #[serde(rename(deserialize = "Interval"))]
  interval: u32,
  #[serde(rename(deserialize = "Timeout"))]
  timeout: u32,
  #[serde(rename(deserialize = "Retires"))]
  retires: u32,
}
