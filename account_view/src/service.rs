pub use self::account_view_service::*;
use super::*;

// This struct represents the state
pub struct AppState {
  pub db_client: db::Pool,
}

pub mod account_view_service {

  use super::*;
  use actix_web::web;

  /// Create the account
  pub async fn create_account_view(
    json_account: &str,
    data: web::Data<service::AppState>,
  ) -> models::Account {
    let message: models::InputAccount = serde_json::from_str(&json_account).unwrap();

    let account = db::create_account(data.db_client.clone(), message)
      .await
      .expect("Account creation failed");

    return account;
  }
}
