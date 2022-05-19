use crate::config;
use envconfig::Envconfig;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::Consumer;
use rdkafka::error::KafkaError;
use rdkafka::message::Message as KafkaMessage;

pub struct KafkaConsumer {
  consumer: StreamConsumer,
}

impl KafkaConsumer {
  pub fn new(kafka_host: &str) -> Result<Self, KafkaError> {
    let consumer = Self::create_consumer(kafka_host)?;
    Ok(KafkaConsumer { consumer })
  }

  pub async fn run(&self) {
    loop {
      match self.consumer.recv().await {
        Err(e) => eprintln!("{}", &format!("errors from kafka, {}", e.to_string())),
        Ok(m) => {
          let payload = match m.payload_view::<str>() {
            None => "",
            Some(Ok(s)) => s,
            Some(Err(e)) => {
              eprintln!("Error while deserializing message payload: {:?}", e);
              ""
            }
          };
          println!("payload: {}", payload,);
          // self.consumer.commit_message(&m, CommitMode::Async).unwrap();
        }
      };
    }
  }

  fn get_topic_name() -> String {
    let conf = config::Config::init_from_env().unwrap();
    return conf.kafka_topic;
  }

  fn create_consumer(kafka_host: &str) -> Result<StreamConsumer, KafkaError> {
    let stream_consumer: StreamConsumer = ClientConfig::new()
      .set("group.id", "account-v1")
      .set("bootstrap.servers", kafka_host)
      .set("auto.offset.reset", "latest")
      .set("enable.partition.eof", "true")
      .set("session.timeout.ms", "6000")
      .set("enable.auto.commit", "true")
      .set_log_level(RDKafkaLogLevel::Debug)
      .create()?;

    stream_consumer.subscribe(&[&Self::get_topic_name()]);

    Ok(stream_consumer)
  }
}
