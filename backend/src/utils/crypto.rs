use sha2::{Digest, Sha256};

use crate::utils::errors::{AppError, AppResult};

pub fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

pub fn blake3_hex(data: &[u8]) -> String {
    blake3::hash(data).to_hex().to_string()
}

pub fn bcrypt_hash(password: &str) -> AppResult<String> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|e| AppError::Internal(format!("bcrypt hash failed: {e}")))
}

pub fn bcrypt_verify(password: &str, hash: &str) -> AppResult<bool> {
    bcrypt::verify(password, hash)
        .map_err(|e| AppError::Internal(format!("bcrypt verify failed: {e}")))
}
