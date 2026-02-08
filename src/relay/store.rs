use anyhow::{Context, Result};
use chrono::Utc;
use std::fs;
use std::path::PathBuf;

use super::types::{Message, Priority, RoleStatus, Status, ALL_ROLES};

pub struct MessageStore {
    base_dir: PathBuf,
}

impl MessageStore {
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    /// Initialize directory structure for all roles
    pub fn init(&self) -> Result<()> {
        for role in ALL_ROLES {
            fs::create_dir_all(self.inbox_dir(role))
                .with_context(|| format!("Failed to create inbox for {}", role))?;
        }
        fs::create_dir_all(self.status_dir())
            .context("Failed to create status directory")?;
        fs::create_dir_all(self.pending_dir())
            .context("Failed to create pending directory")?;

        // Initialize status for all roles
        for role in ALL_ROLES {
            let status_file = self.status_dir().join(format!("{}.json", role));
            if !status_file.exists() {
                self.update_status(role, Status::Idle, None)?;
            }
        }
        Ok(())
    }

    fn inbox_dir(&self, role: &str) -> PathBuf {
        self.base_dir.join("inbox").join(role)
    }

    fn status_dir(&self) -> PathBuf {
        self.base_dir.join("status")
    }

    fn pending_dir(&self) -> PathBuf {
        self.base_dir.join("pending")
    }

    /// Send a message to a role's inbox
    pub fn send_message(
        &self,
        from: &str,
        to: &str,
        subject: &str,
        body: &str,
        priority: Priority,
    ) -> Result<Message> {
        let timestamp = Utc::now();
        let id = format!("{}_{}", timestamp.timestamp_millis(), from);
        let msg = Message {
            id: id.clone(),
            from: from.to_string(),
            to: to.to_string(),
            subject: subject.to_string(),
            body: body.to_string(),
            priority,
            timestamp,
            read: false,
        };

        let inbox = self.inbox_dir(to);
        fs::create_dir_all(&inbox)?;
        let file_path = inbox.join(format!("{}.json", id));
        let content = serde_json::to_string_pretty(&msg)?;
        fs::write(&file_path, content)?;

        Ok(msg)
    }

    /// Read all unread messages from a role's inbox, optionally marking as read
    pub fn check_inbox(&self, role: &str, mark_read: bool) -> Result<Vec<Message>> {
        let inbox = self.inbox_dir(role);
        if !inbox.exists() {
            return Ok(vec![]);
        }

        let mut entries: Vec<_> = fs::read_dir(&inbox)?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "json"))
            .collect();

        entries.sort_by_key(|e| e.path());

        let mut messages = Vec::new();
        for entry in entries {
            let content = fs::read_to_string(entry.path())?;
            let mut msg: Message = serde_json::from_str(&content)?;
            if !msg.read {
                if mark_read {
                    msg.read = true;
                    fs::write(entry.path(), serde_json::to_string_pretty(&msg)?)?;
                }
                messages.push(msg);
            }
        }

        // Clear pending flag
        let _ = fs::remove_file(self.pending_dir().join(role));

        Ok(messages)
    }

    /// Set pending notification flag for a role. Returns true if flag was newly set.
    pub fn set_pending(&self, role: &str) -> Result<bool> {
        let pending_flag = self.pending_dir().join(role);
        if pending_flag.exists() {
            return Ok(false);
        }
        fs::create_dir_all(self.pending_dir())?;
        fs::write(&pending_flag, "")?;
        Ok(true)
    }

    /// Update a role's status
    pub fn update_status(&self, role: &str, status: Status, task: Option<&str>) -> Result<()> {
        let status_data = RoleStatus {
            role: role.to_string(),
            status,
            task: task.map(|s| s.to_string()),
            updated_at: Utc::now(),
        };
        let file_path = self.status_dir().join(format!("{}.json", role));
        fs::create_dir_all(self.status_dir())?;
        fs::write(&file_path, serde_json::to_string_pretty(&status_data)?)?;
        Ok(())
    }

    /// Get a role's current status
    pub fn get_status(&self, role: &str) -> Result<Option<RoleStatus>> {
        let file_path = self.status_dir().join(format!("{}.json", role));
        if !file_path.exists() {
            return Ok(None);
        }
        let content = fs::read_to_string(&file_path)?;
        let status: RoleStatus = serde_json::from_str(&content)?;
        Ok(Some(status))
    }

    /// Get all roles' statuses
    pub fn get_all_statuses(&self) -> Result<Vec<RoleStatus>> {
        let mut statuses = Vec::new();
        for role in ALL_ROLES {
            if let Some(status) = self.get_status(role)? {
                statuses.push(status);
            }
        }
        Ok(statuses)
    }

    /// Clean up all relay data
    pub fn cleanup(&self) -> Result<()> {
        if self.base_dir.exists() {
            fs::remove_dir_all(&self.base_dir).context("Failed to clean up relay directory")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn test_store() -> (TempDir, MessageStore) {
        let dir = TempDir::new().unwrap();
        let store = MessageStore::new(dir.path().to_path_buf());
        store.init().unwrap();
        (dir, store)
    }

    #[test]
    fn test_send_and_receive_message() {
        let (_dir, store) = test_store();

        store
            .send_message(
                "strategist",
                "inferno",
                "Implement auth",
                "See types.rs",
                Priority::Normal,
            )
            .unwrap();

        let messages = store.check_inbox("inferno", true).unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].from, "strategist");
        assert_eq!(messages[0].subject, "Implement auth");

        // After marking read, should be empty
        let messages = store.check_inbox("inferno", true).unwrap();
        assert_eq!(messages.len(), 0);
    }

    #[test]
    fn test_multiple_messages() {
        let (_dir, store) = test_store();

        store
            .send_message("glacier", "inferno", "Types ready", "types.rs", Priority::Normal)
            .unwrap();
        store
            .send_message(
                "strategist",
                "inferno",
                "Start impl",
                "Go ahead",
                Priority::Urgent,
            )
            .unwrap();

        let messages = store.check_inbox("inferno", false).unwrap();
        assert_eq!(messages.len(), 2);
    }

    #[test]
    fn test_pending_flag() {
        let (_dir, store) = test_store();

        assert!(store.set_pending("inferno").unwrap()); // First time: true
        assert!(!store.set_pending("inferno").unwrap()); // Second time: false (already pending)

        // check_inbox clears the flag
        store.check_inbox("inferno", true).unwrap();
        assert!(store.set_pending("inferno").unwrap()); // After clear: true again
    }

    #[test]
    fn test_status() {
        let (_dir, store) = test_store();

        store
            .update_status("inferno", Status::Working, Some("Implementing auth"))
            .unwrap();

        let status = store.get_status("inferno").unwrap().unwrap();
        assert_eq!(status.role, "inferno");
        assert!(matches!(status.status, Status::Working));
        assert_eq!(status.task.as_deref(), Some("Implementing auth"));
    }

    #[test]
    fn test_get_all_statuses() {
        let (_dir, store) = test_store();

        let statuses = store.get_all_statuses().unwrap();
        assert_eq!(statuses.len(), 6); // All roles initialized as idle
    }

    #[test]
    fn test_empty_inbox() {
        let (_dir, store) = test_store();
        let messages = store.check_inbox("inferno", true).unwrap();
        assert!(messages.is_empty());
    }

    #[test]
    fn test_cleanup() {
        let (_dir, store) = test_store();
        store.cleanup().unwrap();
        assert!(!store.base_dir.exists());
    }
}
