use anyhow::{bail, Result};
use colored::Colorize;
use std::io::{self, Write};

use crate::zellij::ZellijSession;

pub fn execute(session_name: &str, force: bool) -> Result<()> {
    let session = ZellijSession::new(session_name);

    // Check if session exists
    if !session.exists()? {
        bail!("Session '{}' not found. Nothing to slay.", session_name);
    }

    // Confirm unless force flag is set
    if !force {
        print!(
            "{} Are you sure you want to slay the army in session '{}'? [y/N] ",
            "Warning:".yellow().bold(),
            session_name
        );
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("{} Aborted. The army lives another day.", "Info:".cyan().bold());
            return Ok(());
        }
    }

    println!(
        "{} Slaying the army in session '{}'...",
        "Overlord:".red().bold(),
        session_name
    );

    // Kill the session
    session.kill()?;

    // Delete session data for complete cleanup
    let _ = session.delete(true); // Ignore errors on delete

    println!(
        "{} The army has been slain. Session '{}' destroyed.",
        "Success:".green().bold(),
        session_name
    );

    Ok(())
}
