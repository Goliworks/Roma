use actix_web::HttpResponse;

pub fn bad_gateway() -> HttpResponse{
    HttpResponse::BadGateway()
        .body("<div style='text-align:center; margin-top:100px;\
            font-family:helvetica, arial;'>\
            <div><h1>Error 502</h1>\
            <span>Bad gateway</span></div></div>")
}
