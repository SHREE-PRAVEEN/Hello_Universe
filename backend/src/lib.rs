use std::sync::Arc;

use axum::Router;
use redis::Client as RedisClient;
use sqlx::PgPool;

pub mod config;
pub mod routes;
pub mod controllers;
pub mod services;
pub mod repositories;
pub mod models;
pub mod middleware;
pub mod jobs;
pub mod events;
pub mod integrations;
pub mod utils;

#[derive(Clone)]
pub struct AppState {
    pub config: config::env::AppConfig,
    pub pg_pool: PgPool,
    pub redis: RedisClient,
}

impl AppState {
    pub fn new(config: config::env::AppConfig, pg_pool: PgPool, redis: RedisClient) -> Self {
        Self {
            config,
            pg_pool,
            redis,
        }
    }
}

pub fn build_app(state: Arc<AppState>) -> Router {
    routes::router(state)
}
