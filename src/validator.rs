use std::{any::Any, fmt};
use std::error::Error;
use profig_commons::types::FieldType;
use serde_json::{json, Number, Value};

use crate::types::FieldSchema;

#[derive(Debug)]
struct MyErr(String);

impl fmt::Display for MyErr {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for MyErr {}

fn less_than_min (value: &Value, min: f64) -> bool {
    match value {
        Value::Number(n) => {
            if let Some(v) = n.as_f64() {
                v < min
            } else {
                false
            }
        },
        _ => false,
    }
}

fn greater_than_max (value: &Value, max: f64) -> bool {
    match value {
        Value::Number(n) => {
            if let Some(v) = n.as_f64() {
                v > max
            } else {
                false
            }
        },
        _ => false,
    }
}

pub fn validate_fields (config: &Value, schema: &[FieldSchema]) -> Result<(), Box<dyn Error>> {
    println!("Config in validator: {:?}", config);

    for s in schema {
        println!("{:#?}", s);
    }

    for f in schema {
        let meta = &f.metadata;
        if let Some(v) = config.get(&f.name) {
            match &f.ty {
                FieldType::Int => {
                    if let Some(min) = &meta.min {
                        // println!("Min = {}, Value = {}", min, v);
                        if less_than_min(&v, *min) {
                            return Err(Box::new(MyErr(format!("Value '{}' less than min: '{}'", v, &f.name))));
                        }
                    }

                    if let Some(max) = &meta.max {
                        // println!("Max = {}, Value = {}", max, v);
                        if greater_than_max(&v, *max) {
                            return Err(Box::new(MyErr(format!("Value '{}' greater than max: '{}'", v, &f.name))));
                        }
                    }
                },
                _ => {},
            }
        }
    }

    Ok(())
}
