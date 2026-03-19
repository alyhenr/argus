use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EventType {
    PageView,
    Click,
    Error,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub project_id: String,
    pub event_type: EventType,
    pub url: String,
    pub timestamp: u64,
    pub payload: Option<HashMap<String, String>>,
}