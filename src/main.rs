use actix_web::{web, App, HttpServer};
use actix_web::client::Client;

mod handler;
mod utils;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Run server.");
    HttpServer::new(|| {
        App::new()
            .data(Client::new())
            .default_service(web::route().to(handler::handler))
    })
        .bind("0:8080")?
        .run()
        .await
}
