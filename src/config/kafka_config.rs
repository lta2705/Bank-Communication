use std::{env, str::FromStr};
use dotenvy::dotenv;

trait KafkaConfig{
    fn setup_producer() -> Self {}
    fn setup_consumer() -> Self {} 
}

pub struct KafkaProducer {
    broker: String,
    acks: String,
    retries: i32,
    enable_idempotence: bool,
    request_timeout_ms: i32,
    delivery_timeout_ms: i32,
    topic: String
}

pub struct KafkaConsumer {
    broker: String,
    enable_auto_commit: bool,
    max_poll_interval: i32,
    session_timeout_ms: i32,
    heartbeat_interval_ms: i32,
    isolation_level: String,
    auto_offset_reset: String
}

fn get_env<T: std::str::FromStr>(key: &str) -> T 
where <T as FromStr>::Err: std::fmt::Debug, 
{
    dotenvy::from_filename("../kafka_cfg.env").expect("Cannot load .env file");
    
    let val = env::var(key).unwrap_or_else(|_| panic!("Missing env variable: {}", key));
    
    val.parse::<T>().unwrap_or_else(|_| panic!("Invalid type for env variable: {}", key)) 
}

impl KafkaConfig for KafkaProducer {
    fn setup_producer() -> Self {
        KafkaProducer { 
            broker: get_env("KAFKA_BOOTSTRAP_SERVERS"), 
            acks: get_env("KAFKA_PRODUCER_ACKS"), 
            retries: get_env("KAFKA_PRODUCER_RETRIES"), 
            enable_idempotence: get_env("KAFKA_PRODUCER_ENABLE_IDEMPOTENCE"), 
            request_timeout_ms: get_env("KAFKA_PRODUCER_ENABLE_IDEMPOTENCE"), 
            delivery_timeout_ms: get_env("KAFKA_PRODUCER_ENABLE_IDEMPOTENCE"), 
            topic: get_env("KAFKA_PRODUCER_ENABLE_IDEMPOTENCE") 
        }
    }
}

impl KafkaConfig for KafkaConsumer  {
    fn setup_consumer() -> Self {
        KafkaConsumer { 
            broker: get_env(""), 
            topic: get_env(""),
            enable_auto_commit: get_env(""), 
            max_poll_interval: get_env(""), 
            session_timeout_ms: get_env(""), 
            heartbeat_interval_ms: get_env(""), 
            isolation_level: get_env(""), 
            auto_offset_reset: get_env("") }
    }
    
}


    