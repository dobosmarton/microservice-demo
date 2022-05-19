pub use self::account_view_service::*;
use super::*;

// This struct represents the state
pub struct AppState {
  pub kafka_producer: rdkafka::producer::FutureProducer,
  pub db_client: mongodb::Client,
}

pub mod account_view_service {

  use super::*;
  use actix_web::web;
  use uuid::Uuid;

  /// Create the account
  pub async fn create_account_view(account: &models::Account, data: web::Data<AppState>) -> String {
    let json_account = serde_json::to_string(&account).unwrap();

    // Insert the event into the db
    // db::insert_event(&data.db_client, &json_account).await;

    return json_account;
  }
}
