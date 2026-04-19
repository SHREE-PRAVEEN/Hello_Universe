use backend::{config, routes, utils::errors::AppError};
use std::net::SocketAddr;
use tracing::info;
 
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file
    dotenvy::dotenv().ok();
 
    // Initialize tracing
    backend::config::tracing::init_tracing();
 
    info!("Starting Robotics Platform Backend");
 
    // Build app state (DB pool, Redis, config)
    let state = config::AppState::new().await?;
 
    // Run DB migrations
    sqlx::migrate!("./migrations")
        .run(&state.db)
        .await
        .expect("Failed to run database migrations");
 
    info!("Database migrations complete");
 
    // Start background jobs
    backend::jobs::start_background_jobs(state.clone()).await;
 
    // Build the router
    let app = routes::create_router(state.clone());
 
    let addr: SocketAddr = format!(
        "{}:{}",
        state.config.host,
        state.config.port
    )
    .parse()?;
 
    info!("Listening on {}", addr);
 
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
 
    Ok(())
}
 