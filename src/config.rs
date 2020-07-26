use std::fs::File;
use std::env;
use std::collections::HashMap;
use rustls::{NoClientAuth, ServerConfig};
use rustls::ResolvesServerCertUsingSNI;

use crate::yaml_model::{ConfigYml, Certificates};
use std::sync::Arc;
use std::io::Read;

use x509_parser::parse_x509_der;
use x509_parser::pem::pem_to_der;

const DEFAULT_PORT: u16 = 80;
const DEFAULT_PORT_TLS: u16 = 443;

type Destinations = HashMap<String, String>;

#[derive(Debug, Clone)]
pub struct Config {
    pub destinations: Destinations,
    pub port: u16,
    pub port_tls: u16
}

impl Config {
    pub fn new() -> Config {
        let yml_conf = get_yml_config();
        let mut dest: Destinations = Destinations::new();
        yml_conf.services.into_iter().for_each(|s| {
            dest.insert(s.0, s.1.location);
        });

        let tls_config = get_tls_config(&yml_conf.http.tls.certificates);

        Config {
            destinations: dest,
            port: yml_conf.http.port.unwrap_or(DEFAULT_PORT),
            port_tls: yml_conf.http.tls.port.unwrap_or(DEFAULT_PORT_TLS)
        }
    }
}

fn get_tls_config(certs: &Vec<Certificates>) -> ServerConfig {
    let mut resolver = ResolvesServerCertUsingSNI::new();
    let mut config_tls = ServerConfig::new(NoClientAuth::new());

    certs.into_iter().for_each(|c| {
        add_certificate_to_resolver(c);
    });

    config_tls.cert_resolver = Arc::new(resolver);
    config_tls
}

fn add_certificate_to_resolver(cert: &Certificates){
    let mut cert_file = File::open( &cert.cert).unwrap();
    let cn = get_common_name(&mut cert_file);
    println!("{}", cn);
}

fn get_common_name(cert: &mut File) -> String {
    let mut buffer:Vec<u8> = Vec::new();
    cert.read_to_end(&mut buffer).unwrap();
    let res = pem_to_der(&buffer);

    let subject = match res {
        Ok((_rem, pem)) => {
            let res_x509 = parse_x509_der(&pem.contents);
            match res_x509 {
                Ok((_rem, cert)) => {
                    cert.tbs_certificate.subject.to_string()
                },
                _ => panic!("x509 parsing failed: {:?}", res_x509),
            }
        },
        _ => panic!("PEM parsing failed: {:?}", res),
    };
    let cn: Vec<&str> = subject.split("CN=").collect();
    cn[1].to_string()
}

fn get_yml_config() -> ConfigYml{
    // Get command line arguments.
    let args: Vec<String> = env::args().collect();
    let cfl = &args[1]; // conf file location.
    let file = File::open(cfl).unwrap();
    let deserialized_conf: ConfigYml = serde_yaml::from_reader(&file).unwrap();
    println!("{:?}", deserialized_conf);
    deserialized_conf
}