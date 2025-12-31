use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use crate::errors::{ApiError, ApiResult};

/// Blockchain/Crypto service for handling Web3 operations
pub struct BlockchainService {
    provider_url: String,
    contract_address: Option<String>,
}

impl BlockchainService {
    pub fn new() -> Self {
        Self {
            provider_url: std::env::var("WEB3_PROVIDER_URL")
                .unwrap_or_else(|_| "https://mainnet.infura.io/v3/YOUR_KEY".to_string()),
            contract_address: std::env::var("CONTRACT_ADDRESS").ok(),
        }
    }

    /// Check if blockchain service is configured
    pub fn is_configured(&self) -> bool {
        !self.provider_url.contains("YOUR_KEY") && self.contract_address.is_some()
    }

    /// Verify wallet signature (EIP-191)
    pub fn verify_signature(&self, message: &str, signature: &str, address: &str) -> ApiResult<bool> {
        // In production, use ethers-rs or web3 crate for proper verification
        // This is a simplified placeholder
        
        if signature.len() != 132 || !signature.starts_with("0x") {
            return Err(ApiError::ValidationError("Invalid signature format".to_string()));
        }
        
        if !Self::is_valid_eth_address(address) {
            return Err(ApiError::ValidationError("Invalid Ethereum address".to_string()));
        }

        // Placeholder: In production, implement proper ECDSA recovery
        log::info!("Verifying signature for address: {}", address);
        Ok(true)
    }

    /// Validate Ethereum address format
    pub fn is_valid_eth_address(address: &str) -> bool {
        if !address.starts_with("0x") {
            return false;
        }
        let hex_part = &address[2..];
        hex_part.len() == 40 && hex_part.chars().all(|c| c.is_ascii_hexdigit())
    }

    /// Generate message for wallet signature
    pub fn generate_sign_message(nonce: &str) -> String {
        format!(
            "Welcome to RoboVeda!\n\n\
            Click to sign in and accept the Terms of Service.\n\n\
            This request will not trigger a blockchain transaction or cost any gas fees.\n\n\
            Nonce: {}",
            nonce
        )
    }

    /// Generate a random nonce for signature verification
    pub fn generate_nonce() -> String {
        use rand::Rng;
        let nonce: u64 = rand::thread_rng().gen();
        format!("{:016x}", nonce)
    }

    /// Hash data using SHA256
    pub fn hash_sha256(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    /// Verify transaction on blockchain (placeholder)
    pub async fn verify_transaction(&self, tx_hash: &str) -> ApiResult<TransactionStatus> {
        if !tx_hash.starts_with("0x") || tx_hash.len() != 66 {
            return Err(ApiError::ValidationError("Invalid transaction hash format".to_string()));
        }

        // In production, query the blockchain
        log::info!("Verifying transaction: {}", tx_hash);
        
        Ok(TransactionStatus {
            hash: tx_hash.to_string(),
            status: "pending".to_string(),
            confirmations: 0,
            block_number: None,
        })
    }

    /// Get token balance for address (placeholder)
    pub async fn get_token_balance(&self, address: &str) -> ApiResult<TokenBalance> {
        if !Self::is_valid_eth_address(address) {
            return Err(ApiError::ValidationError("Invalid Ethereum address".to_string()));
        }

        // In production, query the blockchain/contract
        Ok(TokenBalance {
            address: address.to_string(),
            balance: "0".to_string(),
            symbol: "RBV".to_string(),
            decimals: 18,
        })
    }
}

impl Default for BlockchainService {
    fn default() -> Self {
        Self::new()
    }
}

// Response types
#[derive(Debug, Serialize)]
pub struct TransactionStatus {
    pub hash: String,
    pub status: String,
    pub confirmations: u32,
    pub block_number: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct TokenBalance {
    pub address: String,
    pub balance: String,
    pub symbol: String,
    pub decimals: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletVerification {
    pub address: String,
    pub message: String,
    pub nonce: String,
}

#[derive(Debug, Deserialize)]
pub struct SignatureVerifyRequest {
    pub address: String,
    pub message: String,
    pub signature: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_eth_address() {
        assert!(BlockchainService::is_valid_eth_address("0x742d35Cc6634C0532925a3b844Bc9e7595f5E4E1"));
        assert!(!BlockchainService::is_valid_eth_address("0x742d35")); // Too short
        assert!(!BlockchainService::is_valid_eth_address("742d35Cc6634C0532925a3b844Bc9e7595f5E4E1")); // No 0x
        assert!(!BlockchainService::is_valid_eth_address("0x742d35Cc6634C0532925a3b844Bc9e7595f5E4EG")); // Invalid hex
    }

    #[test]
    fn test_generate_nonce() {
        let nonce1 = BlockchainService::generate_nonce();
        let nonce2 = BlockchainService::generate_nonce();
        assert_eq!(nonce1.len(), 16);
        assert_ne!(nonce1, nonce2);
    }

    #[test]
    fn test_hash_sha256() {
        let hash = BlockchainService::hash_sha256(b"hello world");
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_generate_sign_message() {
        let nonce = "abc123";
        let message = BlockchainService::generate_sign_message(nonce);
        assert!(message.contains(nonce));
        assert!(message.contains("RoboVeda"));
    }
}
