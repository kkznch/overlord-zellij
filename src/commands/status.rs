use anyhow::Result;
use colored::Colorize;

use crate::army::Role;
use crate::zellij::ZellijSession;

pub fn execute(session_name: &str) -> Result<()> {
    let session = ZellijSession::new(session_name);

    println!("{}", "=== Overlord Army Status ===".red().bold());
    println!();

    // Check if session exists
    if session.exists()? {
        println!(
            "{} {}",
            "Session:".cyan().bold(),
            session_name.green()
        );
        println!("{} {}", "Status:".cyan().bold(), "ACTIVE".green().bold());
    } else {
        println!(
            "{} {}",
            "Session:".cyan().bold(),
            session_name.yellow()
        );
        println!("{} {}", "Status:".cyan().bold(), "NOT FOUND".red().bold());
        println!();
        println!(
            "{} Use '{}' to summon the army.",
            "Hint:".yellow(),
            "ovld summon".cyan()
        );
        return Ok(());
    }

    println!();
    println!("{}", "=== Army Hierarchy ===".red().bold());
    println!();

    for role in Role::all() {
        let icon = match role {
            Role::Overlord => "👑",
            Role::Strategist => "🗡️",
            Role::LegionImpl => "💪",
            Role::LegionDebug => "🔥",
            Role::LegionDocs => "📜",
        };
        println!("  {} {}", icon, role.display_name());
    }

    println!();
    println!(
        "{} Use '{}' to terminate the session.",
        "Hint:".yellow(),
        "ovld slay".cyan()
    );

    Ok(())
}
