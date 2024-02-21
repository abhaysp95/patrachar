use actix_web::{HttpServer, Responder};
use std::net::TcpListener;

pub mod controller;
pub mod dto;

pub fn run(listener: TcpListener) -> Result<actix_web::dev::Server, std::io::Error> {
    let server = HttpServer::new(|| {
        actix_web::App::new()
            .route("/health", actix_web::web::get().to(controller::health))
            .route("/subscription", actix_web::web::put().to(controller::subscription))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
