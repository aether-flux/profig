#[warn(dead_code)]

use profig::Profig;
use serde::Deserialize;

#[derive(Profig, Deserialize, Debug)]
struct Config {
    #[profig(min = 1, max = "10")]
    threads: u32,

    #[profig(default = "localhost", required = true)]
    host: String,

    #[profig(pi = 3.14)]
    ratio: f32,

    #[profig(optional)]
    debug: bool,
}

fn main () {
    let config = Config::load().unwrap();

    println!("Config: {:#?}", config);
}
