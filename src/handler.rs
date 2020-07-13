use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web::client::Client;

pub async fn handler(
    req: HttpRequest,
    body: web::Bytes,
    client: web::Data<Client>)
    -> Result<HttpResponse, Error> {

    println!("{}", req.uri().path_and_query().unwrap()); // test.

    let url = "http://localhost:3000";
    let dest = format!("{}{}", url, req.uri().path_and_query().unwrap());

    let forwarded_req = client
        .request_from(dest, req.head())
        .no_decompress();

    let forwarded_req = if let Some(addr) = req.head().peer_addr {
        forwarded_req.header("x-forwarded-for", format!("{}", addr.ip()))
    } else {
        forwarded_req
    };

    let res = forwarded_req.send_body(body).await.map_err(Error::from)?;

    let mut client_resp = HttpResponse::build(res.status());

    // Add headers from res to client_resp.
    for (header_name, header_value) in
    res.headers().iter().filter(|(h, _)| *h != "connection")
    {
        client_resp.header(header_name.clone(), header_value.clone());
    }

    Ok(client_resp.streaming(res))
}