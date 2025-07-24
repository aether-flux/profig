use serde::de::DeserializeOwned;

use crate::validator::validate_fields;

pub fn load_from_file<T: DeserializeOwned>(path: &str) -> Result<T, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let parsed = toml::from_str(&content)?;

    Ok(parsed)
}
