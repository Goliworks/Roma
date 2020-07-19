use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigYml {
    pub http: Http
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Http {
    pub port: Option<u16>
}