use serde::de::DeserializeOwned;

#[cfg(feature = "toml")]
pub mod toml {
    use std::error::Error;
    use std::fs::write;

    pub fn load_as_value(path: &str) -> Result<serde_json::Value, Box<dyn Error>> {
        let content = std::fs::read_to_string(path)?;
        let parsed: toml::Value = toml::from_str(&content)?;
        let json_val = serde_json::to_value(parsed)?;
        Ok(json_val)
    }

    pub fn save_sample(path: &str, val: &serde_json::Value) -> Result<(), Box<dyn Error>> {
        let toml_val: toml::Value = serde_json::from_value(val.clone())?;
        let serialized = toml::to_string_pretty(&toml_val)?;
        write(path, serialized)?;

        Ok(())
    }
}

#[cfg(feature = "json")]
pub mod json {
    use std::error::Error;
    use std::fs::write;

    pub fn load_as_value(path: &str) -> Result<serde_json::Value, Box<dyn Error>> {
        let content = std::fs::read_to_string(path)?;
        let json_val: serde_json::Value = serde_json::from_str(&content)?;
        Ok(json_val)
    }

    pub fn save_sample(path: &str, val: &serde_json::Value) -> Result<(), Box<dyn Error>> {
        let serialized = serde_json::to_string_pretty(val)?;
        write(path, serialized)?;

        Ok(())
    }
}

#[cfg(feature = "yaml")]
pub mod yaml {
    use std::error::Error;
    use std::fs::write;

    pub fn load_as_value(path: &str) -> Result<serde_json::Value, Box<dyn Error>> {
        let content = std::fs::read_to_string(path)?;
        let parsed: serde_yaml::Value = serde_yaml::from_str(&content)?;
        let json_val = serde_json::to_value(parsed)?;
        Ok(json_val)
    }

    pub fn save_sample(path: &str, val: &serde_json::Value) -> Result<(), Box<dyn Error>> {
        let yaml_val: serde_yaml::Value = serde_json::from_value(val.clone())?;
        let serialized = serde_yaml::to_string(&yaml_val)?;
        write(path, serialized)?;

        Ok(())
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

