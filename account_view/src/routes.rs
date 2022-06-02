use super::*;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/")]
async fn healthcheck(_: web::Data<service::AppState>) -> impl Responder {
    HttpResponse::Ok().body("I'm alive!")
}

#[get("/{id}")]
async fn get_account(
    path: web::Path<(String,)>,
    data: web::Data<service::AppState>,
) -> impl Responder {
    let account_id = path.into_inner().0;
    let account = service::get_account(&account_id, data).await;

    let account_json = serde_json::to_string(&account).unwrap();

    HttpResponse::Ok().body(account_json)
}

/// Init of the service and routes
pub fn init(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("health").service(healthcheck))
        .service(web::scope("account").service(get_account));
}
