use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct OffsetPagination {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl OffsetPagination {
    pub fn normalized(self) -> (i64, i64) {
        let limit = self.limit.unwrap_or(20).clamp(1, 100);
        let offset = self.offset.unwrap_or(0).max(0);
        (limit, offset)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Page<T> {
    pub data: Vec<T>,
    pub limit: i64,
    pub offset: i64,
}
