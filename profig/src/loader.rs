#[cfg(feature = "toml")]
pub mod toml {
    use std::error::Error;
    use std::fs::write;
    use profig_commons::error::ProfigError;

    pub fn load_as_value(path: &str) -> Result<serde_json::Value, Box<dyn Error>> {
        let content = std::fs::read_to_string(path).map_err(ProfigError::from)?;
        let parsed: toml::Value = toml::from_str(&content).map_err(|e| ProfigError::Parse { format: "toml", error: e.to_string() })?;
        let json_val = serde_json::to_value(parsed).map_err(|e| ProfigError::Parse { format: "json", error: e.to_string() })?;
        Ok(json_val)
    }

    pub fn save_sample(path: &str, val: &serde_json::Value) -> Result<(), Box<dyn Error>> {
        let toml_val: toml::Value = serde_json::from_value(val.clone()).map_err(|e| ProfigError::Parse { format: "json", error: e.to_string() })?;
        let serialized = toml::to_string_pretty(&toml_val).map_err(|e| ProfigError::Parse { format: "toml", error: e.to_string() })?;
        write(path, serialized).map_err(ProfigError::from)?;

        Ok(())
    }
}

#[cfg(feature = "json")]
pub mod json {
    use std::error::Error;
    use std::fs::write;
    use profig_commons::error::ProfigError;

    pub fn load_as_value(path: &str) -> Result<serde_json::Value, Box<dyn Error>> {
        let content = std::fs::read_to_string(path).map_err(ProfigError::from)?;
        let json_val: serde_json::Value = serde_json::from_str(&content).map_err(|e| ProfigError::Parse { format: "json", error: e.to_string() })?;
        Ok(json_val)
    }

    pub fn save_sample(path: &str, val: &serde_json::Value) -> Result<(), Box<dyn Error>> {
        let serialized = serde_json::to_string_pretty(val).map_err(|e| ProfigError::Parse { format: "json", error: e.to_string() })?;
        write(path, serialized).map_err(ProfigError::from)?;

        Ok(())
    }
}

#[cfg(feature = "yaml")]
pub mod yaml {
    use std::error::Error;
    use std::fs::write;
    use profig_commons::error::ProfigError;

    pub fn load_as_value(path: &str) -> Result<serde_json::Value, Box<dyn Error>> {
        let content = std::fs::read_to_string(path).map_err(ProfigError::from)?;
        let parsed: serde_yaml::Value = serde_yaml::from_str(&content).map_err(|e| ProfigError::Parse { format: "yaml", error: e.to_string() })?;
        let json_val = serde_json::to_value(parsed).map_err(|e| ProfigError::Parse { format: "json", error: e.to_string() })?;
        Ok(json_val)
    }

    pub fn save_sample(path: &str, val: &serde_json::Value) -> Result<(), Box<dyn Error>> {
        let yaml_val: serde_yaml::Value = serde_json::from_value(val.clone()).map_err(|e| ProfigError::Parse { format: "json", error: e.to_string() })?;
        let serialized = serde_yaml::to_string(&yaml_val).map_err(|e| ProfigError::Parse { format: "yaml", error: e.to_string() })?;
        write(path, serialized).map_err(ProfigError::from)?;

        Ok(())
    }
}

