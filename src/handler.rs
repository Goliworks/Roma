use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web::client::Client;

pub async fn handler(
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