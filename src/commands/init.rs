use anyhow::Result;
use colored::Colorize;

use crate::config::{config_dir, extract_rituals_to};

pub fn execute(force: bool) -> Result<()> {
    let rituals_dir = config_dir()?.join("rituals");

    if rituals_dir.exists() && !force {
        println!(
            "{} グローバル設定は既に展開済みです: {:?}",
            "Info:".cyan().bold(),
            rituals_dir
        );
        println!(
            "{} 上書きするには `ovld init --force` を使用してください。",
            "ヒント:".yellow()
        );
        return Ok(());
    }

    std::fs::create_dir_all(&rituals_dir)?;
    extract_rituals_to(&rituals_dir)?;

    println!(
        "{} グローバル設定を展開しました: {:?}",
        "Success:".green().bold(),
        rituals_dir
    );

    Ok(())
}
