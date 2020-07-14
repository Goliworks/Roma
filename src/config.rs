const DEFAULT_PORT: u16 = 8080; // temporary. Must be 80 afterward.

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16
}

impl Config {
    pub fn new() -> Config {
        Config {
            port: DEFAULT_PORT
        }
    }
}