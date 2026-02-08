use anyhow::Result;
use colored::Colorize;

use crate::army::Role;
use crate::config::load_session_metadata;
use crate::zellij::ZellijSession;

const SESSION_NAME: &str = "overlord";

pub fn execute() -> Result<()> {
    let session = ZellijSession::new(SESSION_NAME);

    println!("{}", "=== 魔王軍ステータス ===".red().bold());
    println!();

    // Check if session exists
    if session.exists()? {
        println!(
            "{} {}",
            "セッション:".cyan().bold(),
            SESSION_NAME.green()
        );
        println!(
            "{} {}",
            "状態:".cyan().bold(),
            "展開中".green().bold()
        );

        // Show session metadata if available
        if let Some(meta) = load_session_metadata()? {
            println!(
                "{} {:?}",
                "作業場所:".cyan().bold(),
                meta.cwd
            );
            println!(
                "{} {}",
                "召喚時刻:".cyan().bold(),
                meta.started_at.format("%Y-%m-%d %H:%M:%S UTC")
            );
        }
    } else {
        println!(
            "{} {}",
            "セッション:".cyan().bold(),
            SESSION_NAME.yellow()
        );
        println!(
            "{} {}",
            "状態:".cyan().bold(),
            "未召喚".red().bold()
        );
        println!();
        println!(
            "{} '{}' で魔王軍を召喚してください。",
            "ヒント:".yellow(),
            "ovld summon".cyan()
        );
        return Ok(());
    }

    println!();
    println!("{}", "=== 魔王軍階級 ===".red().bold());
    println!();

    for role in Role::all() {
        let icon = match role {
            Role::Overlord => "👑",
            Role::Strategist => "🗡️",
            Role::Inferno => "🔥",
            Role::Glacier => "🧊",
            Role::Shadow => "🌑",
            Role::Storm => "⚡",
        };
        println!("  {} {}", icon, role.display_name());
    }

    println!();
    println!(
        "{} '{}' でセッションを還送できます。",
        "ヒント:".yellow(),
        "ovld unsummon".cyan()
    );

    Ok(())
}
