use profig::Profig;
use serde::{Deserialize, Serialize};

#[derive(Profig, Serialize, Deserialize, Debug)]
struct Config {
    #[profig(min = 4, max = 10)]
    threads: f32,

    #[profig(default = "localhost")]
    host: Option<String>,

    #[profig(default = "5000")]
    port: Option<u64>,

    #[profig(default = "3.1415")]
    pi: Option<f32>,

    #[profig(regex = r"^[\w\.-]+@[\w\.-]+\.\w+$")]
    email: String,

    #[profig()]
    debug: Option<bool>,
}

fn main () -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load()?;

    println!("Config: {:#?}", &config);
    println!("Config host: {:?}", &config.host);

    Ok(())
}
