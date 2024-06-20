use serde::{Deserialize, Serialize};
use chrono::prelude::*;
#[derive(Serialize, Deserialize, Debug)]
pub struct update_response {
    pub message : String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}
