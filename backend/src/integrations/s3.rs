use aws_sdk_s3::{
    Client,
    config::{Credentials, Region},
    presigning::PresigningConfig,
    primitives::ByteStream,
};
use std::time::Duration;
use crate::config::AppState;

pub struct S3Client {
    client: Client,
    bucket: String,
    cdn_url: String,
}

impl S3Client {
    pub fn new(state: &AppState) -> Self {
        let credentials = Credentials::new(
            &state.config.aws_access_key_id,
            &state.config.aws_secret_access_key,
            None, None, "static",
        );

        let config = aws_config::SdkConfig::builder()
            .credentials_provider(credentials)
            .region(Region::new(state.config.aws_region.clone()))
            .build();

        Self {
            client: Client::new(&config),
            bucket: state.config.aws_s3_bucket.clone(),
            cdn_url: state.config.aws_s3_cdn_url.clone(),
        }
    }

    /// Upload bytes to S3 and return the CDN URL
    pub async fn upload(&self, key: &str, data: Vec<u8>, content_type: &str) -> anyhow::Result<String> {
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(ByteStream::from(data))
            .content_type(content_type)
            .send()
            .await?;

        Ok(format!("{}/{}", self.cdn_url, key))
    }

    /// Generate a presigned PUT URL (client-side direct upload)
    pub async fn presign_put(&self, key: &str, content_type: &str, expires_secs: u64) -> anyhow::Result<String> {
        let config = PresigningConfig::expires_in(Duration::from_secs(expires_secs))?;
        let presigned = self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .content_type(content_type)
            .presigned(config)
            .await?;
        Ok(presigned.uri().to_string())
    }

    /// Generate a presigned GET URL (private content download)
    pub async fn presign_get(&self, key: &str, expires_secs: u64) -> anyhow::Result<String> {
        let config = PresigningConfig::expires_in(Duration::from_secs(expires_secs))?;
        let presigned = self.client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .presigned(config)
            .await?;
        Ok(presigned.uri().to_string())
    }

    /// Delete an object
    pub async fn delete(&self, key: &str) -> anyhow::Result<()> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await?;
        Ok(())
    }
}
