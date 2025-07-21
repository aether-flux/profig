#[warn(unused_imports)]
#[warn(dead_code)]

use quote::{quote, ToTokens};
use syn::{Data, DeriveInput, Fields, Lit};

#[derive(Debug)]
struct FieldSchema {
    name: String,
    metadata: Vec<(String, String)>,
}

pub fn expand_derive_profig(input: DeriveInput) -> proc_macro2::TokenStream {
    let name = input.ident;

    let mut schema = vec![];

    // Make sure it's a struct
    if let Data::Struct(data_struct) = input.data {
        if let Fields::Named(fields_named) = data_struct.fields {
            for field in fields_named.named.iter() {
                let field_name = field.ident.as_ref().unwrap().to_string();
                let mut metadata = vec![];

                // println!("All attrs for {}: {}", field_name, quote! { #(#field.attrs),* });
                // println!("All attrs for {}: {:#?}", field_name, field.attrs);

                for attr in &field.attrs {
                    // println!("Attr: {:?}", attr.to_token_stream());
                    // println!("Attr: {:#?}", attr);
                    if attr.path().is_ident("profig") {
                        // Parse #[profig(...)] using syn 2.0
                        let _ = attr.parse_nested_meta(|meta| {
                            // if meta.path.is_ident("doc")
                            //     || meta.path.is_ident("min")
                            //     || meta.path.is_ident("max")
                            //     || meta.path.is_ident("default")
                            //     || meta.path.is_ident("regex")
                            // {
                                if let Ok(value) = meta.value() {
                                    let lit: Lit = value.parse()?;
                                    let key = meta.path.get_ident().unwrap().to_string();

                                    let val_str = match lit {
                                        Lit::Str(s) => s.value(),
                                        Lit::Int(i) => i.base10_digits().to_string(),
                                        Lit::Bool(b) => b.value.to_string(),
                                        Lit::Float(f) => f.base10_digits().to_string(),
                                        _ => "UNKNOWN".into(),
                                    };

                                    metadata.push((key, val_str));
                                } else {
                                // Just a bare flag like #[profig(optional)]
                                metadata.push((meta.path.get_ident().unwrap().to_string(), "true".into()));
                                }

                                Ok(())
                            // }
                        });
                    }
                }

                if !metadata.is_empty() {
                    // let line = format!("Field: {field_name} -> {:#?}", metadata);
                    // schema_lines.push((field_name, metadata));
                    schema.push(FieldSchema {
                        name: field_name,
                        metadata,
                    });
                }
            }
        }
    }

    // Print metadata (for now)
    for s in schema {
        println!("{:#?}", s);
    }

    quote! {
        impl #name {
            pub fn load () -> Result<Self, Box<dyn std::error::Error>> {
                // #(#debug_prints)*

                ::profig::loader::load_from_file("config.toml")
            }
        }
    }
}

