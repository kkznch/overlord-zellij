use anyhow::{bail, Context, Result};
use chrono::Utc;
use colored::Colorize;
use std::env;

use crate::config::{
    config_dir, ensure_default_config, extract_plugin, generate_mcp_configs, knowledge_dir,
    load_session_metadata, relay_dir, resolve_rituals_dir, save_session_metadata,
    validate_rituals_dir, SessionMetadata, AppConfig,
};
use crate::i18n;
use crate::layout::create_temp_layout;
use crate::logging;
use crate::relay::store::MessageStore;
use crate::zellij::ZellijSession;
use crate::SESSION_NAME;

pub fn execute(config: &AppConfig, debug: bool, sandbox: bool) -> Result<()> {
    let lang = config.lang;
    let session = ZellijSession::new(SESSION_NAME);
    let cwd = env::current_dir()?;

    // Check if session already exists
    if session.exists()? {
        if let Some(meta) = load_session_metadata()? {
            bail!(
                "{}",
                i18n::tf("summon.already_exists_with_cwd", lang, &[("cwd", &i18n::path_str(&meta.cwd))])
            );
        } else {
            bail!(
                "{}",
                i18n::tf("summon.already_exists", lang, &[("name", SESSION_NAME)])
            );
        }
    }

    // Ensure default config exists (creates ~/.config/ovld/rituals/ if needed)
    ensure_default_config()?;

    // Resolve rituals directory (local first, then global)
    let rituals_dir = resolve_rituals_dir()?;

    // Validate ritual files exist
    validate_rituals_dir(&rituals_dir)?;

    // Initialize relay directory structure
    let relay = relay_dir()?;
    let knowledge = knowledge_dir()?;
    std::fs::create_dir_all(&knowledge)?;
    let store = MessageStore::new(relay.clone())
        .with_knowledge_dir(knowledge);
    store.init()?;

    // Extract notify plugin WASM
    let plugin_path = extract_plugin()?;

    // Generate per-role MCP configs
    let mcp_dir = relay.join("mcp");
    generate_mcp_configs(&mcp_dir, &relay, SESSION_NAME, &plugin_path, debug)?;

    // Save session metadata
    save_session_metadata(&SessionMetadata {
        cwd: cwd.clone(),
        started_at: Utc::now(),
    })?;

    println!(
        "{} {}",
        "Overlord:".red().bold(),
        i18n::tf("summon.starting", lang, &[("cwd", &i18n::path_str(&cwd))])
    );
    println!(
        "{} {}",
        "Info:".cyan().bold(),
        i18n::tf("summon.ritual_files", lang, &[("path", &i18n::path_str(&rituals_dir))])
    );

    // Create sandbox profile if enabled (macOS only)
    let _sandbox_profile = if sandbox {
        if cfg!(target_os = "macos") {
            let ovld_config_dir = config_dir()?;
            let (temp, path) = crate::sandbox::create_temp_profile(&cwd, &ovld_config_dir)?;
            println!(
                "{} {}",
                "Info:".cyan().bold(),
                "Sandbox enabled: file writes restricted to project directory"
            );
            Some((temp, path))
        } else {
            eprintln!(
                "{} {}",
                "Warning:".yellow().bold(),
                "Sandbox is only supported on macOS. Skipping."
            );
            None
        }
    } else {
        None
    };
    let sandbox_path = _sandbox_profile.as_ref().map(|(_, p)| p.as_path());

    // Generate layout with absolute paths to ritual files, MCP configs, and cwd
    let (_temp_file, layout_path) = create_temp_layout(&rituals_dir, &mcp_dir, &cwd, &plugin_path, sandbox_path)?;

    logging::info(&format!("summon: starting session (cwd={})", cwd.display()));

    // Start the session - this blocks until Zellij exits
    let layout_str = layout_path.to_str().context("layout path is not valid UTF-8")?;
    let result = session.start(layout_str);

    // Clean up EXITED session if it still exists (best-effort)
    if session.exists().unwrap_or(false) {
        let _ = session.kill();
        let _ = session.delete(true);
    }
    super::cleanup_session_data();

    // Handle the result
    result?;

    logging::info("summon: session ended");

    println!(
        "{} {}",
        "Info:".cyan().bold(),
        i18n::tf("summon.session_ended", lang, &[("name", SESSION_NAME)])
    );

    Ok(())
}
