use envconfig::Envconfig;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{middleware, route, web::Data, App, HttpServer};
use actix_web_lab::respond::Html;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

// module declarations
mod config;
mod routes;
mod schema;
mod service;

use crate::schema::{create_schema, Schema};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let conf = config::Config::init_from_env().unwrap();

    // Create Juniper schema
    let schema = Arc::new(create_schema());

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(schema.clone()))
            .configure(routes::init)
            // the graphiql UI requires CORS to be enabled
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind((conf.host, conf.port))?
    .run()
    .await
}
