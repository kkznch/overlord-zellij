use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::army::roles::Role;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub from: Role,
    pub to: Role,
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
    pub role: Role,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Insight {
    pub id: String,
    pub from: Role,
    pub category: String,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_default_is_normal() {
        assert!(matches!(Priority::default(), Priority::Normal));
    }

    #[test]
    fn test_status_display() {
        assert_eq!(format!("{}", Status::Idle), "idle");
        assert_eq!(format!("{}", Status::Working), "working");
        assert_eq!(format!("{}", Status::Blocked), "blocked");
        assert_eq!(format!("{}", Status::Done), "done");
    }

    #[test]
    fn test_priority_display() {
        assert_eq!(format!("{}", Priority::Normal), "normal");
        assert_eq!(format!("{}", Priority::Urgent), "urgent");
    }

    #[test]
    fn test_message_json_roundtrip() {
        let msg = Message {
            id: "123_overlord".to_string(),
            from: Role::Overlord,
            to: Role::Inferno,
            subject: "Test".to_string(),
            body: "Hello".to_string(),
            priority: Priority::Urgent,
            timestamp: Utc::now(),
            read: false,
        };
        let json = serde_json::to_string(&msg).unwrap();
        let deserialized: Message = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, msg.id);
        assert_eq!(deserialized.from, Role::Overlord);
        assert_eq!(deserialized.to, Role::Inferno);
        assert!(!deserialized.read);
    }

    #[test]
    fn test_role_status_json_roundtrip() {
        let status = RoleStatus {
            role: Role::Glacier,
            status: Status::Working,
            task: Some("Defining types".to_string()),
            updated_at: Utc::now(),
        };
        let json = serde_json::to_string(&status).unwrap();
        let deserialized: RoleStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.role, Role::Glacier);
        assert!(matches!(deserialized.status, Status::Working));
        assert_eq!(deserialized.task.as_deref(), Some("Defining types"));
    }

    #[test]
    fn test_priority_all_variants_serialize() {
        for priority in [Priority::Normal, Priority::Urgent] {
            let json = serde_json::to_string(&priority).unwrap();
            let deserialized: Priority = serde_json::from_str(&json).unwrap();
            assert_eq!(format!("{}", priority), format!("{}", deserialized));
        }
    }

    #[test]
    fn test_insight_json_roundtrip() {
        let insight = Insight {
            id: "123_glacier".to_string(),
            from: Role::Glacier,
            category: "architecture".to_string(),
            title: "Relay store pattern".to_string(),
            content: "File-based JSON persistence works well for IPC".to_string(),
            tags: vec!["relay".to_string(), "persistence".to_string()],
            timestamp: Utc::now(),
        };
        let json = serde_json::to_string(&insight).unwrap();
        let deserialized: Insight = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, insight.id);
        assert_eq!(deserialized.from, Role::Glacier);
        assert_eq!(deserialized.category, "architecture");
        assert_eq!(deserialized.tags.len(), 2);
    }
}
