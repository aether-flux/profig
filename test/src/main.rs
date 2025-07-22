use profig::Profig;
use serde::Deserialize;

#[derive(Profig, Deserialize, Debug)]
struct Config {
    #[profig(min = 1, max = 10)]
    threads: u32,

    #[profig(default = "localhost")]
    host: String,

    #[profig(optional)]
    debug: bool,
}

fn main () {
    let config = Config::load().unwrap();

    println!("Config: {:#?}", config);
}
