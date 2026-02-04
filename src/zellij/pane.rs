use anyhow::{Context, Result};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

pub struct PaneWriter {
    session_name: String,
}

impl PaneWriter {
    pub fn new(session_name: &str) -> Self {
        Self {
            session_name: session_name.to_string(),
        }
    }

    /// Write characters to the currently focused pane
    pub fn write_chars(&self, text: &str) -> Result<()> {
        Command::new("zellij")
            .args([
                "--session",
                &self.session_name,
                "action",
                "write-chars",
                text,
            ])
            .status()
            .context("Failed to write chars to pane")?;
        Ok(())
    }

    /// Send Enter key (byte 13)
    pub fn send_enter(&self) -> Result<()> {
        Command::new("zellij")
            .args(["--session", &self.session_name, "action", "write", "13"])
            .status()
            .context("Failed to send Enter key")?;
        Ok(())
    }

    /// Focus on a specific tab by name
    pub fn focus_tab(&self, tab_name: &str) -> Result<()> {
        Command::new("zellij")
            .args([
                "--session",
                &self.session_name,
                "action",
                "go-to-tab-name",
                tab_name,
            ])
            .status()
            .context("Failed to focus tab")?;
        Ok(())
    }

    /// Focus next pane
    pub fn focus_next_pane(&self) -> Result<()> {
        Command::new("zellij")
            .args(["--session", &self.session_name, "action", "focus-next-pane"])
            .status()
            .context("Failed to focus next pane")?;
        Ok(())
    }

    /// Inject prompt to pane with delay for stability
    pub fn inject_prompt(&self, prompt: &str) -> Result<()> {
        // Small delay to ensure pane is ready
        sleep(Duration::from_millis(500));

        // Write the prompt
        self.write_chars(prompt)?;

        // Send Enter to execute
        self.send_enter()?;

        Ok(())
    }
}
