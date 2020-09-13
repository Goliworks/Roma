use actix_web::{web, Error, HttpRequest, HttpResponse, http};
use actix_web::client::Client;

use crate::utils;
use crate::config;
use crate::proxy;
use crate::http_error;

pub async fn handler(
    req: HttpRequest,
    body: web::Bytes,
    conf: web::Data<config::Config>,
    client: web::Data<Client>)
    -> Result<HttpResponse, Error> {
    let dom = utils::host_to_domain(req.connection_info().host().to_string());
    let paq = req.uri().path_and_query().unwrap();
    println!("{}", dom);
    println!("{}", paq); // test.

    // simple redirection

    match conf.redirections.get( &dom) {
        Some(tar) => {
            return Ok(HttpResponse::MovedPermanently()
                .header(http::header::LOCATION, tar.to_string())
                .finish()
                .into_body());
        }
        None => {}
    }

    // https redirection.
    let scheme = req.connection_info().scheme().to_string();
    if scheme == "http" && conf.auto_tls {
        let port = if conf.port_tls != config::DEFAULT_PORT_TLS {
            format!(":{}", conf.port_tls)
        } else {
            "".to_string()
        };
        let red_url = format!("https://{}{}{}", dom, port, paq);
        return Ok(HttpResponse::MovedPermanently()
            .header(http::header::LOCATION, red_url)
            .finish()
            .into_body());
    }

    // continue if no redirection.
    match conf.destinations.get(&dom) {
        Some(d) => {
            let dest = format!("http://{}{}", d, paq);
            let proxy = proxy::Proxy::new(&dest, client.get_ref());
            proxy.stream(req, body).await
        },
        None => {
            println!("Unknow domain");
            Ok(http_error::bad_gateway())
        },
    }
}
