use actix_web::{middleware::Logger, web, App, HttpServer};
use envconfig::Envconfig;

// module declarations
mod config;
mod db;
mod kafka;
mod models;
mod routes;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let conf = config::Config::init_from_env().unwrap();

    let producer =
        kafka::create_producer(&conf.kafka_host).expect("failed to create kafka producer");

    let db_client = db::create_client(&conf.mongodb_uri).await;

    let state = web::Data::new(service::AppState {
        kafka_producer: producer,
        db_client,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(Logger::default())
            .configure(routes::init)
    })
    .bind((conf.host, conf.port))?
    .run()
    .await
}
