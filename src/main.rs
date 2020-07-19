use actix_web::{web, App, HttpServer};
use actix_web::client::Client;

mod handler;
mod utils;
mod config;
mod yaml_model;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let configuration = config::Config::new();

    println!("Listen on port {}", configuration.port);

    HttpServer::new(|| {
        App::new()
            .data(Client::new())
            .default_service(web::route().to(handler::handler))
    })
        .bind(format!("0:{}", configuration.port))?
        .run()
        .await
}
