use std::fs::File;
use std::env;
use crate::yaml_model::ConfigYml;

const DEFAULT_PORT: u16 = 80;

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16
}

impl Config {
    pub fn new() -> Config {
        let yml_conf = get_yml_config();
        Config {
            port: yml_conf.http.port.unwrap_or(DEFAULT_PORT)
        }
    }
}

pub fn get_yml_config() -> ConfigYml{
    // Get command line arguments.
    let args: Vec<String> = env::args().collect();
    let cfl = &args[1]; // conf file location.
    let file = File::open(cfl).unwrap();
    let deserialized_conf: ConfigYml = serde_yaml::from_reader(&file).unwrap();
    println!("{:?}", deserialized_conf);
    deserialized_conf
}