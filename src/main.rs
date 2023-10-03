use actix_web::{get, web, App, HttpResponse, HttpServer};
use serde::Serialize;

#[derive(Serialize)]
struct HeadersResponse {
    headers: Vec<(String, String)>,
}

#[get("/ping")]
async fn ping(request: actix_web::HttpRequest) -> HttpResponse {
    let headers_map: Vec<(String, String)> = request
        .headers()
        .iter()
        .map(|header| {
            let name = header.0.as_str().to_string();
            let value = header.1.to_str().unwrap_or("").to_string();
            (name, value)
        })
        .collect();

    let response = HeadersResponse {
        headers: headers_map,
    };

    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::scope("/").service(ping)))
        .bind("127.0.0.1:7878")?
        .run()
        .await
}
