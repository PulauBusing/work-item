use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub color: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Item {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub start_date: String,
    pub end_date: Option<String>,
    pub detail: String,
    pub jira: Option<String>,
    pub tags: Vec<Tag>,
    /// "pending" | "done"，由 end_date 是否为空推导
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct ItemInput {
    pub name: String,
    pub start_date: String,
    pub end_date: Option<String>,
    pub detail: String,
    pub jira: Option<String>,
    pub tags: Vec<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ItemFilter {
    /// None=全部, "pending"=未完成, "done"=已完成
    pub status: Option<String>,
    /// "today" | "week" | "last_week" | "month" | "last_month" | "custom" | None
    pub range: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub tag_ids: Option<Vec<i64>>,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Debug, Serialize)]
pub struct PageResult {
    pub items: Vec<Item>,
    pub total: i64,
}
