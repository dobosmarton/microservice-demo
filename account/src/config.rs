use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "HOST")]
    pub host: String,

    #[envconfig(from = "PORT")]
    pub port: u16,

    #[envconfig(from = "MONGODB_URI")]
    pub mongodb_uri: String,

    #[envconfig(from = "KAFKA_HOST")]
    pub kafka_host: String,

    #[envconfig(from = "KAFKA_TOPIC")]
    pub kafka_topic: String,
}
