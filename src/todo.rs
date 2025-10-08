use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub id: u32,
    pub text: String,
    pub done: bool,
    pub created_at: u64, // Unix timestamp
}

impl Todo {
    pub fn new(text:String) -> Self {
        let created_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
        Todo {
            id:0,
            text,
            done:false,
            created_at,
        }
    }
}