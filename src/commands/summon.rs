use anyhow::{bail, Result};
use colored::Colorize;
use std::env;
use std::path::PathBuf;

use crate::army::RitualInjector;
use crate::zellij::ZellijSession;

pub fn execute(layout: &str, session_name: &str, no_rituals: bool) -> Result<()> {
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

    // Find layout file
    let layout_path = find_layout_file(layout)?;

    println!(
        "{} Summoning the army with layout '{}'...",
        "Overlord:".red().bold(),
        layout
    );

    // Start the session
    session.start(layout_path.to_str().unwrap())?;

    println!(
        "{} Session '{}' created.",
        "Success:".green().bold(),
        session_name
    );

    // Inject rituals if not disabled
    if !no_rituals {
        let rituals_dir = find_rituals_dir()?;
        println!(
            "{} Performing rituals...",
            "Overlord:".red().bold()
        );

        // Wait a bit for session to be ready
        std::thread::sleep(std::time::Duration::from_secs(2));

        let injector = RitualInjector::new(rituals_dir, session_name);
        injector.inject_all()?;

        println!(
            "{} All rituals complete. The army awaits your command.",
            "Success:".green().bold()
        );
    }

    Ok(())
}

fn find_layout_file(layout_name: &str) -> Result<PathBuf> {
    // Try current directory layouts/
    let current_dir = env::current_dir()?;
    let layout_file = format!("{}.kdl", layout_name);

    // Check layouts/ directory
    let layouts_dir = current_dir.join("layouts");
    let layout_path = layouts_dir.join(&layout_file);
    if layout_path.exists() {
        return Ok(layout_path);
    }

    // Check if it's already a full path
    let direct_path = PathBuf::from(layout_name);
    if direct_path.exists() {
        return Ok(direct_path);
    }

    bail!(
        "Layout file '{}' not found. Looked in: {:?}",
        layout_name,
        layouts_dir
    );
}

fn find_rituals_dir() -> Result<PathBuf> {
    let current_dir = env::current_dir()?;
    let rituals_dir = current_dir.join("rituals");

    if rituals_dir.exists() {
        Ok(rituals_dir)
    } else {
        bail!("Rituals directory not found at {:?}", rituals_dir);
    }
}
