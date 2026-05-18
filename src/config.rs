#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub db_max_connections: u32,
    pub rust_log: String,
    pub tls_cert_path: String,
    pub tls_key_path: String,
}

impl AppConfig {
    pub fn load() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            server_host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("SERVER_PORT must be a number"),
            db_max_connections: std::env::var("DB_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .expect("DB_MAX_CONNECTIONS must be a number"),
            rust_log: std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "info,tower_http=debug,sqlx=warn".to_string()),
            tls_cert_path: std::env::var("TLS_CERT_PATH")
                .unwrap_or_else(|_| "certs/cert.pem".to_string()),
            tls_key_path: std::env::var("TLS_KEY_PATH")
                .unwrap_or_else(|_| "certs/key.pem".to_string()),
        }
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}
