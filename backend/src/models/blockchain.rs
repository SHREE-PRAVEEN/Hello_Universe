use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ContentHash {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub hash_algorithm: String,
    pub hash_value: String,
    pub computed_at: OffsetDateTime,
    pub computed_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IpfsRecord {
    pub id: Uuid,
    pub media_file_id: Option<Uuid>,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub cid: String,
    pub pin_status: String,
    pub gateway_url: Option<String>,
    pub pinned_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OwnershipRecord {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub owner_user_id: Option<Uuid>,
    pub owner_org_id: Option<Uuid>,
    pub transferred_from_id: Option<Uuid>,
    pub transfer_reason: Option<String>,
    pub metadata: serde_json::Value,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BlockchainTransaction {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub network: String,
    pub tx_hash: String,
    pub block_number: Option<i64>,
    pub block_hash: Option<String>,
    pub contract_address: Option<String>,
    pub payload_hash: Option<String>,
    pub status: String,
    pub confirmed_at: Option<OffsetDateTime>,
    pub created_by: Option<Uuid>,
    pub created_at: OffsetDateTime,
}

// ---- DTOs ----

#[derive(Debug, Deserialize)]
pub struct AnchorToBlockchainRequest {
    pub entity_type: String,
    pub entity_id: Uuid,
    pub network: String,
}

#[derive(Debug, Serialize)]
pub struct VerificationResult {
    pub entity_type: String,
    pub entity_id: Uuid,
    pub hash_matches: bool,
    pub ipfs_pinned: bool,
    pub blockchain_confirmed: bool,
    pub content_hash: Option<String>,
    pub ipfs_cid: Option<String>,
    pub blockchain_tx: Option<String>,
    pub verified_at: OffsetDateTime,
}

#[derive(Debug, Deserialize)]
pub struct TransferOwnershipRequest {
    pub entity_type: String,
    pub entity_id: Uuid,
    pub to_user_id: Option<Uuid>,
    pub to_org_id: Option<Uuid>,
    pub reason: Option<String>,
}
