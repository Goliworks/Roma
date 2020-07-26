use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigYml {
    pub http: Http,
    pub services: HashMap<String, Services>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Http {
    pub port: Option<u16>,
    pub tls: Tls
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Tls {
    pub port: Option<u16>,
    pub certificates: Vec<Certificates>
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Certificates {
    pub cert: String,
    pub key: String
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Services {
    pub location: String
}