mod http_error;
mod proxy;
mod handler;
mod utils;
mod config;
mod server;

fn main() -> std::io::Result<()> {
    let configuration: config::Config = config::Config::new();

    let mut domain_list:Vec<&String> = Vec::new();
    for (dom, _) in &configuration.destinations {
        domain_list.push(dom);
    }
    let tls_config = config::tls::TLSConfig::new(&configuration.certificates, domain_list).get_tls_config();

    server::server(configuration, tls_config)
}
