use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web::http::header::AUTHORIZATION;
use std::future::{Ready, ready};
use uuid::Uuid;
use crate::errors::ApiError;
use crate::utils::jwt::{verify_token, Claims};

/// Authenticated user information extracted from JWT
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub claims: Claims,
}

/// Extractor for authenticated requests
/// Usage: pub async fn handler(user: AuthenticatedUser) -> impl Responder
impl actix_web::FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        // Try to get from request extensions first (if middleware already validated)
        if let Some(user) = req.extensions().get::<AuthenticatedUser>() {
            return ready(Ok(user.clone()));
        }

        // Otherwise, extract from Authorization header
        let auth_header = req.headers().get(AUTHORIZATION);
        
        let token = match auth_header {
            Some(header_value) => {
                match header_value.to_str() {
                    Ok(auth_str) if auth_str.starts_with("Bearer ") => &auth_str[7..],
                    _ => return ready(Err(ApiError::Unauthorized("Invalid authorization header format".to_string()).into())),
                }
            }
            None => return ready(Err(ApiError::Unauthorized("Missing authorization header".to_string()).into())),
        };

        // Get JWT secret from environment
        let secret = match std::env::var("JWT_SECRET") {
            Ok(s) => s,
            Err(_) => return ready(Err(ApiError::InternalError("JWT secret not configured".to_string()).into())),
        };

        // Verify token
        match verify_token(token, &secret) {
            Ok(claims) => {
                match Uuid::parse_str(&claims.sub) {
                    Ok(user_id) => {
                        ready(Ok(AuthenticatedUser { user_id, claims }))
                    }
                    Err(_) => ready(Err(ApiError::InvalidToken("Invalid user ID in token".to_string()).into())),
                }
            }
            Err(e) => ready(Err(ApiError::InvalidToken(e.to_string()).into())),
        }
    }
}

/// Optional authentication - doesn't fail if no token provided
#[derive(Debug, Clone)]
pub struct OptionalUser(pub Option<AuthenticatedUser>);

impl actix_web::FromRequest for OptionalUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let auth_header = req.headers().get(AUTHORIZATION);
        
        if auth_header.is_none() {
            return ready(Ok(OptionalUser(None)));
        }

        let token = match auth_header {
            Some(header_value) => {
                match header_value.to_str() {
                    Ok(auth_str) if auth_str.starts_with("Bearer ") => &auth_str[7..],
                    _ => return ready(Ok(OptionalUser(None))),
                }
            }
            None => return ready(Ok(OptionalUser(None))),
        };

        let secret = match std::env::var("JWT_SECRET") {
            Ok(s) => s,
            Err(_) => return ready(Ok(OptionalUser(None))),
        };

        match verify_token(token, &secret) {
            Ok(claims) => {
                match Uuid::parse_str(&claims.sub) {
                    Ok(user_id) => ready(Ok(OptionalUser(Some(AuthenticatedUser { user_id, claims })))),
                    Err(_) => ready(Ok(OptionalUser(None))),
                }
            }
            Err(_) => ready(Ok(OptionalUser(None))),
        }
    }
}

/// Admin-only authentication extractor
#[derive(Debug, Clone)]
pub struct AdminUser(pub AuthenticatedUser);

impl actix_web::FromRequest for AdminUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
        match AuthenticatedUser::from_request(req, payload).into_inner() {
            Ok(user) => {
                // Check if user has admin role (you can customize this logic)
                if user.claims.role.as_deref() == Some("admin") {
                    ready(Ok(AdminUser(user)))
                } else {
                    ready(Err(ApiError::Forbidden("Admin access required".to_string()).into()))
                }
            }
            Err(e) => ready(Err(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authenticated_user_clone() {
        let claims = Claims {
            sub: Uuid::new_v4().to_string(),
            exp: 0,
            iat: 0,
            role: None,
        };
        let user = AuthenticatedUser {
            user_id: Uuid::new_v4(),
            claims,
        };
        let _cloned = user.clone();
    }
}
