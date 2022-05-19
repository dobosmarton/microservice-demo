use super::*;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/")]
async fn healthcheck(_: web::Data<service::AppState>) -> impl Responder {
    HttpResponse::Ok().body("I'm alive!")
}

/// Init of the service and routes
pub fn init(config: &mut web::ServiceConfig) {
    config.service(web::scope("health").service(healthcheck));
}
