use actix_web::{middleware::Logger, App, HttpServer};
use envconfig::Envconfig;

// module declarations
mod config;
mod db;
mod kafka;
mod models;
mod routes;
mod service;

use kafka::KafkaConsumer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let conf = config::Config::init_from_env().unwrap();

    let consumer = KafkaConsumer::new(&conf.kafka_host).expect("failed to create kafka producer");

    let db_client = db::create_client(&conf.mongodb_uri).await;

    actix_web::rt::spawn(async move { consumer.run().await });

    HttpServer::new(move || App::new().wrap(Logger::default()).configure(routes::init))
        .bind((conf.host, conf.port))?
        .run()
        .await
}
