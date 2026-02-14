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

const SHITENNOH: &[&str] = &["inferno", "glacier", "shadow", "storm"];

fn is_shitennoh(role: &str) -> bool {
    SHITENNOH.contains(&role)
}

pub fn is_valid_role(role: &str) -> bool {
    ALL_ROLES.contains(&role)
}

/// Check if a message route is allowed under the chain-of-command policy.
/// Only overlord <-> strategist <-> shitennoh routes are permitted.
pub fn is_allowed_route(from: &str, to: &str) -> bool {
    match (from, to) {
        ("overlord", "strategist") | ("strategist", "overlord") => true,
        ("strategist", to) if is_shitennoh(to) => true,
        (from, "strategist") if is_shitennoh(from) => true,
        _ => false,
    }
}

/// Return the list of roles this role is allowed to send messages to.
pub fn allowed_targets(role: &str) -> Vec<&'static str> {
    match role {
        "overlord" => vec!["strategist"],
        "strategist" => vec!["overlord", "inferno", "glacier", "shadow", "storm"],
        r if is_shitennoh(r) => vec!["strategist"],
        _ => vec![],
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_role_valid() {
        for role in ALL_ROLES {
            assert!(is_valid_role(role), "{} should be valid", role);
        }
    }

    #[test]
    fn test_is_valid_role_invalid() {
        assert!(!is_valid_role("king"));
        assert!(!is_valid_role(""));
        assert!(!is_valid_role("Overlord")); // case-sensitive
    }

    #[test]
    fn test_all_roles_count() {
        assert_eq!(ALL_ROLES.len(), 6);
    }

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
            from: "overlord".to_string(),
            to: "inferno".to_string(),
            subject: "Test".to_string(),
            body: "Hello".to_string(),
            priority: Priority::Urgent,
            timestamp: Utc::now(),
            read: false,
        };
        let json = serde_json::to_string(&msg).unwrap();
        let deserialized: Message = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, msg.id);
        assert_eq!(deserialized.from, msg.from);
        assert_eq!(deserialized.to, msg.to);
        assert!(!deserialized.read);
    }

    #[test]
    fn test_role_status_json_roundtrip() {
        let status = RoleStatus {
            role: "glacier".to_string(),
            status: Status::Working,
            task: Some("Defining types".to_string()),
            updated_at: Utc::now(),
        };
        let json = serde_json::to_string(&status).unwrap();
        let deserialized: RoleStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.role, "glacier");
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
    fn test_allowed_route_overlord_strategist() {
        assert!(is_allowed_route("overlord", "strategist"));
        assert!(is_allowed_route("strategist", "overlord"));
    }

    #[test]
    fn test_allowed_route_strategist_shitennoh() {
        for role in ["inferno", "glacier", "shadow", "storm"] {
            assert!(is_allowed_route("strategist", role), "strategist -> {} should be allowed", role);
            assert!(is_allowed_route(role, "strategist"), "{} -> strategist should be allowed", role);
        }
    }

    #[test]
    fn test_forbidden_route_overlord_shitennoh() {
        for role in ["inferno", "glacier", "shadow", "storm"] {
            assert!(!is_allowed_route("overlord", role), "overlord -> {} should be forbidden", role);
            assert!(!is_allowed_route(role, "overlord"), "{} -> overlord should be forbidden", role);
        }
    }

    #[test]
    fn test_forbidden_route_shitennoh_to_shitennoh() {
        let shitennoh = ["inferno", "glacier", "shadow", "storm"];
        for from in &shitennoh {
            for to in &shitennoh {
                if from != to {
                    assert!(!is_allowed_route(from, to), "{} -> {} should be forbidden", from, to);
                }
            }
        }
    }

    #[test]
    fn test_allowed_targets() {
        assert_eq!(allowed_targets("overlord"), vec!["strategist"]);
        assert_eq!(allowed_targets("strategist"), vec!["overlord", "inferno", "glacier", "shadow", "storm"]);
        for role in ["inferno", "glacier", "shadow", "storm"] {
            assert_eq!(allowed_targets(role), vec!["strategist"], "{} should only target strategist", role);
        }
        assert!(allowed_targets("unknown").is_empty());
    }
}
