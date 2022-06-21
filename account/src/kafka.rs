pub use self::kafka::*;

pub mod kafka {

    use crate::config;
    use envconfig::Envconfig;
    use rdkafka::config::ClientConfig;
    use rdkafka::error::KafkaError;
    use rdkafka::producer::{FutureProducer, FutureRecord};
    use std::time::Duration;

    fn get_topic_name() -> String {
        let conf = config::Config::init_from_env().unwrap();
        return conf.kafka_topic;
    }

    pub async fn send_message(producer: FutureProducer, message: &str) {
        let topic_name = get_topic_name();
        let record = FutureRecord::to(topic_name.as_str())
            .payload(message)
            .key("");

        match producer.clone().send(record, Duration::from_secs(0)).await {
            Ok(_) => println!("Message sent to {}: {}", topic_name, message),
            Err(cancelled_err) => eprintln!("{}", format!("cancelled, {:?}", cancelled_err)),
        }
    }

    pub fn create_producer(kafka_host: &str) -> Result<FutureProducer, KafkaError> {
        ClientConfig::new()
            .set("bootstrap.servers", kafka_host)
            .set("queue.buffering.max.ms", "0") // Do not buffer
            .create()
    }
}
