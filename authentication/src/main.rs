use actix_web::{middleware::Logger, App, HttpServer};
use envconfig::Envconfig;

// module declarations
mod config;
mod models;
mod routes;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let conf = config::Config::init_from_env().unwrap();

    HttpServer::new(move || App::new().wrap(Logger::default()).configure(routes::init))
        .bind((conf.host, conf.port))?
        .run()
        .await
}
