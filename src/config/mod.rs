use std::fs::File;
use std::{env, panic};
use std::collections::HashMap;

use yaml_model::{ConfigYml, Certificates, Tls};

mod yaml_model;
pub mod tls;

const DEFAULT_PORT: u16 = 80;
const DEFAULT_PORT_TLS: u16 = 443;

type Destinations = HashMap<String, String>;

#[derive(Debug, Clone)]
pub struct Config {
    pub destinations: Destinations,
    pub port: u16,
    pub port_tls: u16,
    pub certificates: Vec<Certificates>,
}

impl Config {
    pub fn new() -> Config {
        let yml_conf = get_yml_config();
        let mut dest: Destinations = Destinations::new();
        yml_conf.services.into_iter().for_each(|s| {
            dest.insert(s.0, s.1.location);
        });

        let tls_conf = yml_conf.http.tls.unwrap_or(Tls { port: None, certificates: None });

        Config {
            destinations: dest,
            port: yml_conf.http.port.unwrap_or(DEFAULT_PORT),
            port_tls: tls_conf.port.unwrap_or(DEFAULT_PORT_TLS),
            certificates: tls_conf.certificates.unwrap_or_default(),
        }
    }
}

fn get_yml_config() -> ConfigYml {
    // Get command line arguments.
    let args: Vec<String> = env::args().collect();
    let cfl = &args[1]; // conf file location.
    let file = File::open(cfl).unwrap();
    let deserialized_conf: ConfigYml = serde_yaml::from_reader(&file)
        .unwrap_or_else(|_| {
            panic::set_hook(Box::new(|_| {
                println!("Error : Invalid configuration file.\nCheck your YAML structure.");
            }));
            panic!();
        });
    println!("{:?}", deserialized_conf);
    deserialized_conf
}
