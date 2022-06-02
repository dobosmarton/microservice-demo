use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
  #[envconfig(from = "HOST")]
  pub host: String,

  #[envconfig(from = "PORT")]
  pub port: u16,

  #[envconfig(from = "CREATE_ACCOUNT_URL")]
  pub create_account_url: String,

  #[envconfig(from = "ACCOUNT_VIEW_URL")]
  pub account_view_url: String,
}
