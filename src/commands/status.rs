use anyhow::Result;
use colored::Colorize;

use crate::army::Role;
use crate::config::{load_session_metadata, AppConfig};
use crate::i18n;
use crate::zellij::ZellijSession;
use crate::SESSION_NAME;

pub fn execute(config: &AppConfig) -> Result<()> {
    let lang = config.lang;
    let session = ZellijSession::new(SESSION_NAME);

    println!("{}", i18n::t("status.header", lang).red().bold());
    println!();

    // Check if session exists
    if session.exists()? {
        println!(
            "{} {}",
            i18n::t("status.session", lang).cyan().bold(),
            SESSION_NAME.green()
        );
        println!(
            "{} {}",
            i18n::t("status.state", lang).cyan().bold(),
            i18n::t("status.active", lang).green().bold()
        );

        // Show session metadata if available
        if let Some(meta) = load_session_metadata()? {
            println!(
                "{} {:?}",
                i18n::t("status.cwd", lang).cyan().bold(),
                meta.cwd
            );
            println!(
                "{} {}",
                i18n::t("status.started_at", lang).cyan().bold(),
                meta.started_at.format("%Y-%m-%d %H:%M:%S UTC")
            );
        }
    } else {
        println!(
            "{} {}",
            i18n::t("status.session", lang).cyan().bold(),
            SESSION_NAME.yellow()
        );
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
