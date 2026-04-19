use serde::{Deserialize, Serialize};
 
pub const DEFAULT_PAGE_SIZE: i64 = 20;
pub const MAX_PAGE_SIZE: i64 = 100;
 
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub sort: Option<String>,
    pub order: Option<String>,
}
 
impl PaginationParams {
    pub fn page(&self) -> i64 {
        self.page.unwrap_or(1).max(1)
    }
 
    pub fn per_page(&self) -> i64 {
        self.per_page
            .unwrap_or(DEFAULT_PAGE_SIZE)
            .min(MAX_PAGE_SIZE)
            .max(1)
    }
 
    pub fn offset(&self) -> i64 {
        (self.page() - 1) * self.per_page()
    }
 
    pub fn limit(&self) -> i64 {
        self.per_page()
    }
 
    pub fn order_dir(&self) -> &str {
        match self.order.as_deref() {
            Some("asc") | Some("ASC") => "ASC",
            _ => "DESC",
        }
    }
}
 
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub data: Vec<T>,
    pub meta: PaginationMeta,
}
 
#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub total_pages: i64,
    pub has_next: bool,
    pub has_prev: bool,
}
 
impl<T: Serialize> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total: i64, params: &PaginationParams) -> Self {
        let page = params.page();
        let per_page = params.per_page();
        let total_pages = (total + per_page - 1) / per_page;
 
        Self {
            data,
            meta: PaginationMeta {
                page,
                per_page,
                total,
                total_pages,
                has_next: page < total_pages,
                has_prev: page > 1,
            },
        }
    }
}
 


