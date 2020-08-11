use actix_web::client::Client;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use crate::http_error;

pub struct Proxy<'a> {
    destination: &'a String,
    client: &'a Client,
}

impl<'a> Proxy<'a> {
    pub fn new(dest: &'a String, client: &'a Client) -> Proxy<'a> {
        Proxy {
            destination: dest,
            client,
        }
    }

    pub async fn stream(&self, req: HttpRequest, body: web::Bytes) -> Result<HttpResponse, Error> {
        let forwarded_req = self.client
            .request_from(self.destination, req.head())
            .no_decompress();

        let forwarded_req = if let Some(addr) = req.head().peer_addr {
            forwarded_req.header("x-forwarded-for", format!("{}", addr.ip()))
        } else {
            forwarded_req
        };

        let res;

        // Check for bad gateway.
        match forwarded_req.send_body(body).await.map_err(Error::from) {
            Ok(f) => res = f,
            Err(_) => return Ok(http_error::bad_gateway())
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
}
