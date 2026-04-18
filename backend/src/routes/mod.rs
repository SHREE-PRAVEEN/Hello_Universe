use axum::{
    middleware,
    routing::{delete, get, patch, post, put},
    Router,
};
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use crate::{
    config::AppState,
    controllers::{
        ai_ctrl, analytics_ctrl, auth_ctrl, blockchain_ctrl,
        commerce_ctrl, engagement_ctrl, media_ctrl, moderation_ctrl,
        org_ctrl, project_ctrl, search_ctrl, user_ctrl,
    },
    middleware::{
        rate_limit::rate_limit_middleware,
        request_id::request_id_middleware,
    },
};

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        // ---- Health ----
        .route("/health", get(health))
        .route("/ready",  get(ready))
        // ---- API v1 ----
        .nest("/api/v1", api_v1_router())
        // ---- Global middleware ----
        .layer(middleware::from_fn_with_state(state.clone(), rate_limit_middleware))
        .layer(middleware::from_fn(request_id_middleware))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(cors)
        .with_state(state)
}

fn api_v1_router() -> Router<AppState> {
    Router::new()
        .nest("/auth",        auth_routes())
        .nest("/users",       user_routes())
        .nest("/organizations", org_routes())
        .nest("/projects",    project_routes())
        .nest("/media",       media_routes())
        .nest("/search",      search_routes())
        .nest("/ai",          ai_routes())
        .nest("/blockchain",  blockchain_routes())
        .nest("/commerce",    commerce_routes())
        .nest("/analytics",   analytics_routes())
        .nest("/engagement",  engagement_routes())
        .nest("/admin",       admin_routes())
}

// ---- Auth ----
fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/register",              post(auth_ctrl::register))
        .route("/login",                 post(auth_ctrl::login))
        .route("/refresh",               post(auth_ctrl::refresh))
        .route("/logout",                post(auth_ctrl::logout))
        .route("/logout-all",            post(auth_ctrl::logout_all))
        .route("/verify-email",          get(auth_ctrl::verify_email))
        .route("/request-password-reset",post(auth_ctrl::request_password_reset))
        .route("/reset-password",        post(auth_ctrl::reset_password))
        .route("/me",                    get(auth_ctrl::me))
}

// ---- Users ----
fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/me",                    get(user_ctrl::get_me).patch(user_ctrl::update_me))
        .route("/me/delete",             delete(user_ctrl::delete_account))
        .route("/search",                get(user_ctrl::search_users))
        .route("/:username",             get(user_ctrl::get_profile))
        .route("/id/:id",                get(user_ctrl::get_by_id))
}

// ---- Organizations ----
fn org_routes() -> Router<AppState> {
    Router::new()
        .route("/",                               post(org_ctrl::create))
        .route("/mine",                           get(org_ctrl::my_organizations))
        .route("/:slug",                          get(org_ctrl::get).patch(org_ctrl::update))
        .route("/:org_id/members",               get(org_ctrl::list_members).post(org_ctrl::add_member))
        .route("/:org_id/members/:user_id",      delete(org_ctrl::remove_member))
}

// ---- Projects ----
fn project_routes() -> Router<AppState> {
    Router::new()
        .route("/",                               get(project_ctrl::list).post(project_ctrl::create))
        .route("/categories",                     get(project_ctrl::list_categories))
        .route("/top-downloads",                  get(project_ctrl::top_downloads))
        .route("/:slug",                          get(project_ctrl::get))
        .route("/:id/update",                     patch(project_ctrl::update))
        .route("/:id/submit",                     post(project_ctrl::submit_for_review))
        .route("/:id/publish",                    post(project_ctrl::publish))
        .route("/:id/archive",                    post(project_ctrl::archive))
        .route("/:id/versions",                   get(project_ctrl::get_versions).post(project_ctrl::create_version))
        .route("/:id/collaborators",              post(project_ctrl::add_collaborator))
        .route("/:id/media",                      get(media_ctrl::list_for_project))
}

// ---- Media ----
fn media_routes() -> Router<AppState> {
    Router::new()
        .route("/upload",                         post(media_ctrl::upload))
        .route("/presign",                        post(media_ctrl::presign))
        .route("/:id",                            get(media_ctrl::get).delete(media_ctrl::delete))
        .route("/:id/download",                   get(media_ctrl::download_url))
}

// ---- Search ----
fn search_routes() -> Router<AppState> {
    Router::new()
        .route("/",                               get(search_ctrl::search))
        .route("/autocomplete",                   get(search_ctrl::autocomplete))
        .route("/popular",                        get(search_ctrl::popular))
}

// ---- AI ----
fn ai_routes() -> Router<AppState> {
    Router::new()
        .route("/tags/:entity_type/:entity_id",   get(ai_ctrl::get_tags))
        .route("/tag/project/:id",                post(ai_ctrl::tag_project))
        .route("/tag/media/:id",                  post(ai_ctrl::tag_media))
}

// ---- Blockchain ----
fn blockchain_routes() -> Router<AppState> {
    Router::new()
        .route("/verify/:entity_type/:entity_id", post(blockchain_ctrl::verify))
        .route("/ownership",                      post(blockchain_ctrl::record_ownership))
        .route("/anchor",                         post(blockchain_ctrl::anchor_transaction))
}

// ---- Commerce ----
fn commerce_routes() -> Router<AppState> {
    Router::new()
        .route("/plans",                          get(commerce_ctrl::list_plans))
        .route("/subscriptions/me",               get(commerce_ctrl::get_my_subscription))
        .route("/subscriptions",                  post(commerce_ctrl::subscribe))
        .route("/subscriptions/cancel",           post(commerce_ctrl::cancel_subscription))
        .route("/purchases",                      post(commerce_ctrl::initiate_purchase).get(commerce_ctrl::my_purchases))
        .route("/purchases/:id/complete",         post(commerce_ctrl::complete_purchase))
        .route("/access",                         get(commerce_ctrl::check_access))
        .route("/revenue-report",                 get(commerce_ctrl::revenue_report))
}

// ---- Analytics ----
fn analytics_routes() -> Router<AppState> {
    Router::new()
        .route("/view",                           post(analytics_ctrl::track_view))
        .route("/download",                       post(analytics_ctrl::track_download))
        .route("/projects/:id/stats",             get(analytics_ctrl::project_stats))
        .route("/engagement/:entity_id",          get(analytics_ctrl::engagement))
        .route("/top-projects",                   get(analytics_ctrl::top_projects))
}

// ---- Engagement ----
fn engagement_routes() -> Router<AppState> {
    Router::new()
        .route("/reviews",                                    post(engagement_ctrl::create_review))
        .route("/reviews/:entity_type/:entity_id",            get(engagement_ctrl::get_reviews))
        .route("/comments",                                   post(engagement_ctrl::create_comment))
        .route("/comments/:entity_type/:entity_id",           get(engagement_ctrl::get_comments))
        .route("/comments/:id",                               delete(engagement_ctrl::delete_comment))
        .route("/favorites",                                  post(engagement_ctrl::toggle_favorite).get(engagement_ctrl::is_favorited))
        .route("/follows",                                    post(engagement_ctrl::toggle_follow))
        .route("/reports",                                    post(engagement_ctrl::create_report))
}

// ---- Admin / Moderation ----
fn admin_routes() -> Router<AppState> {
    Router::new()
        .route("/moderation/queue",               get(moderation_ctrl::list_queue))
        .route("/moderation/queue/:id/decide",    post(moderation_ctrl::decide))
        .route("/notifications",                  get(moderation_ctrl::my_notifications))
        .route("/notifications/:id/read",         post(moderation_ctrl::mark_notification_read))
        .route("/users/:id",                      delete(user_ctrl::admin_delete_user))
}

// ---- Health handlers ----
async fn health() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({ "status": "ok" }))
}

async fn ready() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({ "status": "ready" }))
}
