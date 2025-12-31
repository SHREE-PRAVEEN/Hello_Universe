use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use actix_web::HttpRequest;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,      // user_id
    pub exp: i64,         // expiration timestamp
    pub iat: i64,         // issued at timestamp
    pub role: Option<String>, // user role (admin, user, etc.)
}

/// Create a JWT token for a user
pub fn create_token(user_id: &str, secret: &str, expiration_seconds: i64) -> Result<String, jsonwebtoken::errors::Error> {
    create_token_with_role(user_id, secret, expiration_seconds, None)
}

/// Create a JWT token with an optional role
pub fn create_token_with_role(
    user_id: &str, 
    secret: &str, 
    expiration_seconds: i64,
    role: Option<&str>
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let claims = Claims {
        sub: user_id.to_owned(),
        iat: now.timestamp(),
        exp: (now + Duration::seconds(expiration_seconds)).timestamp(),
        role: role.map(String::from),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

/// Verify and decode a JWT token
pub fn verify_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let mut validation = Validation::default();
    validation.leeway = 60; // Allow 60 seconds clock skew
    
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )
    .map(|data| data.claims)
}

/// Extract user ID from Authorization header in request
pub fn extract_user_id_from_request(req: &HttpRequest) -> Option<Uuid> {
    let auth_header = req.headers().get("Authorization")?;
    let auth_str = auth_header.to_str().ok()?;
    
    if !auth_str.starts_with("Bearer ") {
        return None;
    }
    
    let token = &auth_str[7..];
    let secret = std::env::var("JWT_SECRET").ok()?;
    let claims = verify_token(token, &secret).ok()?;
    
    Uuid::parse_str(&claims.sub).ok()
}

/// Extract full claims from request
pub fn extract_claims_from_request(req: &HttpRequest) -> Option<Claims> {
    let auth_header = req.headers().get("Authorization")?;
    let auth_str = auth_header.to_str().ok()?;
    
    if !auth_str.starts_with("Bearer ") {
        return None;
    }
    
    let token = &auth_str[7..];
    let secret = std::env::var("JWT_SECRET").ok()?;
    verify_token(token, &secret).ok()
}

/// Check if a token is still valid (not expired)
pub fn is_token_valid(token: &str, secret: &str) -> bool {
    verify_token(token, secret).is_ok()
}

/// Get remaining time until token expiration in seconds
pub fn token_expires_in(token: &str, secret: &str) -> Option<i64> {
    let claims = verify_token(token, secret).ok()?;
    let now = Utc::now().timestamp();
    Some(claims.exp - now)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_verify_token() {
        let user_id = Uuid::new_v4().to_string();
        let secret = "test_secret_key_12345";
        
        let token = create_token(&user_id, secret, 3600).unwrap();
        let claims = verify_token(&token, secret).unwrap();
        
        assert_eq!(claims.sub, user_id);
        assert!(claims.exp > Utc::now().timestamp());
    }

    #[test]
    fn test_create_token_with_role() {
        let user_id = Uuid::new_v4().to_string();
        let secret = "test_secret_key_12345";
        
        let token = create_token_with_role(&user_id, secret, 3600, Some("admin")).unwrap();
        let claims = verify_token(&token, secret).unwrap();
        
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.role, Some("admin".to_string()));
    }

    #[test]
    fn test_expired_token() {
        let user_id = Uuid::new_v4().to_string();
        let secret = "test_secret_key_12345";
        
        // Create token that expired 1 hour ago
        let token = create_token(&user_id, secret, -3600).unwrap();
        let result = verify_token(&token, secret);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_secret() {
        let user_id = Uuid::new_v4().to_string();
        let secret = "test_secret_key_12345";
        let wrong_secret = "wrong_secret";
        
        let token = create_token(&user_id, secret, 3600).unwrap();
        let result = verify_token(&token, wrong_secret);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_is_token_valid() {
        let user_id = Uuid::new_v4().to_string();
        let secret = "test_secret_key_12345";
        
        let valid_token = create_token(&user_id, secret, 3600).unwrap();
        assert!(is_token_valid(&valid_token, secret));
        
        let expired_token = create_token(&user_id, secret, -3600).unwrap();
        assert!(!is_token_valid(&expired_token, secret));
    }
}
