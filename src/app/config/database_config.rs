use dotenvy;
use std::env;

#[derive(Debug, Clone)]
pub struct DataBaseCfg {
    pub host: String,
    pub port: String,
    pub user_name: String,
    pub password: String,
    pub db_name: String,
    pub max_conn: i32,
    pub min_conn: i32,
    pub max_idle_conn: i32,
    pub max_life_time: i32,
    pub idle_timeout: i32,
}

impl DataBaseCfg {
    pub fn new() -> Self {
        // Load enviroment variables
        dotenvy::dotenv().ok(); //use ok if file not exists

        let get_env_var = |key: &str| env::var(key).unwrap_or_else(|_|
            panic!("Environment variables {} was not configured", key)
        );

        // Hàm helper để lấy giá trị i32 từ env
        let get_env_i32 = |key: &str| get_env_var(key).parse::<i32>().unwrap_or_else(|_|
            panic!("Environment variable {} was not proper integer", key)
        );

        DataBaseCfg {
            host: get_env_var("DB_HOST"),
            port: get_env_var("DB_PORT"),
            user_name: get_env_var("DB_USERNAME"),
            password: get_env_var("DB_PASSWORD"),
            db_name: get_env_var("DB_NAME"),
            max_conn: get_env_i32("DB_MAX_CONNECTIONS"),
            min_conn: get_env_i32("DB_MIN_CONNECTIONS"),
            max_idle_conn: get_env_i32("DB_MAX_IDLE_CONNECTIONS"),
            max_life_time: get_env_i32("DB_CONNECTION_MAX_LIFETIME"),
            idle_timeout: get_env_i32("DB_IDLE_TIMEOUT")
        }
    }
}