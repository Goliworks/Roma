use crate::config::yaml_model::Certificates;
use crate::utils;
use rustls::{NoClientAuth, ServerConfig, ResolvesServerCertUsingSNI};
use std::io::{BufReader, BufRead};
use std::fs::File;
use rustls::internal::pemfile::{certs, rsa_private_keys};
use rustls::sign::{RSASigningKey, SigningKey};
use std::sync::Arc;
use x509_parser::pem::pem_to_der;
use x509_parser::{parse_x509_der, X509Certificate};
use x509_parser::extensions::{ParsedExtension, GeneralName};

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
        utils::resolve_path(cert.cert.clone())
    ).unwrap());
    let br_key = &mut BufReader::new(File::open(
        utils::resolve_path(cert.key.clone())
    ).unwrap());

    let buffer = br_cert.fill_buf().unwrap();
    let cn = get_domain(buffer);

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

fn get_domain(buffer: &[u8]) -> String {
    let res = pem_to_der(&buffer);

    let cn = match res {
        Ok((_, pem)) => {
            let x509 = parse_x509_der(&pem.contents);
            match x509 {
                Ok((_, cert)) => {
                    get_san(&cert);
                    get_common_name(&cert)
                }
                _ => panic!("x509 parsing failed: {:?}", x509),
            }
        }
        _ => panic!("PEM parsing failed: {:?}", res),
    };
    cn

}

fn get_common_name(cert: &X509Certificate) -> String {
    let subject= cert.tbs_certificate.subject.to_string();
    let cn: Vec<&str> = subject.split("CN=").collect();
    cn[1].to_string()
}

fn get_san(cert: &X509Certificate) {
    for (a, b) in cert.extensions() {
        match b.parsed_extension() {
            ParsedExtension::SubjectAlternativeName(san) => {
                for n in &san.general_names {
                    match n {
                        GeneralName::DNSName(a) => {
                            println!("{}", a);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
