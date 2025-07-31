use profig_commons::{error::ProfigError, types::{FieldSchema, FieldType}};

pub fn generate_doc (path: &str, schema: &[FieldSchema], name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // for s in schema {
    //     println!("{:#?}", s);
    // }

    let mut content = format!("# Config - {}\n", name).to_string();

    for f in schema {
        let meta = &f.metadata;

        if let Some(d) = &meta.doc {
            // let content = format!("{}\n{}", content, d.as_str()).as_str();
            let heading = format!("## Field: *{}* - `{:?}`", &f.name, &f.ty);
            let body = format!("{}\n{}", heading, d);
            content = content.clone().to_owned() + "\n\n" + body.as_str();
        }
    }

    std::fs::write(path, content).map_err(ProfigError::from)?;
    println!("\nConfig documentation created at {}.", path);

    Ok(())
}

pub fn sample_conf (path: &str, schema: &[FieldSchema]) -> Result<(), Box<dyn std::error::Error>> {
    let mut map = serde_json::Map::new();

    for f in schema {
        let meta = &f.metadata;

        let value = match &f.ty {
            FieldType::Str => {
                if let Some(def) = &meta.default {
                    serde_json::Value::String(def.clone())
                } else {
                    if let Some(r) = &meta.regex {
                        serde_json::Value::String(format!("REQUIRED; must match {}", r).to_string())
                    } else {
                        serde_json::Value::String("REQUIRED".to_string())
                    }
                }
            },
            FieldType::Int => {
                if let Some(def) = &meta.default {
                    serde_json::Value::Number(def.parse().unwrap_or(serde_json::Number::from(0)))
                } else if let Some(min) = &meta.min {
                    serde_json::Value::Number(serde_json::Number::from(min.to_owned() as i64))
                } else {
                    serde_json::Value::Number(serde_json::Number::from(0))
                }
            },
            FieldType::Float => {
                if let Some(def) = &meta.default {
                    serde_json::Value::Number(serde_json::Number::from_f64(def.parse::<f64>().unwrap_or(0.0)).unwrap())
                } else if let Some(min) = &meta.min {
                    serde_json::Value::Number(serde_json::Number::from_f64(min.to_owned()).unwrap_or(serde_json::Number::from(0)))
                } else {
                    serde_json::Value::Number(serde_json::Number::from_f64(0.0).unwrap())
                }
            },
            FieldType::Bool => {
                if let Some(def) = &meta.default {
                    serde_json::Value::Bool(def == "true")
                } else {
                    serde_json::Value::Bool(false)
                }
            },
            
        };

        map.insert(f.name.clone(), value);
    }

    let val = serde_json::Value::Object(map);

    // Auto-detect config file type
    let ext = std::path::Path::new(path)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    #[cfg(feature = "json")]
    if ext == "json" {
        crate::loader::json::save_sample(path, &val)?;
        println!("\nSample config created at {}.", path);
        return Ok(());
    }

    #[cfg(feature = "toml")]
    if ext == "toml" {
        crate::loader::toml::save_sample(path, &val)?;
        println!("\nSample config created at {}.", path);
        return Ok(());
    }

    #[cfg(feature = "yaml")]
    if ext == "yaml" || ext == "yml" {
        crate::loader::yaml::save_sample(path, &val)?;
        println!("\nSample config created at {}.", path);
        return Ok(());
    }

    return Err(Box::new(ProfigError::InvalidFormat(format!("Unsupported or missing file extension: '{}'", ext))));
    // Ok(())
}
