use super::*;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

#[get("/")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("I'm alive!")
}

#[get("/verify")]
async fn verify_token(req: HttpRequest) -> impl Responder {
    match service::verify_jwt(&req).await {
        Ok(token) => HttpResponse::Accepted().json(token.claims),
        Err(err) => HttpResponse::Unauthorized().body(err.to_string()),
    }
}

/// Init of the service and routes
pub fn init(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("health").service(healthcheck))
        .service(web::scope("jwt").service(verify_token));
}
