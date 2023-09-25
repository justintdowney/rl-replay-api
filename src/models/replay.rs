use serde::{Deserialize, Serialize};
use chrono::prelude::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Replay {
    pub id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

