use profig_commons::types::{FieldSchema, FieldType};

pub fn generate_doc (path: &str, schema: &[FieldSchema], name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // for s in schema {
    //     println!("{:#?}", s);
    // }

    // let mut content = "# Config File\n".to_string();
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

    std::fs::write(path, content)?;

    Ok(())
}

pub fn sample_conf (path: &str, schema: &[FieldSchema]) -> Result<(), Box<dyn std::error::Error>> {
    for s in schema {
        println!("{:#?}", s);
    }

    let mut map = serde_json::Map::new();

    for f in schema {
        let meta = &f.metadata;

        let value = match &f.ty {
            FieldType::Str => {
                if let Some(def) = &meta.default {
                    serde_json::Value::String(def.clone())
                } else {
                    serde_json::Value::String("REQUIRED".to_string())
                }
            },
            FieldType::Int => {
                if let Some(def) = &meta.default {
                    serde_json::Value::Number(def.parse().unwrap_or(serde_json::Number::from(0)))
                } else {
                    serde_json::Value::Number(serde_json::Number::from(0))
                }
            },
            FieldType::Float => {
                if let Some(def) = &meta.default {
                    serde_json::Value::Number(serde_json::Number::from_f64(def.parse::<f64>().unwrap_or(0.0)).unwrap())
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

    println!("Sample: {:#?}", val);

    Ok(())
}
