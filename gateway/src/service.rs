pub use self::gateway_service::*;
use super::*;

pub mod gateway_service {

  use super::*;

  pub async fn get_account(_id: &str) -> Result<schema::Account, reqwest::Error> {
    let conf = config::Config::init_from_env().unwrap();
    let account_url = conf.account_view_url + _id;

    reqwest::get(account_url)
      .await?
      .json::<schema::Account>()
      .await
  }
}
