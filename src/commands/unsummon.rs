use anyhow::{bail, Result};
use colored::Colorize;
use std::io::{self, Write};

use crate::config::AppConfig;
use crate::i18n;
use crate::zellij::ZellijSession;
use crate::SESSION_NAME;

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

    // Clean up session metadata and relay data
    super::cleanup_session_data();

    println!(
        "{} {}",
        "Success:".green().bold(),
        i18n::tf("unsummon.success", lang, &[("name", SESSION_NAME)])
    );

    Ok(())
}
