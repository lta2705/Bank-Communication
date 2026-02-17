use std::env;

pub struct ConnAttr {
    pub host: String,
    pub port: u16,
    #[allow(dead_code)]
    pub tls_port: u16,
}

impl ConnAttr {
    /// Loads environment variables and returns a Result instead of panicking.
    pub fn load_env() -> Result<Self, String> {
        // Load .env file
        dotenvy::dotenv().ok();

        // Helper closure to read string env
        let get_var = |key: &str| {
            env::var(key).map_err(|_| format!("Environment variable '{}' is missing", key))
        };

        // Helper closure to parse u16 (for ports)
        let get_u16 = |key: &str| {
            get_var(key)?.parse::<u16>().map_err(|_| {
                format!(
                    "Environment variable '{}' must be a valid port (0-65535)",
                    key
                )
            })
        };

        // Return the struct wrapped in Ok
        Ok(ConnAttr {
            host: get_var("APP_HOST")?,
            port: get_u16("APP_TCP_PORT")?,
            tls_port: get_u16("APP_TLS_TCP_PORT")?,
        })
    }
}
