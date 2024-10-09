use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde;
use std::net::TcpListener;

// pub mod routes;
// import health_check from routes

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route(
                "/health_check",
                web::get().to(crate::routes::health_check::health_check),
            )
            .route(
                "/subscriptions",
                web::post().to(crate::routes::subscription::subscribe),
            )
    })
    .listen(listener)?
    .run();
    Ok(server)
}
