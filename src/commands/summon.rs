use anyhow::{bail, Result};
use chrono::Utc;
use colored::Colorize;
use std::env;

use crate::config::{
    delete_session_metadata, ensure_default_config, generate_mcp_configs, load_session_metadata,
    relay_dir, resolve_rituals_dir, save_session_metadata, validate_rituals_dir, SessionMetadata,
};
use crate::layout::create_temp_layout;
use crate::relay::store::MessageStore;
use crate::zellij::ZellijSession;

const SESSION_NAME: &str = "overlord";

pub fn execute() -> Result<()> {
    let session = ZellijSession::new(SESSION_NAME);
    let cwd = env::current_dir()?;

    // Check if session already exists
    if session.exists()? {
        if let Some(meta) = load_session_metadata()? {
            bail!(
                "既に {:?} で召喚されています。\n`ovld slay` で撃滅してから再召喚してください。",
                meta.cwd
            );
        } else {
            bail!(
                "既存セッション '{}' があります。\n`ovld slay` で撃滅してから再召喚してください。",
                SESSION_NAME
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
    let store = MessageStore::new(relay.clone());
    store.init()?;

    // Generate per-role MCP configs
    let mcp_dir = relay.join("mcp");
    generate_mcp_configs(&mcp_dir, &relay, SESSION_NAME)?;

    // Save session metadata
    save_session_metadata(&SessionMetadata {
        cwd: cwd.clone(),
        started_at: Utc::now(),
    })?;

    println!(
        "{} {:?} で魔王軍を召喚中...",
        "Overlord:".red().bold(),
        cwd
    );
    println!(
        "{} 儀式ファイル: {:?}",
        "Info:".cyan().bold(),
        rituals_dir
    );

    // Generate layout with absolute paths to ritual files, MCP configs, and cwd
    let (_temp_file, layout_path) = create_temp_layout(&rituals_dir, &mcp_dir, &cwd)?;

    // Start the session - this blocks until Zellij exits
    let result = session.start(layout_path.to_str().unwrap());

    // Clean up session metadata when session ends (regardless of success/failure)
    let _ = delete_session_metadata();

    // Handle the result
    result?;

    println!(
        "{} セッション '{}' が終了しました。",
        "Info:".cyan().bold(),
        SESSION_NAME
    );

    Ok(())
}
