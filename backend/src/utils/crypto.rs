//! Cryptographic utilities for the RoboVeda backend

use sha2::{Sha256, Sha512, Digest};
use rand::Rng;
use base64::{Engine as _, engine::general_purpose};

/// Generate a cryptographically secure random string
pub fn generate_random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// Generate a random hex string
pub fn generate_random_hex(bytes: usize) -> String {
    let mut rng = rand::thread_rng();
    let random_bytes: Vec<u8> = (0..bytes).map(|_| rng.gen()).collect();
    hex::encode(random_bytes)
}

/// Hash data using SHA-256
pub fn sha256_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

/// Hash data using SHA-512
pub fn sha512_hash(data: &[u8]) -> String {
    let mut hasher = Sha512::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

/// Hash a string using SHA-256
pub fn hash_string(input: &str) -> String {
    sha256_hash(input.as_bytes())
}

/// Encode bytes to base64
pub fn base64_encode(data: &[u8]) -> String {
    general_purpose::STANDARD.encode(data)
}

/// Decode base64 to bytes
pub fn base64_decode(input: &str) -> Result<Vec<u8>, base64::DecodeError> {
    general_purpose::STANDARD.decode(input)
}

/// URL-safe base64 encode
pub fn base64_url_encode(data: &[u8]) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(data)
}

/// URL-safe base64 decode
pub fn base64_url_decode(input: &str) -> Result<Vec<u8>, base64::DecodeError> {
    general_purpose::URL_SAFE_NO_PAD.decode(input)
}

/// Generate a secure API key
pub fn generate_api_key() -> String {
    format!("rbv_{}", generate_random_hex(32))
}

/// Generate a secure session token
pub fn generate_session_token() -> String {
    let timestamp = chrono::Utc::now().timestamp_millis();
    let random = generate_random_hex(16);
    let combined = format!("{}:{}", timestamp, random);
    sha256_hash(combined.as_bytes())
}

/// Constant-time string comparison to prevent timing attacks
pub fn secure_compare(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    
    a.bytes()
        .zip(b.bytes())
        .fold(0, |acc, (x, y)| acc | (x ^ y)) == 0
}

/// Mask sensitive data for logging (show first/last n characters)
pub fn mask_sensitive(data: &str, visible_chars: usize) -> String {
    if data.len() <= visible_chars * 2 {
        return "*".repeat(data.len());
    }
    
    let start = &data[..visible_chars];
    let end = &data[data.len() - visible_chars..];
    let masked_len = data.len() - (visible_chars * 2);
    
    format!("{}{}{}",start, "*".repeat(masked_len), end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_string() {
        let s1 = generate_random_string(32);
        let s2 = generate_random_string(32);
        
        assert_eq!(s1.len(), 32);
        assert_eq!(s2.len(), 32);
        assert_ne!(s1, s2);
    }

    #[test]
    fn test_generate_random_hex() {
        let hex = generate_random_hex(16);
        assert_eq!(hex.len(), 32); // 16 bytes = 32 hex chars
        assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_sha256_hash() {
        let hash = sha256_hash(b"hello world");
        assert_eq!(hash.len(), 64);
        assert_eq!(hash, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
    }

    #[test]
    fn test_base64() {
        let data = b"hello world";
        let encoded = base64_encode(data);
        let decoded = base64_decode(&encoded).unwrap();
        assert_eq!(data.to_vec(), decoded);
    }

    #[test]
    fn test_secure_compare() {
        assert!(secure_compare("hello", "hello"));
        assert!(!secure_compare("hello", "world"));
        assert!(!secure_compare("hello", "hello!"));
    }

    #[test]
    fn test_mask_sensitive() {
        assert_eq!(mask_sensitive("1234567890123456", 4), "1234********3456");
        assert_eq!(mask_sensitive("short", 4), "*****");
    }

    #[test]
    fn test_generate_api_key() {
        let key = generate_api_key();
        assert!(key.starts_with("rbv_"));
        assert_eq!(key.len(), 68); // "rbv_" + 64 hex chars
    }
}