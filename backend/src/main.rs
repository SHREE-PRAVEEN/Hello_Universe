mod config;
mod routes;
mod controllers;
mod services;
mod models;
mod utils;
mod errors;
mod middleware;

use actix_web::{web, App, HttpServer, middleware as actix_middleware, HttpResponse};
use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use sqlx::PgPool;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing for better logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "backend=debug,actix_web=info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv::dotenv().ok();

    let config = config::AppConfig::from_env();
    
    // Try to connect to database, but don't fail if unavailable
    let pool: Option<Arc<PgPool>> = match PgPool::connect(&config.database_url).await {
        Ok(pool) => {
            tracing::info!("✅ Connected to database");
            // Run migrations if connected
            if let Err(e) = sqlx::migrate!("./migrations").run(&pool).await {
                tracing::warn!("⚠️ Migration warning: {}", e);
            }
            Some(Arc::new(pool))
        }
        Err(e) => {
            tracing::warn!("⚠️ Database not available: {}. Running in limited mode.", e);
            None
        }
    };
    
    // Rate limiter: 100 requests per minute per IP
    let governor_conf = GovernorConfigBuilder::default()
        .seconds_per_request(1)
        .burst_size(100)
        .finish()
        .unwrap();

    let host = config.host.clone();
    let port = config.port;

    tracing::info!("🚀 Server starting on {}:{}", host, port);
    tracing::info!("📚 API documentation available at http://{}:{}/api/health", host, port);

    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                // In production, be more restrictive
                origin.as_bytes().starts_with(b"http://localhost") ||
                origin.as_bytes().starts_with(b"https://")
            })
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);
        
        let mut app = App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::JsonConfig::default()
                .limit(4096 * 1024) // 4MB max JSON payload
                .error_handler(|err, _req| {
                    actix_web::error::InternalError::from_response(
                        err,
                        HttpResponse::BadRequest().json(serde_json::json!({
                            "error": "Invalid JSON payload",
                            "success": false
                        }))
                    ).into()
                }))
            .wrap(cors)
            .wrap(actix_middleware::Logger::new("%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T"))
            .wrap(Governor::new(&governor_conf))
            .wrap(actix_middleware::Compress::default())
            // Security headers
            .wrap(actix_middleware::DefaultHeaders::new()
                .add(("X-Content-Type-Options", "nosniff"))
                .add(("X-Frame-Options", "DENY"))
                .add(("X-XSS-Protection", "1; mode=block"))
                .add(("Referrer-Policy", "strict-origin-when-cross-origin"))
                .add(("Permissions-Policy", "geolocation=(), microphone=(), camera=()"))
            )
            // Health check endpoints
            .route("/health", web::get().to(health_check))
            .route("/api/health", web::get().to(health_check))
            .route("/api/version", web::get().to(version_info));
        
        // Add database pool if available
        if let Some(ref p) = pool {
            app = app.app_data(web::Data::new(p.clone()));
        }
        
        // Configure API routes
        app.configure(routes::auth::configure)
            .configure(routes::ai::configure)
            .configure(routes::robotics::configure)
            .configure(routes::blockchain::configure)
            .configure(routes::dashboard::configure)
            // 404 handler
            .default_service(web::route().to(not_found))
    })
    .bind((host.as_str(), port))?
    .workers(num_cpus::get())
    .run()
    .await
}

/// Health check endpoint
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "RoboVeda API",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

/// Version info endpoint
async fn version_info() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "name": "RoboVeda API",
        "version": env!("CARGO_PKG_VERSION"),
        "rust_version": env!("CARGO_PKG_RUST_VERSION"),
        "api_version": "v1",
        "endpoints": {
            "auth": "/api/auth",
            "ai": "/api/ai",
            "robotics": "/api/robotics",
            "blockchain": "/api/blockchain",
            "dashboard": "/api/dashboard"
        }
    }))
}

/// 404 Not Found handler
async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().json(serde_json::json!({
        "error": "Not Found",
        "message": "The requested resource was not found",
        "success": false
    }))
}
