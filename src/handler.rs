use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web::client::Client;

use crate::utils;
use crate::config;

pub async fn handler(
    req: HttpRequest,
    body: web::Bytes,
    conf: web::Data<config::Config>,
    client: web::Data<Client>)
    -> Result<HttpResponse, Error> {

    let dom = utils::get_domain(&req);
    println!("{}", dom);

    println!("{}", req.uri().path_and_query().unwrap()); // test.

    let dest = format!("http://{}{}", conf.destinations.get(dom).unwrap(), req.uri().path_and_query().unwrap());

    let forwarded_req = client
        .request_from(dest, req.head())
        .no_decompress();

    let forwarded_req = if let Some(addr) = req.head().peer_addr {
        forwarded_req.header("x-forwarded-for", format!("{}", addr.ip()))
    } else {
        forwarded_req
    };

    let res;

    match forwarded_req.send_body(body).await.map_err(Error::from) {
        Ok(f) => res = f,
        Err(_) => return Ok(HttpResponse::BadGateway().body("<h1>Error 502</h1><h2>Bad Gateway</h2>"))
    };

    let mut client_resp = HttpResponse::build(res.status());

    // Add headers from res to client_resp.
    for (header_name, header_value) in
    res.headers().iter().filter(|(h, _)| *h != "connection")
    {
        client_resp.header(header_name.clone(), header_value.clone());
    }

    Ok(client_resp.streaming(res))
}