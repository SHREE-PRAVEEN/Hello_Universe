use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;
use super::DomainEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaUploaded {
    pub media_id: Uuid,
    pub project_id: Option<Uuid>,
    pub uploader_id: Uuid,
    pub filename: String,
    pub media_type: String,
    pub occurred_at: OffsetDateTime,
}

impl DomainEvent for MediaUploaded {
    fn event_type(&self) -> &'static str { "media.uploaded" }
    fn aggregate_id(&self) -> Uuid { self.media_id }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaProcessed {
    pub media_id: Uuid,
    pub occurred_at: OffsetDateTime,
}

impl DomainEvent for MediaProcessed {
    fn event_type(&self) -> &'static str { "media.processed" }
    fn aggregate_id(&self) -> Uuid { self.media_id }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaDeleted {
    pub media_id: Uuid,
    pub deleted_by: Uuid,
    pub occurred_at: OffsetDateTime,
}

impl DomainEvent for MediaDeleted {
    fn event_type(&self) -> &'static str { "media.deleted" }
    fn aggregate_id(&self) -> Uuid { self.media_id }
}
