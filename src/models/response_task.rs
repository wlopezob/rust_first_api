use chrono::{FixedOffset, DateTime};
use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseTask {
    pub id: i32,
    pub title: String,
    pub priority: Option<String>,
    pub description: Option<String>,
    pub deleted_at: Option<DateTime<FixedOffset>>,
    pub user_id: Option<i32>
}
