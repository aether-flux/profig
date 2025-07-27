use profig_commons::types::FieldSchema;

pub fn genrate_doc (path: &str, schema: &[FieldSchema]) -> Result<(), Box<dyn std::error::Error>> {
    for s in schema {
        println!("{:#?}", s);
    }

    let mut content = "# Config File\n".to_string();

    for f in schema {
        let meta = &f.metadata;

        if let Some(d) = &meta.doc {
            // let content = format!("{}\n{}", content, d.as_str()).as_str();
            let heading = format!("## Field: `{}` - {:?}", &f.name, &f.ty);
            let body = format!("{}\n{}", heading, d);
            content = content.clone().to_owned() + "\n\n" + body.as_str();
        }
    }

    println!("Content Doc:\n{}", content);

    Ok(())
}
