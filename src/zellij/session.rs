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

    /// Start a new Zellij session with the specified layout and attach immediately
    /// This blocks until the Zellij session ends (user exits or detaches)
    pub fn start(&self, layout_path: &str) -> Result<()> {
        let status = Command::new("zellij")
            .args([
                "--session",
                &self.name,
                "--new-session-with-layout",
                layout_path,
            ])
            .status()
            .context("Failed to start Zellij session")?;

        if !status.success() {
            anyhow::bail!("Zellij exited with status: {}", status);
        }
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

    /// Attach to an existing session (blocks until detach)
    pub fn attach(&self) -> Result<()> {
        let status = Command::new("zellij")
            .args(["attach", &self.name])
            .status()
            .context("Failed to attach to Zellij session")?;

        if !status.success() {
            anyhow::bail!("Zellij attach exited with status: {}", status);
        }
        Ok(())
    }

    /// Check if session exists (exact match on session name)
    pub fn exists(&self) -> Result<bool> {
        let output = Command::new("zellij")
            .args(["list-sessions"])
            .output()
            .context("Failed to list Zellij sessions")?;

        let sessions = String::from_utf8_lossy(&output.stdout);
        Ok(sessions.lines().any(|line| {
            line.split_whitespace()
                .next()
                .is_some_and(|name| name == self.name)
        }))
    }
}
