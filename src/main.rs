use actix_web::{App, HttpServer};

mod controllers;
mod routers;
mod services;

use routers::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(routes::get_routers()))
    .bind("127.0.0.1:8000")?
    .run()
    .await
}