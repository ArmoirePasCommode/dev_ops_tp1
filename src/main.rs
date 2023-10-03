use actix_web::{web, App, HttpResponse, HttpServer, Result};
use env_logger::Env;
use serde::Serialize;

#[derive(Serialize)]
struct RequestHeaders {
    headers: Vec<(String, String)>,
}

async fn ping(request: actix_web::HttpRequest) -> HttpResponse {
    let headers = request.headers();
    let headers_vec: Vec<(String, String)> = headers
        .iter()
        .map(|(name, value)| {
            (
                name.as_str().to_string(),
                value.to_str().unwrap_or("").to_string(),
            )
        })
        .collect();

    let response_data = RequestHeaders {
        headers: headers_vec,
    };

    HttpResponse::Ok().json(response_data)
}

async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let listen_port = std::env::var("PING_LISTEN_PORT")
        .and_then(|s| s.parse().ok())
        .unwrap_or(7878);

    HttpServer::new(|| {
        App::new()
            .route("/ping", web::get().to(ping))
            .default_service(web::route().to(not_found)) // Gère toutes les autres routes
    })
    .bind(("127.0.0.1", listen_port))?
    .run()
    .await
}
