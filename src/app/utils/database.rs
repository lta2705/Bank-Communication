use crate::app::config::database_config::DataBaseCfg;
use sqlx::{Error, Pool, Postgres, postgres::PgPoolOptions};

pub async fn establish_db_conn() -> Result<Pool<Postgres>, Error> {
    let db_cfg = DataBaseCfg::new();

    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_cfg.user_name.as_str(),
        db_cfg.password.as_str(),
        db_cfg.host.as_str(),
        db_cfg.port.as_str(),
        db_cfg.db_name.as_str(),
    );

    let pool = PgPoolOptions::new()
        .max_connections(db_cfg.max_conn as u32)
        .min_connections(db_cfg.min_conn as u32)
        .max_lifetime(std::time::Duration::from_secs(db_cfg.max_life_time as u64))
        .idle_timeout(std::time::Duration::from_secs(db_cfg.idle_timeout as u64))
        .acquire_timeout(std::time::Duration::from_millis(db_cfg.acquired_timeout as u64))
        .connect(&db_url.as_str())
        .await?;

    Ok(pool)
}
