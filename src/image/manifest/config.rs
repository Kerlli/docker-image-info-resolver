use serde::{Serialize,Deserialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Healthcheck {
  #[serde(rename(deserialize = "Test"))]
  test: Vec<String>,
  #[serde(rename(deserialize = "Interval"))]
  interval: u32,
  #[serde(rename(deserialize = "Timeout"))]
  timeout: u32,
  #[serde(rename(deserialize = "Retires"))]
  retires: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Config {
  #[serde(rename(deserialize = "ArgsEscaped"))]
  args_escaped: Option<bool>,
  #[serde(rename(deserialize = "Cmd"))]
  cmd: Option<Vec<String>>,
  #[serde(rename(deserialize = "CpuShares"))]
  cpu_shares: Option<u8>,
  #[serde(rename(deserialize = "Entrypoint"))]
  entry_point: Vec<String>,
  #[serde(rename(deserialize = "Env"))]
  env: Vec<String>,
  #[serde(rename(deserialize = "ExposedPorts"))]
  exposed_ports: Value,
  #[serde(rename(deserialize = "Healthcheck"))]
  healthcheck: Option<Healthcheck>,
  #[serde(rename(deserialize = "Labels"))]
  labels: Value,
  #[serde(rename(deserialize = "OnBuild"))]
  on_build: Option<Vec<String>>,
  #[serde(rename(deserialize = "Memory"))]
  memory: Option<u64>,
  #[serde(rename(deserialize = "MemorySwap"))]
  memory_swap: Option<u64>,
  #[serde(rename(deserialize = "Volumes"))]
  volumes: Option<Value>,
  #[serde(rename(deserialize = "Shell"))]
  shell: Option<Vec<String>>,
  #[serde(rename(deserialize = "WorkingDir"))]
  working_dir: String,
  #[serde(rename(deserialize = "User"))]
  user: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct History {
  author: Option<String>,
  comment: Option<String>,
  created: String,
  created_by: String,
  empty_layer: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct RootFs {
  #[serde(rename = "type")]
  fs_type: String,
  diff_ids: Vec<String>,
}

impl RootFs {
  pub fn diff_ids(&self) -> &Vec<String> {
    &self.diff_ids
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ManifestConfig {
  architecture: String,
  author: Option<String>,
  config: Config,
  created: String,
  history: Vec<History>,
  os: String,
  rootfs: RootFs,
}

impl ManifestConfig {
  pub fn arch(&self) -> &str {
    &self.architecture
  }

  pub fn os(&self) -> &str {
    &self.os
  }

  pub fn diff_ids(&self) -> &Vec<String> {
    self.rootfs.diff_ids()
  }
}
