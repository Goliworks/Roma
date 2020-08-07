use std::fs::File;
use std::env;
use std::collections::HashMap;
use rustls::{NoClientAuth, ServerConfig};
use rustls::ResolvesServerCertUsingSNI;
use rustls::internal::pemfile::{certs, rsa_private_keys};
use rustls::sign::{SigningKey, RSASigningKey};

use crate::yaml_model::{ConfigYml, Certificates, Tls};
use std::sync::Arc;
use std::io::{BufReader, BufRead};

use x509_parser::parse_x509_der;
use x509_parser::pem::pem_to_der;

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

pub fn get_tls_config(certs: &Vec<Certificates>) -> ServerConfig {
    let mut resolver = ResolvesServerCertUsingSNI::new();
    let mut config_tls = ServerConfig::new(NoClientAuth::new());

    certs.into_iter().for_each(|c| {
        add_certificate_to_resolver(c, &mut resolver);
    });

    config_tls.cert_resolver = Arc::new(resolver);
    config_tls
}

fn add_certificate_to_resolver(
    cert: &Certificates,
    resolver: &mut ResolvesServerCertUsingSNI,
) {
    let br_cert = &mut BufReader::new(File::open(
        &cert.cert
    ).unwrap());
    let br_key = &mut BufReader::new(File::open(
        &cert.key
    ).unwrap());

    let buffer = br_cert.fill_buf().unwrap();
    let cn = get_common_name(buffer);

    let cert_chain = certs(br_cert).unwrap();
    let mut keys = rsa_private_keys(br_key).unwrap();
    let signing_key = RSASigningKey::new(
        &keys.remove(0)
    ).unwrap();
    let signing_key_boxed: Arc<Box<dyn SigningKey>> = Arc::new(
        Box::new(signing_key)
    );

    resolver.add(cn.as_str(), rustls::sign::CertifiedKey::new(
        cert_chain, signing_key_boxed,
    )).expect("Invalid certificate");
}

fn get_common_name(buffer: &[u8]) -> String {
    let res = pem_to_der(&buffer);

    let subject = match res {
        Ok((_rem, pem)) => {
            let res_x509 = parse_x509_der(&pem.contents);
            match res_x509 {
                Ok((_rem, cert)) => {
                    cert.tbs_certificate.subject.to_string()
                }
                _ => panic!("x509 parsing failed: {:?}", res_x509),
            }
        }
        _ => panic!("PEM parsing failed: {:?}", res),
    };
    let cn: Vec<&str> = subject.split("CN=").collect();
    cn[1].to_string()
}

fn get_yml_config() -> ConfigYml {
    // Get command line arguments.
    let args: Vec<String> = env::args().collect();
    let cfl = &args[1]; // conf file location.
    let file = File::open(cfl).unwrap();
    let deserialized_conf: ConfigYml = serde_yaml::from_reader(&file).unwrap();
    println!("{:?}", deserialized_conf);
    deserialized_conf
}
