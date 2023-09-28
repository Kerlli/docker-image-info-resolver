mod descriptor;
mod layer;
mod manifest;
mod media_type;

use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;
use std::ffi::OsStr;
use std::path::Path;
use serde::Serialize;
use tar::Archive;
use crate::{
  error::ResolverError,
  // digest::{digest, digest_io},
};
use self::layer::Layer;
use self::manifest::{Manifest, ManifestConfig};

const MANIFEST_NAME: &'static str = "manifest.json";
const LAYER_JSON_FILE_NAME: &'static str = "json";

#[derive(Serialize)]
struct ManifestMeta {
  manifest: Manifest,
  config: ManifestConfig,
}

/// Image -> []manifests -> manifest -> manifest config
#[derive(Serialize)]
pub struct Image {
    manifests: Vec<ManifestMeta>,
    layers: Vec<Layer>,
}

impl Image {
    pub fn try_from_tar(filename: &str) -> Result<Self, ResolverError> {
        let tar = File::open(filename)?;

        let mut archive = Archive::new(tar);
        let entries = archive.entries()?;

        let mut manifests: Vec<ManifestMeta> = vec![];
        let mut layers: Vec<Layer> = vec![];

        let mut archive_content_map: HashMap<String, String> = HashMap::new();
        let mut tar_size_map: HashMap<String, u64> = HashMap::new();

        for entry in entries {
          let mut f = entry?;
          let file_path = f.header().path()?;
          let file_name = file_path.clone().to_str().unwrap().to_string();
          // ignore tars
          if file_path.extension() != Some(OsStr::new("tar")) {
            let mut file_content = String::new();
            f.read_to_string(&mut file_content)?;
            archive_content_map.insert(file_name, file_content);
          } else {
            tar_size_map.insert(file_name, f.size());
          }
        }

        let manifest_json_string = archive_content_map.get(MANIFEST_NAME)
          .expect("Can not find manifest");
        let mut manifest_list: Vec<Manifest> = serde_json::from_str(&manifest_json_string)?;

        for manifest in &mut manifest_list {
          let layers = manifest.layers_mut();
          for layer in layers {
            let size = tar_size_map.get(&layer.digest);
            if size.is_some() {
              layer.size = *size.unwrap();
            }
            // todo! layer media type
          }
        }

        for manifest in manifest_list {
          let config_file_name = manifest.config();
          let config_file_json_string = archive_content_map.get(config_file_name).expect("Can not find config file");
          let config: ManifestConfig = serde_json::from_str(&config_file_json_string)?;

          manifests.push(ManifestMeta {
            manifest,
            config,
          })
        }

        let layer_json_file_name = OsStr::new(LAYER_JSON_FILE_NAME);

        for (p, content) in archive_content_map {
          let path = Path::new(&p);
          if path.file_name() == Some(layer_json_file_name) {
            let layer: Layer = serde_json::from_str(&content)?;
            layers.push(layer);
          }
        }

        sort_layers(&mut layers);

        Ok(Self {
            manifests,
            layers,
        })
    }

    pub fn layers(&self) -> &Vec<Layer> {
      &self.layers
    }

    pub fn layer_digests(&self) -> Vec<&str> {
      self.layers().iter().map(|layer| layer.id()).collect()
    }

    pub fn json(&self) -> Result<String, ResolverError> {
      Ok(serde_json::to_string(self)?)
    }
}

fn sort_layers(layers: &mut Vec<Layer>) {
  let mut sorted_layers: Vec<Layer> = vec![];

  if let Some(root) = layers.iter().find(|&layer| layer.parent().is_none()) {
      sorted_layers.push(root.clone());
      let mut parent = root.id();
      while let Some(layer) = layers.iter().find(|&l| l.parent().is_some() && l.parent().unwrap() == parent) {
          sorted_layers.push(layer.clone());
          parent = layer.id();
      }
  }

  *layers = sorted_layers;
}
