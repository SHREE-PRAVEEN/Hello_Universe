use crate::{
    config::AppState,
    models::blockchain::*,
    utils::{crypto::{sha256_hex, blake3_hex}, errors::AppResult},
};
use time::OffsetDateTime;
use uuid::Uuid;

pub struct BlockchainService;

impl BlockchainService {
    /// Compute and store SHA-256 + BLAKE3 hashes for an entity
    pub async fn hash_entity(
        state: &AppState,
        entity_type: &str,
        entity_id: Uuid,
        data: &[u8],
        computed_by: Uuid,
    ) -> AppResult<ContentHash> {
        let hash = sha256_hex(data);

        Ok(sqlx::query_as!(
            ContentHash,
            r#"INSERT INTO content_hashes (entity_type, entity_id, hash_algorithm, hash_value, computed_by)
               VALUES ($1, $2, 'sha256', $3, $4)
               ON CONFLICT (entity_type, entity_id, hash_algorithm)
               DO UPDATE SET hash_value = EXCLUDED.hash_value, computed_at = NOW()
               RETURNING id, entity_type, entity_id, hash_algorithm, hash_value, computed_at, computed_by"#,
            entity_type, entity_id, hash, computed_by
        )
        .fetch_one(&state.db)
        .await?)
    }

    /// Verify file integrity by re-computing hash against stored record
    pub async fn verify(
        state: &AppState,
        entity_type: &str,
        entity_id: Uuid,
        data: &[u8],
    ) -> AppResult<VerificationResult> {
        let stored = sqlx::query!(
            "SELECT hash_value FROM content_hashes WHERE entity_type=$1 AND entity_id=$2 AND hash_algorithm='sha256'",
            entity_type, entity_id
        )
        .fetch_optional(&state.db)
        .await?;

        let computed = sha256_hex(data);
        let hash_matches = stored.as_ref().map(|r| r.hash_value == computed).unwrap_or(false);

        let ipfs = sqlx::query!(
            "SELECT cid, pin_status FROM ipfs_records WHERE entity_id=$1",
            entity_id
        )
        .fetch_optional(&state.db)
        .await?;

        let blockchain_tx = sqlx::query!(
            "SELECT tx_hash, status FROM blockchain_transactions WHERE entity_id=$1 ORDER BY created_at DESC LIMIT 1",
            entity_id
        )
        .fetch_optional(&state.db)
        .await?;

        Ok(VerificationResult {
            entity_type: entity_type.to_string(),
            entity_id,
            hash_matches,
            ipfs_pinned: ipfs.as_ref().map(|r| r.pin_status == "pinned").unwrap_or(false),
            blockchain_confirmed: blockchain_tx.as_ref().map(|r| r.status == "confirmed").unwrap_or(false),
            content_hash: stored.map(|r| r.hash_value),
            ipfs_cid: ipfs.map(|r| r.cid),
            blockchain_tx: blockchain_tx.map(|r| r.tx_hash),
            verified_at: OffsetDateTime::now_utc(),
        })
    }

    /// Record IPFS CID for an asset
    pub async fn record_ipfs(
        state: &AppState,
        media_id: Uuid,
        cid: &str,
        gateway_url: &str,
    ) -> AppResult<IpfsRecord> {
        Ok(sqlx::query_as!(
            IpfsRecord,
            r#"INSERT INTO ipfs_records (media_file_id, cid, pin_status, gateway_url, pinned_at)
               VALUES ($1, $2, 'pinned', $3, NOW())
               ON CONFLICT (cid) DO UPDATE SET pin_status = 'pinned', pinned_at = NOW()
               RETURNING id, media_file_id, entity_type, entity_id, cid, pin_status, gateway_url, pinned_at, created_at"#,
            media_id, cid, gateway_url
        )
        .fetch_one(&state.db)
        .await?)
    }

    /// Record a blockchain transaction anchor
    pub async fn record_transaction(
        state: &AppState,
        entity_type: &str,
        entity_id: Uuid,
        network: &str,
        tx_hash: &str,
        payload_hash: &str,
        created_by: Uuid,
    ) -> AppResult<BlockchainTransaction> {
        Ok(sqlx::query_as!(
            BlockchainTransaction,
            r#"INSERT INTO blockchain_transactions
               (entity_type, entity_id, network, tx_hash, payload_hash, created_by, status)
               VALUES ($1,$2,$3,$4,$5,$6,'pending')
               RETURNING id, entity_type, entity_id, network, tx_hash, block_number,
               block_hash, contract_address, payload_hash, status, confirmed_at, created_by, created_at"#,
            entity_type, entity_id, network, tx_hash, payload_hash, created_by
        )
        .fetch_one(&state.db)
        .await?)
    }

    /// Record ownership
    pub async fn record_ownership(
        state: &AppState,
        entity_type: &str,
        entity_id: Uuid,
        owner_user_id: Option<Uuid>,
        owner_org_id: Option<Uuid>,
    ) -> AppResult<OwnershipRecord> {
        Ok(sqlx::query_as!(
            OwnershipRecord,
            r#"INSERT INTO ownership_records (entity_type, entity_id, owner_user_id, owner_org_id)
               VALUES ($1,$2,$3,$4)
               RETURNING id, entity_type, entity_id, owner_user_id, owner_org_id,
               transferred_from_id, transfer_reason, metadata, created_at"#,
            entity_type, entity_id, owner_user_id as _, owner_org_id as _
        )
        .fetch_one(&state.db)
        .await?)
    }
}
