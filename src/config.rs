use std::fs::File;
use std::env;
use std::collections::HashMap;
use crate::yaml_model::ConfigYml;

const DEFAULT_PORT: u16 = 80;

type Destinations = HashMap<String, String>;

#[derive(Debug, Clone)]
pub struct Config {
    pub destinations: Destinations,
    pub port: u16
}

impl Config {
    pub fn new() -> Config {
        let yml_conf = get_yml_config();
        let mut dest: Destinations = Destinations::new();
        yml_conf.services.into_iter().for_each(|s| {
            dest.insert(s.0, s.1.location);
        });
        Config {
            destinations: dest,
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