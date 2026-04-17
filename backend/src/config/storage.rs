#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageBackend {
    S3,
    Gcs,
    Ipfs,
}

impl StorageBackend {
    pub fn parse(value: &str) -> Self {
        match value.to_ascii_lowercase().as_str() {
            "gcs" => Self::Gcs,
            "ipfs" => Self::Ipfs,
            _ => Self::S3,
        }
    }
}
