use redis::{aio::ConnectionManager, Client};
use tracing::info;
 
pub type RedisPool = ConnectionManager;
 
pub async fn create_redis_pool(redis_url: &str) -> anyhow::Result<RedisPool> {
    info!("Connecting to Redis...");
    let client = Client::open(redis_url)?;
    let manager = ConnectionManager::new(client).await?;
    info!("Redis connection established");
    Ok(manager)
}
 