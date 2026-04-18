use crate::{
    config::AppState,
    models::user::*,
    repositories::UserRepo,
    utils::errors::AppResult,
};
use uuid::Uuid;

pub struct UserService;

impl UserService {
    pub async fn get_profile(state: &AppState, id: Uuid) -> AppResult<UserWithStats> {
        UserRepo::get_profile_with_stats(&state.db, id).await
    }

    pub async fn update_profile(state: &AppState, id: Uuid, req: &UpdateUserRequest) -> AppResult<User> {
        UserRepo::update(&state.db, id, req).await
    }

    pub async fn get_public(state: &AppState, username: &str) -> AppResult<PublicUser> {
        let user = UserRepo::find_by_username(&state.db, username)
            .await?
            .ok_or_else(|| crate::utils::errors::AppError::not_found("User"))?;
        Ok(PublicUser {
            id: user.id,
            username: user.username,
            display_name: user.display_name,
            avatar_url: user.avatar_url,
            bio: user.bio,
            website_url: user.website_url,
            location: user.location,
            created_at: user.created_at,
        })
    }

    pub async fn search(state: &AppState, q: &str, page: i64, per_page: i64) -> AppResult<Vec<PublicUser>> {
        let limit = per_page.min(50);
        let offset = (page - 1) * limit;
        UserRepo::search(&state.db, q, limit, offset).await
    }

    pub async fn delete(state: &AppState, id: Uuid) -> AppResult<()> {
        UserRepo::soft_delete(&state.db, id).await
    }
}
