pub use self::account_view_service::*;
use super::*;

// This struct represents the state
pub struct AppState {
  pub db_client: db::Pool,
}

pub mod account_view_service {

  use super::*;
  use actix_web::web;

  pub async fn get_account(
    account_id: &str,
    data: web::Data<service::AppState>,
  ) -> models::Account {
    db::get_account(data.db_client.clone(), account_id)
      .await
      .expect("Account loading fails!")
  }

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
