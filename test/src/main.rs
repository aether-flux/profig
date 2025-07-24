use profig::Profig;
use serde::{Deserialize, Serialize};
// use profig::types::FieldSchema;

#[derive(Profig, Serialize, Deserialize, Debug)]
struct Config {
    #[profig(min = 4, max = 10)]
    threads: u32,

    #[profig(default = "localhost")]
    host: String,

    #[profig()]
    debug: bool,
}

fn main () -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load()?;

    println!("Config: {:#?}", config);

    Ok(())
}
