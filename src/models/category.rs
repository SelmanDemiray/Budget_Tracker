use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub color: String,
    pub icon: Option<String>,
}

impl Category {
    pub fn new(name: String, color: String, icon: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            color,
            icon,
        }
    }
}
