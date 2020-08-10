mod handler;
mod utils;
mod config;
mod server;

fn main() -> std::io::Result<()> {
    let configuration: config::Config = config::Config::new();
    let tls_config = config::tls::get_tls_config(&configuration.certificates);

    server::server(configuration, tls_config)
}
