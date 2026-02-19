use anyhow::Result;
use colored::Colorize;
use std::env;

use crate::army::Role;
use crate::config::{find_session_by_cwd, load_registry, AppConfig};
use crate::i18n;
use crate::zellij::ZellijSession;

pub fn execute(all: bool, config: &AppConfig) -> Result<()> {
    let lang = config.lang;

    println!("{}", i18n::t("status.header", lang).red().bold());
    println!();

    if all {
        // Show all registered sessions
        let registry = load_registry()?;
        if registry.sessions.is_empty() {
            println!(
                "{} {}",
                "Info:".cyan().bold(),
                i18n::t("status.no_sessions", lang)
            );
            return Ok(());
        }
        for (name, entry) in &registry.sessions {
            let session = ZellijSession::new(name);
            let alive = session.exists().unwrap_or(false);
            let state = if alive {
                i18n::t("status.active", lang).green().bold()
            } else {
                i18n::t("status.dead", lang).red().bold()
            };
            println!(
                "  {} {} (cwd: {}, started: {})",
                name.green(),
                state,
                entry.cwd.display(),
                entry.started_at.format("%Y-%m-%d %H:%M:%S UTC")
            );
        }
        return Ok(());
    }

    // Resolve session from cwd
    let cwd = env::current_dir()?;
    let (session_name, entry) = match find_session_by_cwd(&cwd)? {
        Some(pair) => pair,
        None => {
            println!(
                "{} {}",
                i18n::t("status.state", lang).cyan().bold(),
                i18n::t("status.not_summoned", lang).red().bold()
            );
            println!();
            println!(
                "{} {}",
                "Hint:".yellow(),
                i18n::t("status.hint_summon", lang)
            );
            return Ok(());
        }
    };

    let session = ZellijSession::new(&session_name);

    if session.exists()? {
        println!(
            "{} {}",
            i18n::t("status.session", lang).cyan().bold(),
            session_name.green()
        );
        println!(
            "{} {}",
            i18n::t("status.state", lang).cyan().bold(),
            i18n::t("status.active", lang).green().bold()
        );
        println!(
            "{} {:?}",
            i18n::t("status.cwd", lang).cyan().bold(),
            entry.cwd
        );
        println!(
            "{} {}",
            i18n::t("status.started_at", lang).cyan().bold(),
            entry.started_at.format("%Y-%m-%d %H:%M:%S UTC")
        );
    } else {
        println!(
            "{} {}",
            i18n::t("status.session", lang).cyan().bold(),
            session_name.yellow()
        );
        println!(
            "{} {}",
            i18n::t("status.state", lang).cyan().bold(),
            i18n::t("status.dead", lang).red().bold()
        );
        println!();
        println!(
            "{} {}",
            "Hint:".yellow(),
            i18n::t("status.hint_summon", lang)
        );
        return Ok(());
    }

    println!();
    println!("{}", i18n::t("status.ranks_header", lang).red().bold());
    println!();

    for role in Role::all() {
        println!("  {} {}", role.icon(), role.display_name());
    }

    println!();
    println!(
        "{} {}",
        "Hint:".yellow(),
        i18n::t("status.hint_unsummon", lang)
    );

    Ok(())
}
