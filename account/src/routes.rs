use super::*;
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/")]
async fn healthcheck(_: web::Data<service::AppState>) -> impl Responder {
    HttpResponse::Ok().body("I'm alive!")
}

#[post("/create")]
async fn create_account(
    account: web::Json<models::AccountInput>,
    data: web::Data<service::AppState>,
) -> impl Responder {
    let new_account_id = service::create_account(&account.into_inner(), data).await;

    let account_response_json = serde_json::to_string(&new_account_id).unwrap();

    HttpResponse::Ok().body(account_response_json)
}

/// Init of the service and routes
pub fn init(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("health").service(healthcheck))
        .service(web::scope("account").service(create_account));
}
