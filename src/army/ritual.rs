use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

use crate::army::roles::Role;
use crate::zellij::pane::PaneWriter;

pub struct RitualInjector {
    rituals_dir: PathBuf,
    pane_writer: PaneWriter,
}

impl RitualInjector {
    pub fn new(rituals_dir: PathBuf, session_name: &str) -> Self {
        Self {
            rituals_dir,
            pane_writer: PaneWriter::new(session_name),
        }
    }

    /// Load ritual content for a role
    fn load_ritual(&self, role: &Role) -> Result<String> {
        let ritual_path = self.rituals_dir.join(role.ritual_file());
        fs::read_to_string(&ritual_path)
            .with_context(|| format!("Failed to read ritual file: {:?}", ritual_path))
    }

    /// Inject ritual into a specific role's pane
    pub fn inject_ritual(&self, role: &Role) -> Result<()> {
        let ritual_content = self.load_ritual(role)?;

        // Navigate to the correct tab
        let tab_name = match role {
            Role::Overlord => "overlord",
            Role::Strategist => "strategist",
            Role::LegionImpl | Role::LegionDebug | Role::LegionDocs => "legions",
        };
        self.pane_writer.focus_tab(tab_name)?;

        // For legions tab, need to focus the correct pane
        if matches!(
            role,
            Role::LegionImpl | Role::LegionDebug | Role::LegionDocs
        ) {
            // Focus the appropriate pane within legions tab
            match role {
                Role::LegionImpl => {} // First pane, already focused
                Role::LegionDebug => {
                    self.pane_writer.focus_next_pane()?;
                }
                Role::LegionDocs => {
                    self.pane_writer.focus_next_pane()?;
                    self.pane_writer.focus_next_pane()?;
                }
                _ => {}
            }
        }

        // Inject the ritual prompt as a claude command
        let escaped_content = ritual_content.replace('"', "\\\"").replace('\n', " ");
        let command = format!("claude --print-system-prompt \"{}\"", escaped_content);

        self.pane_writer.inject_prompt(&command)?;

        Ok(())
    }

    /// Inject rituals to all roles
    pub fn inject_all(&self) -> Result<()> {
        for role in Role::all() {
            println!("Injecting ritual to {}...", role.display_name());
            self.inject_ritual(&role)?;
            // Small delay between injections
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        Ok(())
    }
}
