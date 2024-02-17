use actix_web::{HttpResponse, Responder, HttpServer};

async fn health() -> impl Responder {
    HttpResponse::Ok().finish()
}

async fn greet(name: String) -> impl Responder {
    format!("Hello, {}", if name.is_empty() { String::from("World") } else { name })
}

pub fn run() -> Result<actix_web::dev::Server, std::io::Error> {
    let server = HttpServer::new(|| {
        actix_web::App::new()
            .route("/health", actix_web::web::get().to(health))
            .route("/greet/{name}", actix_web::web::get().to(greet))
    }).bind("127.0.0.1:8080")?.run();

    Ok(server)
}
