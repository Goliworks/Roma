use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigYml {
    pub http: Http,
    pub services: HashMap<String, Services>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Http {
    pub port: Option<u16>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Services {
    pub location: String
}