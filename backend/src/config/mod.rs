pub mod db;
pub mod env;
pub mod redis;
pub mod storage;
pub mod tracing;
 
use std::sync::Arc;
use sqlx::PgPool;
 
pub use env::Config;
pub use redis::RedisPool;
 
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: RedisPool,
    pub config: Arc<Config>,
}
 
impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        let config = Config::from_env()?;
 
        let db = db::create_pool(
            &config.database_url,
            config.database_max_connections,
            config.database_min_connections,
        )
        .await?;
 
        let redis = redis::create_redis_pool(&config.redis_url).await?;
 
        Ok(Self {
            db,
            redis,
            config: Arc::new(config),
        })
    }
}
 