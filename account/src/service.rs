pub use self::account_service::*;
use super::*;
use actix_web::web;
use uuid::Uuid;

// This struct represents the state
pub struct AppState {
  pub kafka_producer: rdkafka::producer::FutureProducer,
  pub db_client: mongodb::Client,
}

pub mod account_service {
  use super::*;

  /// Create the account
  pub async fn create_account(
    input_account: &models::AccountInput,
    data: web::Data<AppState>,
  ) -> String {
    let account = models::Account {
      id: Uuid::new_v4().to_string(),
      first_name: String::from(&input_account.first_name),
      last_name: String::from(&input_account.last_name),
      email: String::from(&input_account.email),
    };

    let json_account = serde_json::to_string(&account).unwrap();

    // Insert the event into the db
    db::insert_event(&data.db_client, &json_account)
      .await
      .expect("Insert event failed!");

    // Send the message
    kafka::send_message(data.kafka_producer.clone(), &json_account).await;

    return account.id;
  }
}
