use actix_web::{web, App, HttpServer};
use actix_web::client::Client;
use std::process::exit;
use std::io::{ErrorKind, Error};

mod handler;
mod utils;
mod config;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let configuration: config::Config = config::Config::new();

    let tls_config = config::tls::get_tls_config(&configuration.certificates);

    let http_port: u16 = configuration.port;
    let https_port: u16 = configuration.port_tls;

    HttpServer::new(move || {
        App::new()
            .data(configuration.clone())
            .data(Client::new())
            .default_service(web::route().to(handler::handler))
    })
        .bind(format!("0:{}", http_port))
        .and_then(|hs| {
            println!("Listen on port {}", http_port);
            Ok(hs)
        })
        .unwrap_or_else(|err| {
            port_err(&err, &http_port);
            exit(1);
        })
        .bind_rustls(format!("0:{}", https_port), tls_config)
        .and_then(|hs| {
            println!("Listen on TLS port {}", https_port);
            Ok(hs)
        })
        .unwrap_or_else(|err| {
            port_err(&err, &https_port);
            exit(1);
        })
        .run()
        .await?;

    Ok(())
}

fn port_err(err: &Error, port: &u16) {
    if err.kind() == ErrorKind::PermissionDenied {
        println!("Error : port {} is not allowed.", port);
    } else {
        println!("{:?}", err);
    }
}