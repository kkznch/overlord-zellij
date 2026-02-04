use anyhow::{bail, Result};
use colored::Colorize;
use std::io::{self, Write};

use crate::config::delete_session_metadata;
use crate::zellij::ZellijSession;

const SESSION_NAME: &str = "overlord";

pub fn execute(force: bool) -> Result<()> {
    let session = ZellijSession::new(SESSION_NAME);

    // Check if session exists
    if !session.exists()? {
        bail!("セッション '{}' が見つかりません。撃滅対象なし。", SESSION_NAME);
    }

    // Confirm unless force flag is set
    if !force {
        print!(
            "{} セッション '{}' を撃滅しますか？ [y/N] ",
            "Warning:".yellow().bold(),
            SESSION_NAME
        );
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!(
                "{} 中止しました。魔王軍は健在です。",
                "Info:".cyan().bold()
            );
            return Ok(());
        }
    }

    println!(
        "{} セッション '{}' を撃滅中...",
        "Overlord:".red().bold(),
        SESSION_NAME
    );

    // Kill the session
    session.kill()?;

    // Delete session data for complete cleanup
    let _ = session.delete(true);

    // Delete session metadata
    let _ = delete_session_metadata();

    println!(
        "{} 魔王軍は撃滅されました。セッション '{}' は消滅しました。",
        "Success:".green().bold(),
        SESSION_NAME
    );

    Ok(())
}
