use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "HOST")]
    pub host: String,

    #[envconfig(from = "PORT")]
    pub port: u16,

    #[envconfig(from = "JWT_SECRET_KEY")]
    pub jwt_secret_key: String,
}

impl Config {
    pub fn get_config() -> Config {
        Config::init_from_env().unwrap()
    }
}
