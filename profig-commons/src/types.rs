#[derive(Debug)]
pub enum FieldType {
    Int,
    Float,
    Str,
    Bool,
}

#[derive(Debug, Default)]
pub struct MetaField {
    pub default: Option<String>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub regex: Option<String>,
    pub doc: Option<String>,
}

#[derive(Debug)]
pub struct FieldSchema {
    pub name: String,
    pub ty: FieldType,
    pub metadata: MetaField,
}

