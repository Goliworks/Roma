use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web::client::Client;

use crate::utils;
use crate::config;
use crate::proxy;

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

    let proxy = proxy::Proxy::new(&dest, client.get_ref());

    proxy.stream(req, body).await
}
