use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use tracing::info;
 
pub async fn create_pool(database_url: &str, max: u32, min: u32) -> anyhow::Result<PgPool> {
    info!("Connecting to PostgreSQL...");
 
    let pool = PgPoolOptions::new()
        .max_connections(max)
        .min_connections(min)
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
        .connect(database_url)
        .await?;
 
    // Verify connection
    sqlx::query("SELECT 1").execute(&pool).await?;
    info!("PostgreSQL connection established (pool max={})", max);
 
    Ok(pool)
}