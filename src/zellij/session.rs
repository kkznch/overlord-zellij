use anyhow::{Context, Result};
use std::process::Command;

pub struct ZellijSession {
    pub name: String,
}

impl ZellijSession {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    /// Start a new Zellij session with the specified layout
    pub fn start(&self, layout_path: &str) -> Result<()> {
        Command::new("zellij")
            .args(["--session", &self.name, "--layout", layout_path])
            .spawn()
            .context("Failed to start Zellij session")?;
        Ok(())
    }

    /// Kill the session
    pub fn kill(&self) -> Result<()> {
        Command::new("zellij")
            .args(["kill-session", &self.name])
            .status()
            .context("Failed to kill Zellij session")?;
        Ok(())
    }

    /// Delete the session (for cleanup)
    pub fn delete(&self, force: bool) -> Result<()> {
        let mut args = vec!["delete-session", &self.name];
        if force {
            args.push("--force");
        }
        Command::new("zellij")
            .args(&args)
            .status()
            .context("Failed to delete Zellij session")?;
        Ok(())
    }

    /// Check if session exists
    pub fn exists(&self) -> Result<bool> {
        let output = Command::new("zellij")
            .args(["list-sessions"])
            .output()
            .context("Failed to list Zellij sessions")?;

        let sessions = String::from_utf8_lossy(&output.stdout);
        Ok(sessions.lines().any(|line| line.contains(&self.name)))
    }

    /// Attach to existing session
    pub fn attach(&self) -> Result<()> {
        Command::new("zellij")
            .args(["attach", &self.name])
            .status()
            .context("Failed to attach to Zellij session")?;
        Ok(())
    }

    /// List all sessions
    pub fn list_sessions() -> Result<Vec<String>> {
        let output = Command::new("zellij")
            .args(["list-sessions"])
            .output()
            .context("Failed to list Zellij sessions")?;

        let sessions = String::from_utf8_lossy(&output.stdout);
        Ok(sessions
            .lines()
            .filter(|line| !line.is_empty())
            .map(|s| s.to_string())
            .collect())
    }
}
