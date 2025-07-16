use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Job {
    pub queue: String,
    pub args: JsonValue,
    pub retry: bool,
    pub class: String,
    pub jid: i64,
    pub created_at: f64,
    pub enqueued_at: Option<f64>,
    pub failed_at: Option<f64>,
    pub error_message: Option<String>,
    pub retry_count: Option<usize>,
    pub retried_at: Option<f64>,

    #[serde(skip)]
    pub unique_for: Option<std::time::Duration>,
}
