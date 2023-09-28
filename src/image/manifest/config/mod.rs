mod exposed_port;
mod health_check;
mod history;
mod root_fs;

use serde::{Serialize, Deserialize, Deserializer, de::Error};
use serde_json::Value;

use exposed_port::ExposedPort;
use health_check::Healthcheck;
use history::History;
use root_fs::RootFs;

fn deserialize_exposed_ports<'de, D>(deserializer: D) -> Result<Option<Vec<ExposedPort>>, D::Error>
where
  D: Deserializer<'de>,
{
  let v: Value = Deserialize::deserialize(deserializer)?;

  if v.is_null() {
    Ok(None)
  } else if v.is_object() {
    let map = v.as_object().unwrap();
    // ignore map value
    let mut ports: Vec<ExposedPort> = vec![];

    for (key, _) in map {
      let port_and_protocol: Vec<&str> = key.split('/').collect();
      if port_and_protocol.len() < 2 {
        return Err(Error::custom("Unrecognized port formats"));
      }
      let port = u16::from_str_radix(port_and_protocol[0], 10);
      if port.is_err() {
        return Err(Error::custom("Unable to deserialize port number to u16"));
      }
      let protocol = port_and_protocol[1];
      match protocol {
        "tcp" => {
          ports.push(ExposedPort::Tcp(port.unwrap()));
        },
        "udp" => {
          ports.push(ExposedPort::Udp(port.unwrap()));
        },
        _ => {
          return Err(Error::custom("Unknown protocol"));
        }
      }
    }

    return Ok(Some(ports))
  } else {
    return Err(Error::custom("Unrecognized data formats"))
  }
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
  #[serde(rename(deserialize = "ExposedPorts"), deserialize_with = "deserialize_exposed_ports")]
  exposed_ports: Option<Vec<ExposedPort>>,
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
