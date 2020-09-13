use std::fs::File;
use std::{env, panic};
use std::collections::HashMap;

use yaml_model::{ConfigYml, Certificates, Tls};

mod yaml_model;
pub mod tls;

const DEFAULT_PORT: u16 = 80;
pub const DEFAULT_PORT_TLS: u16 = 443;
const DEFAULT_AUTO_TLS: bool = true;

type Destinations = HashMap<String, String>;
type Redirections = HashMap<String, String>;

#[derive(Debug, Clone)]
pub struct Config {
    pub destinations: Destinations,
    pub redirections: Redirections,
    pub port: u16,
    pub port_tls: u16,
    pub auto_tls: bool,
    pub certificates: Vec<Certificates>,
}

impl Config {
    pub fn new() -> Config {
        let yml_conf = get_yml_config();
        let mut dest: Destinations = Destinations::new();
        let mut redi: Redirections = Redirections::new();
        yml_conf.services.into_iter().for_each(|(name, ser)| {
            dest.insert(name, ser.location);
        });

        yml_conf.redirections.simple.into_iter().for_each(|(url, tar)| {
           redi.insert(url, tar);
        });

        let tls_conf = yml_conf.http.tls.unwrap_or(
            Tls { port: None, certificates: None, auto: None });

        Config {
            destinations: dest,
            redirections: redi,
            port: yml_conf.http.port.unwrap_or(DEFAULT_PORT),
            port_tls: tls_conf.port.unwrap_or(DEFAULT_PORT_TLS),
            auto_tls: tls_conf.auto.unwrap_or(DEFAULT_AUTO_TLS),
            certificates: tls_conf.certificates.unwrap_or_default(),
        }
    }
}

fn get_yml_config() -> ConfigYml {
    // Get env var.
    let cfl = env::var("ROMA_CONFIG_FILE").unwrap(); // conf file location.
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
