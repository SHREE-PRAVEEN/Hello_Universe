use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;
use super::DomainEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectCreated {
    pub project_id: Uuid,
    pub owner_id: Uuid,
    pub title: String,
    pub occurred_at: OffsetDateTime,
}

impl DomainEvent for ProjectCreated {
    fn event_type(&self) -> &'static str { "project.created" }
    fn aggregate_id(&self) -> Uuid { self.project_id }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectPublished {
    pub project_id: Uuid,
    pub owner_id: Uuid,
    pub published_by: Uuid,
    pub occurred_at: OffsetDateTime,
}

impl DomainEvent for ProjectPublished {
    fn event_type(&self) -> &'static str { "project.published" }
    fn aggregate_id(&self) -> Uuid { self.project_id }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRejected {
    pub project_id: Uuid,
    pub owner_id: Uuid,
    pub rejected_by: Uuid,
    pub reason: String,
    pub occurred_at: OffsetDateTime,
}

impl DomainEvent for ProjectRejected {
    fn event_type(&self) -> &'static str { "project.rejected" }
    fn aggregate_id(&self) -> Uuid { self.project_id }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectVersionCreated {
    pub project_id: Uuid,
    pub version_id: Uuid,
    pub version_label: String,
    pub created_by: Uuid,
    pub occurred_at: OffsetDateTime,
}

impl DomainEvent for ProjectVersionCreated {
    fn event_type(&self) -> &'static str { "project.version_created" }
    fn aggregate_id(&self) -> Uuid { self.project_id }
}
