use actix_web::{HttpServer, Responder};
use std::net::TcpListener;

mod controller;

pub fn run(listener: TcpListener) -> Result<actix_web::dev::Server, std::io::Error> {
    let server = HttpServer::new(|| {
        actix_web::App::new()
            .route("/health", actix_web::web::get().to(controller::health))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
