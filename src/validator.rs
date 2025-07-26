use std::{any::Any, fmt};
use std::error::Error;
use profig_commons::types::FieldType;
use regex::Regex;
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

fn valid_regex (value: &Value, pattern: &str) -> Result<bool, regex::Error> {
    if let Value::String(s) = value {
        let re = Regex::new(pattern)?;
        Ok(re.is_match(s))
    } else {
        Ok(false)
    }
}

pub fn validate_fields (config: &mut Value, schema: &[FieldSchema]) -> Result<(), Box<dyn Error>> {
    // for s in schema {
    //     println!("{:#?}", s);
    // }

    for f in schema {
        let meta = &f.metadata;
        match config.get(&f.name) {
            Some(v) if !v.is_null() => {
                match &f.ty {
                    FieldType::Int => {
                        if let Some(min) = &meta.min {
                            // println!("Min = {}, Value = {}", min, v);
                            if less_than_min(&v, *min) {
                                return Err(Box::new(MyErr(format!("Value '{}' less than min. Field: '{}'", v, &f.name))));
                            }
                        }

                        if let Some(max) = &meta.max {
                            // println!("Max = {}, Value = {}", max, v);
                            if greater_than_max(&v, *max) {
                                return Err(Box::new(MyErr(format!("Value '{}' greater than max. Field: '{}'", v, &f.name))));
                            }
                        }
                    },
                    FieldType::Float => {
                        if let Some(min) = &meta.min {
                            // println!("Min = {}, Value = {}", min, v);
                            if less_than_min(&v, *min) {
                                return Err(Box::new(MyErr(format!("Value '{}' less than min. Field: '{}'", v, &f.name))));
                            }
                        }

                        if let Some(max) = &meta.max {
                            // println!("Max = {}, Value = {}", max, v);
                            if greater_than_max(&v, *max) {
                                return Err(Box::new(MyErr(format!("Value '{}' greater than max. Field: '{}'", v, &f.name))));
                            }
                        }
                    },
                    FieldType::Str => {
                        if let Some(rx) = &meta.regex {
                            if !valid_regex(&v, &rx)? {
                                return Err(Box::new(MyErr(format!("Value '{}' does not match provided regex. Field: '{}'", v, &f.name))));
                            }
                        }
                    }
                    _ => {},
                }
            },

            Some(val) if val.is_null() => {
                if let Some(def) = &meta.default {
                    match &f.ty {
                        FieldType::Str => {
                            if let Some(map) = config.as_object_mut() {
                                map.insert(f.name.clone(), Value::String(def.clone()));
                            }
                        },
                        FieldType::Int => {
                            if let Ok(parsed) = def.parse() {
                                if let Some(map) = config.as_object_mut() {
                                    map.insert(f.name.clone(), Value::Number(parsed));
                                }
                            } else {
                                return Err(Box::new(MyErr(format!("Failed to parse default '{}' as integer for field '{}'", def, &f.name))));
                            }
                        },
                        FieldType::Float => {
                            if let Ok(parsed) = def.parse() {
                                if let Some(map) = config.as_object_mut() {
                                    map.insert(f.name.clone(), Value::Number(parsed));
                                }
                            } else {
                                return Err(Box::new(MyErr(format!("Failed to parse default '{}' as float for field '{}'", def, &f.name))));
                            }
                        },
                        FieldType::Bool => {
                            if let Ok(parsed) = def.parse::<bool>() {
                                if let Some(map) = config.as_object_mut() {
                                    map.insert(f.name.clone(), Value::Bool(parsed));
                                }
                            } else {
                                return Err(Box::new(MyErr(format!("Failed to parse default '{}' as bool for field '{}'", def, &f.name))));
                            }
                        },
                        _ => {},
                    }
                }
            },

            None => {
                if let Some(def) = &meta.default {
                    match &f.ty {
                        FieldType::Str => {
                            if let Some(map) = config.as_object_mut() {
                                map.insert(f.name.clone(), Value::String(def.clone()));
                            }
                        },
                        FieldType::Int => {
                            if let Ok(parsed) = def.parse() {
                                if let Some(map) = config.as_object_mut() {
                                    map.insert(f.name.clone(), Value::Number(parsed));
                                }
                            } else {
                                return Err(Box::new(MyErr(format!("Failed to parse default '{}' as integer for field '{}'", def, &f.name))));
                            }
                        },
                        FieldType::Float => {
                            if let Ok(parsed) = def.parse() {
                                if let Some(map) = config.as_object_mut() {
                                    map.insert(f.name.clone(), Value::Number(parsed));
                                }
                            } else {
                                return Err(Box::new(MyErr(format!("Failed to parse default '{}' as float for field '{}'", def, &f.name))));
                            }
                        },
                        FieldType::Bool => {
                            if let Ok(parsed) = def.parse::<bool>() {
                                if let Some(map) = config.as_object_mut() {
                                    map.insert(f.name.clone(), Value::Bool(parsed));
                                }
                            } else {
                                return Err(Box::new(MyErr(format!("Failed to parse default '{}' as bool for field '{}'", def, &f.name))));
                            }
                        },
                        _ => {},
                    }
                }
            },

            _ => {},
        }
    }

    Ok(())
}
