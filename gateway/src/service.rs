pub use self::gateway_service::*;
use super::*;
use actix_web::HttpRequest;
use config::Config;

pub mod gateway_service {

    use super::*;

    pub async fn jwt_check(req: HttpRequest) -> Result<reqwest::Response, reqwest::Error> {
        let authentication_url = Config::get_config().authentication_url;
        let client = reqwest::Client::new();

        let mut headers = reqwest::header::HeaderMap::new();

        req.headers().into_iter().for_each(|header| {
            headers.insert(header.0.clone(), header.1.clone());
        });

        client.get(authentication_url).headers(headers).send().await
    }

    pub async fn get_account(_id: &str) -> Result<schema::Account, reqwest::Error> {
        let account_url = Config::get_config().account_view_url + _id;

        reqwest::get(account_url)
            .await?
            .json::<schema::Account>()
            .await
    }

    pub async fn create_account(
        account: schema::AccountInput,
    ) -> Result<schema::AccountId, reqwest::Error> {
        reqwest::Client::new()
            .post(Config::get_config().create_account_url)
            .json(&account)
            .send()
            .await?
            .json::<schema::AccountId>()
            .await
    }
}
