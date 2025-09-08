use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    pub id: u32,
    pub name: String,
    pub username: String,
    pub password: String,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VaultV1 {
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub last_modified: DateTime<Utc>,
    pub entries: Vec<Entry>,
}
