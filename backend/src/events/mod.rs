pub mod commerce_events;
pub mod media_events;
pub mod project_events;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Base trait for all domain events
pub trait DomainEvent: Serialize + Send + Sync + 'static {
    fn event_type(&self) -> &'static str;
    fn aggregate_id(&self) -> Uuid;
}
