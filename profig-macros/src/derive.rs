use profig_commons::types::{FieldType, FieldSchema, MetaField};
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput, Fields, Lit};

pub fn expand_derive_profig(input: DeriveInput) -> proc_macro2::TokenStream {
    let name = input.ident;

    let mut schema = vec![];

    // Make sure it's a struct
    if let Data::Struct(data_struct) = input.data {
        if let Fields::Named(fields_named) = data_struct.fields {
            for field in fields_named.named.iter() {
                let field_name = field.ident.as_ref().unwrap().to_string();
                for attr in &field.attrs {
                    if attr.path().is_ident("profig") {
                        let mut meta_field = MetaField::default();
                        let ty_string = quote!(#field.ty).to_string();

                        let field_type = match &field.ty {
                            syn::Type::Path(type_path) => {
                                let ident = type_path.path.segments.last().unwrap().ident.to_string();

                                match ident.as_str() {
                                    "String" => FieldType::Str,
                                    "bool" => FieldType::Bool,
                                    "i8" | "i16" | "i32" | "i64" | "isize" |
                                    "u8" | "u16" | "u32" | "u64" | "usize" => FieldType::Int,
                                    "f32" | "f64" => FieldType::Float,
                                    _ => panic!("Unsupported type: {}", ident),
                                }
                            },
                            _ => panic!("Unsupported type structure for field: {}", &field_name),
                        };

                        // Parse #[profig(...)] using syn 2.0
                        let result = attr.parse_nested_meta(|meta| {
                            let key = meta.path.get_ident().map(|i| i.to_string()).unwrap_or_default();

                            if let Ok(value) = meta.value() {
                                let lit: Lit = value.parse()?;

                                // let val_str = match lit {
                                //     Lit::Str(s) => s.value(),
                                //     Lit::Int(i) => i.base10_digits().to_string(),
                                //     Lit::Bool(b) => b.value.to_string(),
                                //     Lit::Float(f) => f.base10_digits().to_string(),
                                //     _ => "UNKNOWN".into(),
                                // };

                                match (key.as_str(), lit) {
                                    ("default", Lit::Str(s)) => meta_field.default = Some(s.value()),
                                    ("min", Lit::Int(i)) => meta_field.min = Some(i.base10_parse::<f64>()?),
                                    ("min", Lit::Float(f)) => meta_field.min = Some(f.base10_parse::<f64>()?),
                                    ("max", Lit::Int(i)) => meta_field.max = Some(i.base10_parse::<f64>()?),
                                    ("max", Lit::Float(f)) => meta_field.max = Some(f.base10_parse::<f64>()?),
                                    ("regex", Lit::Str(s)) => meta_field.regex = Some(s.value()),
                                    _ => {
                                        return Err(syn::Error::new_spanned(meta.path, format!("Unknown key or wrong value type for '{}'", key)));
                                    }
                                }

                                // metadata.push((key, val_str));
                            } else {
                                return Err(syn::Error::new_spanned(meta.path, format!("Unknown flag: '{}'", key)));
                            }

                            // metadata.push(field_type);

                            // schema.push(FieldSchema {
                            //     name: field_name,
                            //     ty: field_type,
                            //     metadata: meta_field,
                            // });

                            Ok(())
                        });

                        if let Err(e) = result {
                            return e.to_compile_error().into();
                        }

                        schema.push(FieldSchema {
                            name: field_name.clone(),
                            ty: field_type,
                            metadata: meta_field,
                        });
                    }
                }

                // if !metadata.is_empty() {
                    // let line = format!("Field: {field_name} -> {:#?}", metadata);
                    // schema_lines.push((field_name, metadata));
                    // schema.push(FieldSchema {
                    //     name: field_name,
                    //     ty: field_type,
                    //     metadata: meta_field,
                    // });
                // }
            }
        }
    }

    // Print metadata (for now)
    // for s in &schema {
    //     println!("{:#?}", s);
    // }

    let schema_entries = schema.iter().map(|f| {
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
        } = &f.metadata;

        let default = match default {
            Some(v) => quote!(Some(#v.to_string())),
            None => quote!(None),
        };

        let regex = match regex {
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
                }
            }
        }
    });

    quote! {
        impl #name {
            pub fn load () -> Result<Self, Box<dyn std::error::Error>> {
                let obj = ::profig::loader::load_from_file("config.toml")?;
                
                let schema_vec = vec![
                    #(#schema_entries),*
                ];

                let json_val = ::serde_json::to_value(&obj)?;
                ::profig::validator::validate_fields(&json_val, &schema_vec)?;

                return Ok(obj);
            }
        }
    }
}

