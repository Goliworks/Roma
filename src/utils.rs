use actix_web::{HttpRequest, http};

pub fn get_domain(req: &HttpRequest) -> &str {
    let domain = match req.uri().host() {
        Some(_) => req.uri().host().unwrap(),
        None => match req.headers().get(http::header::HOST) {
            None => "",
            Some(h) => {
                let host: Vec<&str> = h.to_str().unwrap().split(":").collect();
                host[0]
            }
        }
    };
    domain
}