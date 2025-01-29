use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
}

#[derive(Debug)]
pub struct TaskStats {
    pub total: usize,
    pub last_id: u32,
}