use actix_web::{web, Error, HttpRequest, HttpResponse, http};
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

    // https redirection.
    let scheme = req.connection_info().scheme().to_string();
    if scheme == "http" && conf.auto_tls {
        let port = if conf.port_tls != config::DEFAULT_PORT_TLS {
            format!(":{}", conf.port_tls)
        } else {
            "".to_string()
        };
        let red_url = format!("https://{}{}", dom, port);
        return Ok(HttpResponse::MovedPermanently()
            .header(http::header::LOCATION, red_url)
            .finish()
            .into_body());
    }

    // continue if no redirection.
    let dest = format!("http://{}{}", conf.destinations.get(dom).unwrap(), req.uri().path_and_query().unwrap());
    let proxy = proxy::Proxy::new(&dest, client.get_ref());
    proxy.stream(req, body).await
}
