use crate::app::config::kafka_config::KafkaConfig;
use rdkafka::config::ClientConfig;
use rdkafka::producer::FutureProducer;
use tracing::info;

pub fn create_producer(cfg: &KafkaConfig) -> anyhow::Result<FutureProducer> {
    let mut client_cfg = ClientConfig::new();

    client_cfg
        // Common
        .set("bootstrap.servers", &cfg.bootstrap_servers)
        // Producer configs
        .set("acks", &cfg.acks)
        .set("retries", cfg.retries.to_string())
        .set("linger.ms", cfg.linger_ms.to_string())
        .set("compression.type", &cfg.compression_type)
        .set(
            "max.in.flight.requests.per.connection",
            cfg.max_in_flight.to_string(),
        )
        .set("enable.idempotence", cfg.enable_idempotence.to_string())
        .set("request.timeout.ms", cfg.request_timeout_ms.to_string())
        .set("delivery.timeout.ms", cfg.delivery_timeout_ms.to_string());

    // Auto create topics configuration
    // Note: This is a broker-side configuration. The actual topic creation happens
    // automatically when a producer tries to send to a non-existent topic IF the
    // broker has "auto.create.topics.enable=true" (which is the default).
    // We log the client-side preference here.
    if cfg.auto_create_topic {
        info!("Kafka producer configured with auto-create topic enabled");
        info!("Topics will be auto-created on first message if broker allows it");
    } else {
        info!("Kafka producer configured with auto-create topic disabled");
        info!("Topics must be manually created before sending messages");
    }

    let producer: FutureProducer = client_cfg.create()?;
    Ok(producer)
}
