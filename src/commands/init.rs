use anyhow::Result;
use colored::Colorize;

use crate::config::{config_dir, deploy_skills, extract_rituals_to, save_default_config, AppConfig};
use crate::i18n;

pub fn execute(force: bool, config: &AppConfig) -> Result<()> {
    let lang = config.lang;
    let config_path = config_dir()?;
    let rituals_dir = config_path.join("rituals");

    if rituals_dir.exists() && !force {
        println!(
            "{} {}",
            "Info:".cyan().bold(),
            i18n::tf("init.already_exists", lang, &[("path", &i18n::path_str(&rituals_dir))])
        );
        println!(
            "{} {}",
            "Hint:".yellow(),
            i18n::t("init.hint_force", lang)
        );
        return Ok(());
    }

    std::fs::create_dir_all(&rituals_dir)?;
    extract_rituals_to(&rituals_dir)?;
    let config_toml = config_path.join("config.toml");
    if !config_toml.exists() {
        save_default_config(&config_path)?;
    }
    deploy_skills()?;

    println!(
        "{} {}",
        "Success:".green().bold(),
        i18n::tf("init.success", lang, &[("path", &i18n::path_str(&rituals_dir))])
    );

    Ok(())
}
