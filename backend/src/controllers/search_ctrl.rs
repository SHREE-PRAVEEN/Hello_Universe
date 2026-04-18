use axum::{extract::{Query, State}, Json};
use crate::{
    config::AppState,
    middleware::OptionalAuthUser,
    models::ai::{SearchRequest, SearchResponse},
    services::SearchService,
    utils::errors::AppResult,
};

pub async fn search(
    State(state): State<AppState>,
    user: OptionalAuthUser,
    Query(req): Query<SearchRequest>,
) -> AppResult<Json<SearchResponse>> {
    let result = SearchService::search(&state, &req, user.user_id()).await?;
    Ok(Json(result))
}

pub async fn autocomplete(
    State(state): State<AppState>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> AppResult<Json<Vec<String>>> {
    let prefix = params.get("q").map(|s| s.as_str()).unwrap_or("");
    let suggestions = SearchService::autocomplete(&state, prefix).await?;
    Ok(Json(suggestions))
}

pub async fn popular(
    State(state): State<AppState>,
) -> AppResult<Json<Vec<String>>> {
    let queries = SearchService::popular_searches(&state).await?;
    Ok(Json(queries))
}
