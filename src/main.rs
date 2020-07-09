use actix_web::{web, App, HttpServer};

async fn handler() -> String {
    let hello:String = String::from("Hello, world");
    hello
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Run server.");
    HttpServer::new(|| {
        App::new()
            .default_service(web::route().to(handler))
    })
        .bind("0:8000")?
        .run()
        .await
}
