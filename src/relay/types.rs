use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub const ALL_ROLES: &[&str] = &[
    "overlord",
    "strategist",
    "inferno",
    "glacier",
    "shadow",
    "storm",
];

pub fn is_valid_role(role: &str) -> bool {
    ALL_ROLES.contains(&role)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub from: String,
    pub to: String,
    pub subject: String,
    pub body: String,
    pub priority: Priority,
    pub timestamp: DateTime<Utc>,
    pub read: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    #[default]
    Normal,
    Urgent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleStatus {
    pub role: String,
    pub status: Status,
    pub task: Option<String>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    #[default]
    Idle,
    Working,
    Blocked,
    Done,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Idle => write!(f, "idle"),
            Status::Working => write!(f, "working"),
            Status::Blocked => write!(f, "blocked"),
            Status::Done => write!(f, "done"),
        }
    }
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Normal => write!(f, "normal"),
            Priority::Urgent => write!(f, "urgent"),
        }
    }
}
