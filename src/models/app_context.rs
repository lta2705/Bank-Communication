use std::sync::Arc;

use rdkafka::{consumer::StreamConsumer, producer::FutureProducer};
use sqlx::{Pool, Postgres};
use thiserror::Error;

pub struct AppContext {
    pub kafka_producer: Arc<FutureProducer>,
    pub kafka_consumer: Arc<StreamConsumer>,
    pub db_pool: Arc<Pool<Postgres>>,
}