use anyhow::{Context, Result};
use colored::Colorize;
use std::env;

use crate::config::{
    config_dir, derive_session_name, ensure_default_config, extract_plugin, find_session_by_cwd,
    generate_mcp_configs, knowledge_dir, register_session, relay_dir, resolve_rituals_dir,
    unregister_session, validate_rituals_dir, AppConfig,
};
use crate::i18n;
use crate::layout::create_temp_layout;
use crate::logging;
use crate::relay::store::MessageStore;
use crate::zellij::ZellijSession;

pub fn execute(config: &AppConfig, debug: bool, sandbox: bool) -> Result<()> {
    let lang = config.lang;
    let cwd = env::current_dir()?;

    // Check if a session already exists for this cwd
    if let Some((existing_name, _entry)) = find_session_by_cwd(&cwd)? {
        let session = ZellijSession::new(&existing_name);
        if session.exists()? {
            // Session is alive — auto-attach
            println!(
                "{} {}",
                "Info:".cyan().bold(),
                i18n::tf("summon.attaching", lang, &[("name", &existing_name)])
            );
            let result = session.attach();
            super::cleanup_session_data(&existing_name);
            return result;
        }
        // Session is dead — purge orphan and recreate
        let _ = unregister_session(&existing_name);
    }

    // Derive a new session name from cwd
    let session_name = derive_session_name(&cwd)?;
    let session = ZellijSession::new(&session_name);

    // Double-check: if a Zellij session with this name already exists (outside registry), bail
    if session.exists()? {
        anyhow::bail!(
            "{}",
            i18n::tf("summon.already_exists", lang, &[("name", &session_name)])
        );
    }

    // Ensure default config exists (creates ~/.config/ovld/rituals/ if needed)
    ensure_default_config()?;

    // Resolve rituals directory (local first, then global)
    let rituals_dir = resolve_rituals_dir()?;

    // Validate ritual files exist
    validate_rituals_dir(&rituals_dir)?;

    // Initialize relay directory structure (per-session)
    let relay = relay_dir(&session_name)?;
    let knowledge = knowledge_dir()?;
    std::fs::create_dir_all(&knowledge)?;
    let store = MessageStore::new(relay.clone())
        .with_knowledge_dir(knowledge);
    store.init()?;

    // Extract notify plugin WASM
    let plugin_path = extract_plugin()?;

    // Generate per-role MCP configs
    let mcp_dir = relay.join("mcp");
    generate_mcp_configs(&mcp_dir, &relay, &session_name, &plugin_path, debug)?;

    // Register session in registry
    register_session(&session_name, &cwd)?;

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
                "{} Sandbox enabled: file writes restricted to project directory",
                "Info:".cyan().bold()
            );
            Some((temp, path))
        } else {
            eprintln!(
                "{} Sandbox is only supported on macOS. Skipping.",
                "Warning:".yellow().bold()
            );
            None
        }
    } else {
        None
    };
    let sandbox_path = _sandbox_profile.as_ref().map(|(_, p)| p.as_path());

    // Generate layout with absolute paths to ritual files, MCP configs, and cwd
    let (_temp_file, layout_path) = create_temp_layout(
        &rituals_dir, &mcp_dir, &cwd, &plugin_path, sandbox_path, &session_name, &relay,
    )?;

    logging::info(&format!("summon: starting session {} (cwd={})", session_name, cwd.display()));

    // Start the session - this blocks until Zellij exits
    let layout_str = layout_path.to_str().context("layout path is not valid UTF-8")?;
    let result = session.start(layout_str);

    // Clean up session (kill + unregister + delete relay data)
    super::cleanup_session_data(&session_name);

    // Handle the result
    result?;

    logging::info("summon: session ended");

    println!(
        "{} {}",
        "Info:".cyan().bold(),
        i18n::tf("summon.session_ended", lang, &[("name", &session_name)])
    );

    Ok(())
}
