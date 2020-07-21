use actix_web::{web, App, HttpServer};
use actix_web::client::Client;

mod handler;
mod utils;
mod config;
mod yaml_model;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let configuration:config::Config = config::Config::new();

    println!("Listen on port {}", configuration.port);

    let http_port:u16 = configuration.port;

    HttpServer::new(move || {
        App::new()
            .data(configuration.clone())
            .data(Client::new())
            .default_service(web::route().to(handler::handler))
    })
        .bind(format!("0:{}", http_port))?
        .run()
        .await
}
