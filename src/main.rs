use actix_web::{web, App, Error, HttpServer, HttpRequest, HttpResponse};
use actix_web::client::Client;

async fn handler(
    req: HttpRequest,
    body: web::Bytes,
    client: web::Data<Client>)
    -> Result<HttpResponse, Error> {
    let forwarded_req = client
        .request_from("http://localhost:3000", req.head())
        .no_decompress();

    let forwarded_req = if let Some(addr) = req.head().peer_addr {
        forwarded_req.header("x-forwarded-for", format!("{}", addr.ip()))
    } else {
        forwarded_req
    };

    let res = forwarded_req.send_body(body).await.map_err(Error::from)?;

    let mut client_resp = HttpResponse::build(res.status());

    Ok(client_resp.streaming(res))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Run server.");
    HttpServer::new(|| {
        App::new()
            .data(Client::new())
            .default_service(web::route().to(handler))
    })
        .bind("0:8080")?
        .run()
        .await
}
