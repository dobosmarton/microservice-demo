pub use self::gateway_service::*;
use super::*;

pub mod gateway_service {

  use super::*;

  fn get_config() -> config::Config {
    config::Config::init_from_env().unwrap()
  }

  pub async fn get_account(_id: &str) -> Result<schema::Account, reqwest::Error> {
    let account_url = get_config().account_view_url + _id;

    reqwest::get(account_url)
      .await?
      .json::<schema::Account>()
      .await
  }

  pub async fn create_account(
    account: schema::AccountInput,
  ) -> Result<schema::AccountId, reqwest::Error> {
    reqwest::Client::new()
      .post(get_config().create_account_url)
      .json(&account)
      .send()
      .await?
      .json::<schema::AccountId>()
      .await
  }
}
