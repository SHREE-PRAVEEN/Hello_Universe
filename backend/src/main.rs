use std::net::SocketAddr;

use anyhow::Context;
use backend::config::{db, env::AppConfig, tracing as tracing_cfg};
use backend::{AppState, build_app};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_cfg::init_tracing();

    let config = AppConfig::from_env().context("failed to load environment")?;
    let pool = db::connect_pg_pool(&config).await?;
    let redis = backend::config::redis::build_redis_client(&config.redis_url)?;

    let state = Arc::new(AppState::new(config.clone(), pool, redis));
    let app = build_app(state);

    let addr: SocketAddr = format!("{}:{}", config.host, config.port)
        .parse()
        .context("invalid host/port bind address")?;

    let listener = TcpListener::bind(addr)
        .await
        .context("failed to bind TCP listener")?;

    info!("backend listening on {}", addr);
    axum::serve(listener, app)
        .await
        .context("axum server failed")?;

    Ok(())
}
