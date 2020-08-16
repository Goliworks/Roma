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

pub struct TLSConfig<'a, 'b> {
    certs: &'a Vec<Certificates>,
    domains: Vec<&'b String>,
}

impl <'a, 'b>TLSConfig<'a, 'b> {
    pub fn new(certs: &'a Vec<Certificates>, domains:Vec<&'b String>) -> TLSConfig<'a, 'b> {
        TLSConfig {
            certs,
            domains,
        }
    }

    pub fn get_tls_config(&self) -> ServerConfig {
        let mut resolver = ResolvesServerCertUsingSNI::new();
        let mut config_tls = ServerConfig::new(NoClientAuth::new());

        self.certs.into_iter().for_each(|c| {
            self.add_certificate_to_resolver(c, &mut resolver);
        });

        config_tls.cert_resolver = Arc::new(resolver);
        config_tls
    }

    fn add_certificate_to_resolver(
        &self,
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
        let cn = self.get_domain(buffer);

        let cert_chain = certs(br_cert).unwrap();
        let mut keys = rsa_private_keys(br_key).unwrap();
        let signing_key = RSASigningKey::new(
            &keys.remove(0)
        ).unwrap();
        let signing_key_boxed: Arc<Box<dyn SigningKey>> = Arc::new(
            Box::new(signing_key)
        );

        cn.into_iter().for_each(|dom| {
            resolver.add(dom.as_str(), rustls::sign::CertifiedKey::new(
                cert_chain.clone(), signing_key_boxed.clone(),
            )).expect("Invalid certificate");
        });
    }

    fn get_domain(&self, buffer: &[u8]) -> Vec<String> {
        let res = pem_to_der(&buffer);

        match res {
            Ok((_, pem)) => {
                let x509 = parse_x509_der(&pem.contents);
                match x509 {
                    Ok((_, cert)) => {
                        println!("CN = {}",  TLSConfig::get_common_name(&cert));
                        TLSConfig::get_san(&cert)
                    }
                    _ => panic!("x509 parsing failed: {:?}", x509),
                }
            }
            _ => panic!("PEM parsing failed: {:?}", res),
        }

    }

    fn get_common_name(cert: &X509Certificate) -> String {
        let subject= cert.tbs_certificate.subject.to_string();
        let cn: Vec<&str> = subject.split("CN=").collect();
        cn[1].to_string()
    }

    fn get_san(cert: &X509Certificate) -> Vec<String> {
        let mut dns:Vec<String> = Vec::new();
        for (_, ext) in cert.extensions() {
            match ext.parsed_extension() {
                ParsedExtension::SubjectAlternativeName(san) => {
                    for gn in &san.general_names {
                        match gn {
                            GeneralName::DNSName(dom) => {
                                dns.push(dom.to_string())
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        println!("SNA : {:?}", dns);
        dns
    }
}