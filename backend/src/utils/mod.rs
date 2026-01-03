pub mod crypto;
pub mod jwt;
pub mod logger;
pub mod verification;

// Re-export commonly used items
pub use crypto::{
    generate_random_string,
    generate_random_hex,
    sha256_hash,
    hash_string,
    base64_encode,
    base64_decode,
    generate_api_key,
    secure_compare,
    mask_sensitive,
};

pub use jwt::{
    create_token,
    create_token_with_role,
    verify_token,
    extract_user_id_from_request,
    extract_claims_from_request,
    is_token_valid,
    Claims,
};

pub use verification::{
    generate_verification_token,
    get_token_expiration,
    create_verification_email,
};

pub use logger::{
    RequestTimer,
    log_auth_event,
    log_db_operation,
    log_external_api,
    log_security_event,
    log_device_event,
    log_blockchain_event,
};
