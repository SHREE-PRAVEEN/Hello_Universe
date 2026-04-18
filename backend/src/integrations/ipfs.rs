use crate::utils::errors::{AppError, AppResult};
use serde::{Deserialize, Serialize};

pub struct IpfsClient {
    api_url: String,
    jwt: String,
    http: reqwest::Client,
}

#[derive(Deserialize)]
struct PinataResponse {
    #[serde(rename = "IpfsHash")]
    ipfs_hash: String,
}

impl IpfsClient {
    pub fn new(api_url: &str, jwt: &str) -> Self {
        Self {
            api_url: api_url.to_string(),
            jwt: jwt.to_string(),
            http: reqwest::Client::new(),
        }
    }

    /// Pin file bytes to IPFS via Pinata and return CID + gateway URL
    pub async fn pin_bytes(&self, data: Vec<u8>, filename: &str) -> AppResult<(String, String)> {
        let part = reqwest::multipart::Part::bytes(data)
            .file_name(filename.to_string())
            .mime_str("application/octet-stream")
            .map_err(|e| AppError::Internal(anyhow::anyhow!("IPFS part error: {}", e)))?;

        let form = reqwest::multipart::Form::new().part("file", part);

        let resp = self.http
            .post(format!("{}/pinning/pinFileToIPFS", self.api_url))
            .bearer_auth(&self.jwt)
            .multipart(form)
            .send()
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("IPFS pin failed: {}", e)))?;

        if !resp.status().is_success() {
            let err = resp.text().await.unwrap_or_default();
            return Err(AppError::Internal(anyhow::anyhow!("IPFS error: {}", err)));
        }

        let pinata: PinataResponse = resp.json().await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("IPFS parse error: {}", e)))?;

        let cid = pinata.ipfs_hash;
        let gateway_url = format!("https://gateway.pinata.cloud/ipfs/{}", cid);

        Ok((cid, gateway_url))
    }

    /// Pin JSON metadata to IPFS
    pub async fn pin_json(&self, metadata: &serde_json::Value) -> AppResult<String> {
        #[derive(Serialize)]
        struct PinJsonRequest { pinataContent: serde_json::Value }

        let body = PinJsonRequest { pinataContent: metadata.clone() };
        let resp = self.http
            .post(format!("{}/pinning/pinJSONToIPFS", self.api_url))
            .bearer_auth(&self.jwt)
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("IPFS JSON pin failed: {}", e)))?;

        let pinata: PinataResponse = resp.json().await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("IPFS JSON parse: {}", e)))?;

        Ok(pinata.ipfs_hash)
    }
}
