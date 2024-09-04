use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub secret_key: String,
}

impl AppConfig {
    // Load configuration from environment variables
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");
        AppConfig { database_url, secret_key }
    }
}
