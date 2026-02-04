use anyhow::Result;
use colored::Colorize;

use crate::config::{ensure_default_config, resolve_rituals_dir, validate_rituals_dir};
use crate::layout::create_temp_layout;
use crate::zellij::ZellijSession;

pub fn execute(_layout: &str, session_name: &str, no_rituals: bool) -> Result<()> {
    let session = ZellijSession::new(session_name);

    // Check if session already exists
    if session.exists()? {
        println!(
            "{} Session '{}' already exists. Attaching...",
            "Info:".cyan().bold(),
            session_name
        );
        return session.attach();
    }

    // Ensure default config exists (creates ~/.config/ovld/rituals/ if needed)
    ensure_default_config()?;

    // Resolve rituals directory (local first, then global)
    let rituals_dir = resolve_rituals_dir()?;

    // Validate ritual files exist
    if !no_rituals {
        validate_rituals_dir(&rituals_dir)?;
    }

    println!(
        "{} Summoning the army...",
        "Overlord:".red().bold(),
    );

    if !no_rituals {
        println!(
            "{} Using rituals from {:?}",
            "Info:".cyan().bold(),
            rituals_dir
        );
    }

    // Generate layout with absolute paths to ritual files
    // The temp file is kept alive until this scope ends
    let (_temp_file, layout_path) = create_temp_layout(&rituals_dir, no_rituals)?;

    // Start the session - this blocks until Zellij exits
    session.start(layout_path.to_str().unwrap())?;

    // This message shows after user exits Zellij
    println!(
        "{} Session '{}' ended.",
        "Info:".cyan().bold(),
        session_name
    );

    Ok(())
}
