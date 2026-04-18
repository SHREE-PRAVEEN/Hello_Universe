use crate::{
    middleware::auth::AuthUser,
    utils::errors::{AppError, AppResult},
};

/// Require user to have one of the given roles
pub fn require_role(user: &AuthUser, roles: &[&str]) -> AppResult<()> {
    if user.is_admin() {
        return Ok(()); // admins bypass all role checks
    }
    if roles.iter().any(|r| user.has_role(r)) {
        Ok(())
    } else {
        Err(AppError::Forbidden(format!(
            "Required role: one of {:?}",
            roles
        )))
    }
}

/// Require user to have a specific permission
pub fn require_permission(user: &AuthUser, permission: &str) -> AppResult<()> {
    if user.is_admin() {
        return Ok(());
    }
    if user.has_permission(permission) {
        Ok(())
    } else {
        Err(AppError::Forbidden(format!(
            "Required permission: {}",
            permission
        )))
    }
}

pub fn require_admin(user: &AuthUser) -> AppResult<()> {
    require_role(user, &["admin"])
}

pub fn require_moderator(user: &AuthUser) -> AppResult<()> {
    require_role(user, &["admin", "moderator"])
}

pub fn require_developer(user: &AuthUser) -> AppResult<()> {
    require_role(user, &["admin", "developer"])
}
