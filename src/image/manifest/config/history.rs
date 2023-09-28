use serde::{Serialize, Deserialize, Deserializer, de::Error};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct History {
  author: Option<String>,
  comment: Option<String>,
  created: String,
  #[serde(deserialize_with = "shrink_spaces")]
  created_by: String,
  empty_layer: Option<bool>,
}

fn shrink_spaces<'de, D>(deserializer: D) -> Result<String, D::Error>
where
  D: Deserializer<'de>,
{
  let v: Value = Deserialize::deserialize(deserializer)?;
  if v.is_string() {
    if let Some(created_by) = v.as_str() {
      let mut s: Vec<char> = vec![];
      let mut meet_space = false;
      for c in created_by.chars() {
        if c == ' ' && !meet_space {
          s.push(c);
          meet_space = true;
        } else if c != ' ' {
          s.push(c);
          meet_space = false;
        }
      }
      Ok(s.iter().collect())
    } else {
      return Err(Error::custom("Unable to convert data to `&str`"));
    }
  } else {
    return Err(Error::custom("Unable to deserialize `created_by`, except data type: String"));
  }
}
