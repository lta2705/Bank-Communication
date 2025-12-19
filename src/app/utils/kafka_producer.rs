// use crate::app::config::kafka_config::KafkaConfig;
// use rdkafka::config::ClientConfig;
// use rdkafka::producer::FutureProducer;

// pub fn create_producer(cfg: &KafkaConfig) -> anyhow::Result<FutureProducer> {
//     let mut client_cfg = ClientConfig::new();

//     client_cfg
//         // Common
//         .set("bootstrap.servers", &cfg.bootstrap_servers)

//         // Producer configs
//         .set("acks", &cfg.acks)
//         .set("retries", cfg.retries.to_string())
//         .set("linger.ms", cfg.linger_ms.to_string())
//         .set("compression.type", &cfg.compression_type)
//         .set(
//             "max.in.flight.requests.per.connection",
//             cfg.max_in_flight.to_string(),
//         )
//         .set("enable.idempotence", cfg.enable_idempotence.to_string())
//         .set(
//             "request.timeout.ms",
//             cfg.request_timeout_ms.to_string(),
//         )
//         .set(
//             "delivery.timeout.ms",
//             cfg.delivery_timeout_ms.to_string(),
//         );

//     let producer: FutureProducer = client_cfg.create()?;
//     Ok(producer)
// }
