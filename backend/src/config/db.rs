use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::config::env::AppConfig;

pub async fn connect_pg_pool(config: &AppConfig) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .min_connections(2)
        .connect(&config.database_url)
        .await?;
    Ok(pool)
}
