use actix_web::{middleware::Logger, web, App, HttpServer};
use envconfig::Envconfig;

#[macro_use]
extern crate diesel;

// module declarations
mod config;
mod db;
mod kafka;
mod models;
mod routes;
mod schema;
mod service;

use kafka::KafkaConsumer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let conf = config::Config::init_from_env().unwrap();

    let consumer = KafkaConsumer::new(&conf.kafka_host).expect("failed to create kafka producer");

    let db_pool = db::create_client(&conf.postgres_uri).await;

    let state = web::Data::new(service::AppState { db_client: db_pool });

    let consumer_state = state.clone();

    actix_web::rt::spawn(async move { consumer.run(&consumer_state).await });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(state.clone())
            .configure(routes::init)
    })
    .bind((conf.host, conf.port))?
    .run()
    .await
}
