use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sha2::{Digest, Sha256};
use crate::utils::errors::{AppError, AppResult};
 
/// Hash a password using Argon2id
pub fn hash_password(password: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Password hash error: {}", e)))
}
 
/// Verify a password against an Argon2 hash
pub fn verify_password(password: &str, hash: &str) -> AppResult<bool> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Hash parse error: {}", e)))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
 
/// SHA-256 hash of a file or arbitrary bytes
pub fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}
 
/// BLAKE3 hash (fast, for content verification)
pub fn blake3_hex(data: &[u8]) -> String {
    let hash = blake3::hash(data);
    hash.to_hex().to_string()
}
 
/// Generate a secure random token (hex string, 32 bytes = 64 chars)
pub fn generate_secure_token() -> String {
    use rand::RngCore;
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    hex::encode(bytes)
}
 
/// Hash a token for safe DB storage
pub fn hash_token(token: &str) -> String {
    sha256_hex(token.as_bytes())
}
 