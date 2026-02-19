use anyhow::{bail, Result};
use colored::Colorize;
use std::env;
use std::io::{self, Write};

use crate::config::{find_session_by_cwd, load_registry, AppConfig};
use crate::i18n;
use crate::zellij::ZellijSession;

/// Unsummon a single session by name.
fn unsummon_one(session_name: &str, force: bool, lang: crate::i18n::Lang) -> Result<()> {
    let session = ZellijSession::new(session_name);

    if !session.exists()? {
        bail!(
            "{}",
            i18n::tf("unsummon.not_found", lang, &[("name", session_name)])
        );
    }

    if !force {
        print!(
            "{} {}",
            "Warning:".yellow().bold(),
            i18n::tf("unsummon.confirm", lang, &[("name", session_name)])
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
        i18n::tf("unsummon.in_progress", lang, &[("name", session_name)])
    );

    session.kill()?;
    let _ = session.delete(true);
    super::cleanup_session_data(session_name);

    println!(
        "{} {}",
        "Success:".green().bold(),
        i18n::tf("unsummon.success", lang, &[("name", session_name)])
    );

    Ok(())
}

pub fn execute(name: Option<String>, all: bool, force: bool, config: &AppConfig) -> Result<()> {
    let lang = config.lang;

    if all {
        // Unsummon all registered sessions
        let registry = load_registry()?;
        if registry.sessions.is_empty() {
            println!(
                "{} {}",
                "Info:".cyan().bold(),
                i18n::t("unsummon.no_sessions", lang)
            );
            return Ok(());
        }
        let names: Vec<String> = registry.sessions.keys().cloned().collect();
        for session_name in names {
            if let Err(e) = unsummon_one(&session_name, force, lang) {
                eprintln!(
                    "{} {}: {}",
                    "Warning:".yellow().bold(),
                    session_name,
                    e
                );
            }
        }
        return Ok(());
    }

    // Resolve session name: explicit arg > cwd lookup
    let session_name = match name {
        Some(n) => n,
        None => {
            let cwd = env::current_dir()?;
            match find_session_by_cwd(&cwd)? {
                Some((n, _)) => n,
                None => bail!(
                    "{}",
                    i18n::t("unsummon.no_session_for_cwd", lang)
                ),
            }
        }
    };

    unsummon_one(&session_name, force, lang)
}
