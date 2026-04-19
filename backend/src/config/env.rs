use serde::Deserialize;
 
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    // Server
    pub host: String,
    pub port: u16,
    pub environment: String,
    pub platform_url: String,
    pub platform_name: String,
    pub cors_allowed_origins: String,
 
    // Database
    pub database_url: String,
    pub database_max_connections: u32,
    pub database_min_connections: u32,
 
    // Redis
    pub redis_url: String,
 
    // JWT
    pub jwt_secret: String,
    pub jwt_access_token_expiry_seconds: i64,
    pub jwt_refresh_token_expiry_days: i64,
 
    // AWS
    pub aws_access_key_id: String,
    pub aws_secret_access_key: String,
    pub aws_region: String,
    pub aws_s3_bucket: String,
    pub aws_s3_cdn_url: String,
 
    // Stripe
    pub stripe_secret_key: String,
    pub stripe_webhook_secret: String,
 
    // OpenAI
    pub openai_api_key: String,
    pub openai_model: String,
    pub openai_embedding_model: String,
 
    // IPFS
    pub ipfs_api_url: String,
    pub ipfs_pinata_jwt: String,
 
    // Email
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub email_from: String,
    pub email_from_name: String,
 
    // Rate limiting
    pub rate_limit_requests_per_minute: u32,
 
    // File uploads
    pub max_file_size_mb: u64,
}
 
impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let cfg = config::Config::builder()
            .add_source(config::Environment::default().separator("_"))
            .set_default("host", "0.0.0.0")?
            .set_default("port", 8080)?
            .set_default("environment", "development")?
            .set_default("database_max_connections", 20)?
            .set_default("database_min_connections", 2)?
            .set_default("jwt_access_token_expiry_seconds", 900)?
            .set_default("jwt_refresh_token_expiry_days", 30)?
            .set_default("rate_limit_requests_per_minute", 60)?
            .set_default("max_file_size_mb", 500)?
            .set_default("platform_name", "Robotics Platform")?
            .set_default("smtp_port", 587)?
            .build()?;
 
        Ok(cfg.try_deserialize()?)
    }
 
    pub fn is_production(&self) -> bool {
        self.environment == "production"
    }
 
    pub fn max_file_size_bytes(&self) -> u64 {
        self.max_file_size_mb * 1024 * 1024
    }
 
    pub fn cors_origins(&self) -> Vec<String> {
        self.cors_allowed_origins
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    }
}
 