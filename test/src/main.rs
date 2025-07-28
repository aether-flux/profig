use profig::Profig;
use serde::{Deserialize, Serialize};

#[derive(Profig, Serialize, Deserialize, Debug)]
#[profig(format="toml")]
struct Config {
    #[profig(min = 4, max = 10, doc = "Number of threads")]
    threads: i32,

    #[profig(default = "localhost", doc = "Host name")]
    host: Option<String>,

    #[profig(default = "5000", doc = "Port number")]
    port: Option<u64>,

    #[profig(default = "3.1415", doc = "Pi value")]
    pi: Option<f32>,

    #[profig(regex = r"^[\w\.-]+@[\w\.-]+\.\w+$", doc = "Email address")]
    email: String,

    #[profig(doc = "Debug option")]
    debug: Option<bool>,
}

fn main () -> Result<(), Box<dyn std::error::Error>> {
    // let json_conf = Config::load("config.json")?;
    let toml_conf = Config::load("config.toml")?;
    // let yaml_conf = Config::load("config.yaml")?;

    // println!("Config (JSON): {:#?}", &json_conf);
    println!("Config (TOML): {:#?}", &toml_conf);
    // println!("Config (YAML): {:#?}", &yaml_conf);

    // Config::generate_docs("config.md")?;
    Config::sample_config("sample.toml")?;

    Ok(())
}
