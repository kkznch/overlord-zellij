use anyhow::{bail, Result};
use colored::Colorize;
use std::io::{self, Write};

use crate::config::{delete_session_metadata, relay_dir, AppConfig};
use crate::i18n;
use crate::relay::store::MessageStore;
use crate::zellij::ZellijSession;

const SESSION_NAME: &str = "overlord";

pub fn execute(force: bool, config: &AppConfig) -> Result<()> {
    let lang = config.lang;
    let session = ZellijSession::new(SESSION_NAME);

    // Check if session exists
    if !session.exists()? {
        bail!(
            "{}",
            i18n::tf("unsummon.not_found", lang, &[("name", SESSION_NAME)])
        );
    }

    // Confirm unless force flag is set
    if !force {
        print!(
            "{} {}",
            "Warning:".yellow().bold(),
            i18n::tf("unsummon.confirm", lang, &[("name", SESSION_NAME)])
        );
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!(
                "{} {}",
                "Info:".cyan().bold(),
                i18n::t("unsummon.cancelled", lang)
            );
            return Ok(());
        }
    }

    println!(
        "{} {}",
        "Overlord:".red().bold(),
        i18n::tf("unsummon.in_progress", lang, &[("name", SESSION_NAME)])
    );

    // Kill the session
    session.kill()?;

    // Delete session data for complete cleanup
    let _ = session.delete(true);

    // Delete session metadata
    let _ = delete_session_metadata();

    // Clean up relay directory
    if let Ok(relay) = relay_dir() {
        let store = MessageStore::new(relay);
        let _ = store.cleanup();
    }

    println!(
        "{} {}",
        "Success:".green().bold(),
        i18n::tf("unsummon.success", lang, &[("name", SESSION_NAME)])
    );

    Ok(())
}
