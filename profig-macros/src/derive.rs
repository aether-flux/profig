use profig_commons::types::{FieldSchema, FieldType, MetaField};
use quote::{quote};
use syn::{Data, DeriveInput, Fields, Lit};

pub fn expand_derive_profig(input: DeriveInput) -> proc_macro2::TokenStream {
    let name = input.ident.clone();

    let mut formats = vec![];
    let mut schema = vec![];

    for attr in &input.attrs {
        if attr.path().is_ident("profig") {
            let result = attr.parse_nested_meta(|meta| {
                let key = meta
                    .path
                    .get_ident()
                    .map(|i| i.to_string())
                    .unwrap_or_default();

                if key == "format" {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Str(litstr) = value {
                        formats = litstr
                            .value()
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .collect();
                    } else {
                        return Err(syn::Error::new_spanned(
                            value,
                            "Expected string literal for format",
                        ));
                    }
                } else {
                    return Err(syn::Error::new_spanned(
                        meta.path,
                        format!("Unknown key: '{}'", key),
                    ));
                }

                Ok(())
            });

            if let Err(e) = result {
                return e.to_compile_error();
            }
        }
    }

    // Make sure it's a struct
    if let Data::Struct(data_struct) = input.data.clone() {
        if let Fields::Named(fields_named) = data_struct.fields {
            for field in fields_named.named.iter() {
                let field_name = field.ident.as_ref().unwrap().to_string();
                for attr in &field.attrs {
                    if attr.path().is_ident("profig") {
                        let mut meta_field = MetaField::default();

                        let field_type = match &field.ty {
                            syn::Type::Path(type_path) => {
                                let last_segment = type_path.path.segments.last().unwrap();
                                let ident =
                                    type_path.path.segments.last().unwrap().ident.to_string();

                                if ident == "Option" {
                                    // Option<T> Handling
                                    if let syn::PathArguments::AngleBracketed(inner_args) =
                                        &last_segment.arguments
                                    {
                                        if let Some(syn::GenericArgument::Type(syn::Type::Path(
                                            inner_ty_path,
                                        ))) = inner_args.args.first()
                                        {
                                            let inner_ident = inner_ty_path
                                                .path
                                                .segments
                                                .last()
                                                .unwrap()
                                                .ident
                                                .to_string();

                                            match inner_ident.as_str() {
                                                "String" => FieldType::Str,
                                                "bool" => FieldType::Bool,
                                                "i8" | "i16" | "i32" | "i64" | "isize" | "u8"
                                                | "u16" | "u32" | "u64" | "usize" => FieldType::Int,
                                                "f32" | "f64" => FieldType::Float,
                                                _ => panic!(
                                                    "Unsupported type inside Option<>: {}",
                                                    inner_ident
                                                ),
                                            }
                                        } else {
                                            panic!("Option<> must have a concrete type argument");
                                        }
                                    } else {
                                        panic!("Unsupported Option<> type structure");
                                    }
                                } else {
                                    match ident.as_str() {
                                        "String" => FieldType::Str,
                                        "bool" => FieldType::Bool,
                                        "i8" | "i16" | "i32" | "i64" | "isize" | "u8" | "u16"
                                        | "u32" | "u64" | "usize" => FieldType::Int,
                                        "f32" | "f64" => FieldType::Float,
                                        _ => panic!("Unsupported type: {}", ident),
                                    }
                                }
                            }
                            _ => panic!("Unsupported type structure for field: {}", &field_name),
                        };

                        // Parse #[profig(...)] using syn 2.0
                        let result = attr.parse_nested_meta(|meta| {
                            let key = meta
                                .path
                                .get_ident()
                                .map(|i| i.to_string())
                                .unwrap_or_default();

                            if let Ok(value) = meta.value() {
                                let lit: Lit = value.parse()?;

                                match (key.as_str(), lit) {
                                    ("default", Lit::Str(s)) => {
                                        meta_field.default = Some(s.value())
                                    }
                                    ("min", Lit::Int(i)) => {
                                        meta_field.min = Some(i.base10_parse::<f64>()?)
                                    }
                                    ("min", Lit::Float(f)) => {
                                        meta_field.min = Some(f.base10_parse::<f64>()?)
                                    }
                                    ("max", Lit::Int(i)) => {
                                        meta_field.max = Some(i.base10_parse::<f64>()?)
                                    }
                                    ("max", Lit::Float(f)) => {
                                        meta_field.max = Some(f.base10_parse::<f64>()?)
                                    }
                                    ("regex", Lit::Str(s)) => meta_field.regex = Some(s.value()),
                                    ("doc", Lit::Str(s)) => meta_field.doc = Some(s.value()),
                                    _ => {
                                        return Err(syn::Error::new_spanned(
                                            meta.path,
                                            format!(
                                                "Unknown key or wrong value type for '{}'",
                                                key
                                            ),
                                        ));
                                    }
                                }
                            } else {
                                return Err(syn::Error::new_spanned(
                                    meta.path,
                                    format!("Unknown flag: '{}'", key),
                                ));
                            }

                            Ok(())
                        });

                        if let Err(e) = result {
                            return e.to_compile_error();
                        }

                        schema.push(FieldSchema {
                            name: field_name.clone(),
                            ty: field_type,
                            metadata: meta_field,
                        });
                    }
                }
            }
        }
    }

    let schema_entries: Vec<_> = schema
        .iter()
        .map(|f| {
            let name = &f.name;
            let ty = match f.ty {
                FieldType::Str => quote!(::profig::types::FieldType::Str),
                FieldType::Int => quote!(::profig::types::FieldType::Int),
                FieldType::Float => quote!(::profig::types::FieldType::Float),
                FieldType::Bool => quote!(::profig::types::FieldType::Bool),
            };

            let MetaField {
                default,
                min,
                max,
                regex,
                doc,
            } = &f.metadata;

            let default = match default {
                Some(v) => quote!(Some(#v.to_string())),
                None => quote!(None),
            };

            let regex = match regex {
                Some(v) => quote!(Some(#v.to_string())),
                None => quote!(None),
            };

            let doc = match doc {
                Some(v) => quote!(Some(#v.to_string())),
                None => quote!(None),
            };

            let min = match min {
                Some(m) => quote!(Some(#m)),
                None => quote!(None),
            };

            let max = match max {
                Some(m) => quote!(Some(#m)),
                None => quote!(None),
            };

            quote! {
                ::profig::types::FieldSchema {
                    name: #name.to_string(),
                    ty: #ty,
                    metadata: ::profig::types::MetaField {
                        default: #default,
                        min: #min,
                        max: #max,
                        regex: #regex,
                        doc: #doc,
                    }
                }
            }
        })
        .collect();
    let schema_entries_doc_gen = schema_entries.clone();
    let schema_entries_sample_gen = schema_entries.clone();

    let format_branches = formats.iter().map(|fmt| {
        let ext_check = match fmt.as_str() {
            "toml" => quote! { ext == "toml" },
            "json" => quote! { ext == "json" },
            "yaml" => quote! { ext == "yaml" },
            _ => {
                return syn::Error::new_spanned(&input, format!("Unsupported format: '{}'", fmt))
                    .to_compile_error();
            }
        };

        let loader_fn = match fmt.as_str() {
            "toml" => quote! { ::profig::loader::toml::load_as_value },
            "json" => quote! { ::profig::loader::json::load_as_value },
            "yaml" | "yml" => quote! { ::profig::loader::yaml::load_as_value },
            _ => quote! {},
        };

        quote! {
            if #ext_check {
                obj = #loader_fn(path)?;
            }
        }
    });

    let struct_name = name.to_string();

    quote! {
        impl #name {
            pub fn load (path: &str) -> Result<Self, Box<dyn std::error::Error>> {
                let ext = ::std::path::Path::new(path).extension().and_then(|s| s.to_str()).unwrap_or("").to_ascii_lowercase();

                let mut obj = ::serde_json::Value::Null;

                #(#format_branches)*

                if obj.is_null() {
                    // return Err(format!("Unsupported or missing file extension: '{}'", ext).into());
                    return Err(Box::new(::profig::error::ProfigError::InvalidFormat(format!("Unsupported or missing file extension: '{}'", ext))));
                }

                let schema_vec = vec![
                    #(#schema_entries),*
                ];

                let mut json_val = ::serde_json::to_value(&obj).map_err(|e| ::profig::error::ProfigError::Parse { format: "json", error: e.to_string() })?;
                ::profig::validator::validate_fields(&mut json_val, &schema_vec)?;

                let conf = ::serde_json::from_value(json_val).map_err(|e| ::profig::error::ProfigError::Parse { format: "json", error: e.to_string() })?;

                return Ok(conf);
            }

            pub fn generate_docs (path: &str) -> Result<(), Box<dyn std::error::Error>> {
                let schema_vec = vec![
                    #(#schema_entries_doc_gen),*
                ];

                // let docContent = ::profig::generator::generate_docs(path);
                ::profig::generator::generate_doc(path, &schema_vec, #struct_name)?;

                Ok(())
            }

            pub fn sample_config (path: &str) -> Result<(), Box<dyn ::std::error::Error>> {
                let schema_vec = vec![
                    #(#schema_entries_sample_gen),*
                ];

                ::profig::generator::sample_conf(path, &schema_vec)?;

                Ok(())
            }
        }
    }
}
