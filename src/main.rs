use actix_web::{web, App, HttpServer};
use actix_web::client::Client;

mod handler;
mod utils;
mod config;
mod yaml_model;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let configuration: config::Config = config::Config::new();

    let tls_config = config::get_tls_config(&configuration.certificates);

    println!("Listen on port {}", configuration.port);
    println!("Listen on TLS port {}", configuration.port_tls);

    let http_port: u16 = configuration.port;
    let https_port: u16 = configuration.port_tls;

    HttpServer::new(move || {
        App::new()
            .data(configuration.clone())
            .data(Client::new())
            .default_service(web::route().to(handler::handler))
    })
        .bind(format!("0:{}", http_port))?
        .bind_rustls(format!("0:{}", https_port), tls_config)?
        .run()
        .await?;

    Ok(())
}
