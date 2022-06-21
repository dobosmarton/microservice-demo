use super::*;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

#[get("/")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("I'm alive!")
}

/// GraphiQL playground UI
#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(
    st: web::Data<Schema>,
    data: web::Json<GraphQLRequest>,
    req: HttpRequest,
) -> impl Responder {
    let response = service::jwt_check(req)
        .await
        .expect("JWT check service error!");

    if response.status().is_success() {
        let response = data.execute(&st, &()).await;
        HttpResponse::Ok().json(response)
    } else {
        HttpResponse::Unauthorized().body(response.text().await.unwrap())
    }
}

/// Init of the service and routes
pub fn init(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("health").service(healthcheck))
        .service(web::scope("").service(graphql_playground).service(graphql));
}
