use serde::de::DeserializeOwned;

#[cfg(feature = "toml")]
pub mod toml {
    use std::error::Error;
    pub fn load_as_value(path: &str) -> Result<serde_json::Value, Box<dyn Error>> {
        let content = std::fs::read_to_string(path)?;
        let parsed: toml::Value = toml::from_str(&content)?;
        let json_val = serde_json::to_value(parsed)?;
        Ok(json_val)
    }
}

#[cfg(feature = "json")]
pub mod json {
    use std::error::Error;
    pub fn load_as_value(path: &str) -> Result<serde_json::Value, Box<dyn Error>> {
        let content = std::fs::read_to_string(path)?;
        let json_val: serde_json::Value = serde_json::from_str(&content)?;
        Ok(json_val)
    }
}

#[cfg(feature = "yaml")]
pub mod yaml {
    use std::error::Error;
    pub fn load_as_value(path: &str) -> Result<serde_json::Value, Box<dyn Error>> {
        let content = std::fs::read_to_string(path)?;
        let parsed: serde_yaml::Value = serde_yaml::from_str(&content)?;
        let json_val = serde_json::to_value(parsed)?;
        Ok(json_val)
    }
}

// pub fn load_from_file<T: DeserializeOwned>(path: &str) -> Result<T, Box<dyn std::error::Error>> {
//     let content = std::fs::read_to_string(path)?;
//     let parsed = toml::from_str(&content)?;
//
//     Ok(parsed)
// }

// pub fn load_as_value(path: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
//     let content = std::fs::read_to_string(path)?;
//     let parsed: toml::Value = toml::from_str(&content)?;
//     let json_val = serde_json::to_value(parsed)?;
//
//     Ok(json_val)
// }

