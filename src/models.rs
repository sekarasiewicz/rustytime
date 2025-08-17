use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub archived: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub description: Option<String>,
    pub archived: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEntry {
    pub id: String,
    pub task_id: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub duration_seconds: Option<i64>,
    pub local_date: String,
    pub created_at: String,
}
