#[derive(Debug)]
pub enum FieldType {
    Int,
    Float,
    Str,
    Bool,
}

#[derive(Debug, Default)]
pub struct MetaField {
    // pub ty: FieldType,
    pub default: Option<String>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub regex: Option<String>,
    pub optional: bool,
}

#[derive(Debug)]
pub struct FieldSchema {
    pub name: String,
    pub ty: FieldType,
    pub metadata: MetaField,
}

