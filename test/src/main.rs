use profig::Profig;
use serde::{Deserialize, Serialize};

#[derive(Profig, Serialize, Deserialize, Debug)]
#[profig(format="toml, json")]
struct Config {
    #[profig(min = 4, max = 10)]
    threads: i32,

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
    let json_conf = Config::load("config.json")?;
    let toml_conf = Config::load("config.toml")?;
    let yaml_conf = Config::load("config.yaml")?;

    println!("Config (JSON): {:#?}", &json_conf);
    println!("Config (TOML): {:#?}", &toml_conf);
    println!("Config (YAML): {:#?}", &yaml_conf);

    Ok(())
}
