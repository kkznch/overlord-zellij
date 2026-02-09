use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::i18n::Lang;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub lang: Lang,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self { lang: Lang::En }
    }
}

/// Load config from ~/.config/ovld/config.toml, falling back to defaults.
pub fn load_config() -> AppConfig {
    let Ok(dir) = config_dir() else {
        return AppConfig::default();
    };
    let path = dir.join("config.toml");
    if !path.exists() {
        return AppConfig::default();
    }
    let Ok(content) = fs::read_to_string(&path) else {
        return AppConfig::default();
    };
    toml::from_str(&content).unwrap_or_default()
}

/// Save default config.toml to the specified directory.
pub fn save_default_config(dir: &Path) -> Result<()> {
    let config = AppConfig::default();
    let content = toml::to_string_pretty(&config).context("Failed to serialize config")?;
    let path = dir.join("config.toml");
    fs::write(&path, content)
        .with_context(|| format!("Failed to write config to {:?}", path))?;
    Ok(())
}

const PLUGIN_WASM: &[u8] = include_bytes!("../target/plugin/ovld-notify-plugin.wasm");

const RITUAL_FILES: [(&str, &str); 6] = [
    ("overlord.md", include_str!("../rituals/overlord.md")),
    ("strategist.md", include_str!("../rituals/strategist.md")),
    ("inferno.md", include_str!("../rituals/inferno.md")),
    ("glacier.md", include_str!("../rituals/glacier.md")),
    ("shadow.md", include_str!("../rituals/shadow.md")),
    ("storm.md", include_str!("../rituals/storm.md")),
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMetadata {
    pub cwd: PathBuf,
    pub started_at: DateTime<Utc>,
}

pub fn config_dir() -> Result<PathBuf> {
    let home = env::var("HOME").context("HOME environment variable not set")?;
    Ok(PathBuf::from(home).join(".config").join("ovld"))
}

pub fn relay_dir() -> Result<PathBuf> {
    Ok(config_dir()?.join("relay"))
}

fn session_metadata_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("session.json"))
}

pub fn save_session_metadata(metadata: &SessionMetadata) -> Result<()> {
    let path = session_metadata_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create config directory: {:?}", parent))?;
    }
    let content = serde_json::to_string_pretty(metadata)
        .context("Failed to serialize session metadata")?;
    fs::write(&path, content)
        .with_context(|| format!("Failed to write session metadata to {:?}", path))?;
    Ok(())
}

pub fn load_session_metadata() -> Result<Option<SessionMetadata>> {
    let path = session_metadata_path()?;
    if !path.exists() {
        return Ok(None);
    }
    let content = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read session metadata from {:?}", path))?;
    let metadata: SessionMetadata = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse session metadata from {:?}", path))?;
    Ok(Some(metadata))
}

pub fn delete_session_metadata() -> Result<()> {
    let path = session_metadata_path()?;
    if path.exists() {
        fs::remove_file(&path)
            .with_context(|| format!("Failed to delete session metadata at {:?}", path))?;
    }
    Ok(())
}

/// Resolve rituals directory: ./rituals/ (local) → ~/.config/ovld/rituals/ (global)
pub fn resolve_rituals_dir() -> Result<PathBuf> {
    let local_rituals = env::current_dir()?.join("rituals");
    if local_rituals.is_dir() {
        return Ok(local_rituals);
    }
    Ok(config_dir()?.join("rituals"))
}

/// Create ~/.config/ovld/rituals/ with default ritual files if it doesn't exist
pub fn ensure_default_config() -> Result<()> {
    let rituals_dir = config_dir()?.join("rituals");
    if rituals_dir.exists() {
        return Ok(());
    }
    fs::create_dir_all(&rituals_dir)
        .with_context(|| format!("Failed to create config directory: {:?}", rituals_dir))?;
    extract_rituals_to(&rituals_dir)
}

/// Extract embedded ritual files to the specified directory
pub fn extract_rituals_to(dir: &Path) -> Result<()> {
    for (filename, content) in RITUAL_FILES {
        let path = dir.join(filename);
        fs::write(&path, content)
            .with_context(|| format!("Failed to write ritual file: {:?}", path))?;
    }
    Ok(())
}

/// Validate that all required ritual files exist in the given directory
pub fn validate_rituals_dir(dir: &Path) -> Result<()> {
    let missing: Vec<&str> = RITUAL_FILES
        .iter()
        .filter(|(f, _)| !dir.join(f).exists())
        .map(|(f, _)| *f)
        .collect();

    if !missing.is_empty() {
        anyhow::bail!("Missing ritual files in {:?}: {}", dir, missing.join(", "));
    }
    Ok(())
}

pub fn plugin_dir() -> Result<PathBuf> {
    Ok(config_dir()?.join("plugins"))
}

/// Extract the embedded notify plugin WASM to ~/.config/ovld/plugins/
pub fn extract_plugin() -> Result<PathBuf> {
    let dir = plugin_dir()?;
    fs::create_dir_all(&dir)
        .with_context(|| format!("Failed to create plugin directory: {:?}", dir))?;
    let path = dir.join("ovld-notify-plugin.wasm");
    fs::write(&path, PLUGIN_WASM)
        .with_context(|| format!("Failed to write plugin WASM to {:?}", path))?;
    Ok(path)
}

pub fn generate_mcp_configs(mcp_dir: &Path, relay_dir: &Path, session_name: &str, plugin_path: &Path) -> Result<()> {
    fs::create_dir_all(mcp_dir)
        .with_context(|| format!("Failed to create MCP config directory: {:?}", mcp_dir))?;

    let ovld_path = env::current_exe().unwrap_or_else(|_| PathBuf::from("ovld"));

    for (filename, _) in RITUAL_FILES {
        let role = filename.trim_end_matches(".md");
        let config = serde_json::json!({
            "mcpServers": {
                "ovld-relay": {
                    "command": ovld_path.to_string_lossy(),
                    "args": ["relay"],
                    "env": {
                        "OVLD_ROLE": role,
                        "OVLD_RELAY_DIR": relay_dir.to_string_lossy(),
                        "OVLD_SESSION": session_name,
                        "OVLD_PLUGIN_PATH": plugin_path.to_string_lossy(),
                    }
                }
            }
        });

        let config_path = mcp_dir.join(format!("{}.json", role));
        let content = serde_json::to_string_pretty(&config)
            .context("Failed to serialize MCP config")?;
        fs::write(&config_path, content)
            .with_context(|| format!("Failed to write MCP config for {}: {:?}", role, config_path))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_validate_rituals_dir_all_present() {
        let dir = TempDir::new().unwrap();
        for (f, _) in RITUAL_FILES {
            fs::write(dir.path().join(f), "test content").unwrap();
        }
        assert!(validate_rituals_dir(dir.path()).is_ok());
    }

    #[test]
    fn test_validate_rituals_dir_missing_files() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("overlord.md"), "test").unwrap();
        fs::write(dir.path().join("strategist.md"), "test").unwrap();

        let result = validate_rituals_dir(dir.path());
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("inferno.md"));
        assert!(err_msg.contains("glacier.md"));
    }

    #[test]
    fn test_extract_default_rituals() {
        let dir = TempDir::new().unwrap();
        extract_rituals_to(dir.path()).unwrap();

        for (f, _) in RITUAL_FILES {
            assert!(dir.path().join(f).exists());
        }
        let content = fs::read_to_string(dir.path().join("overlord.md")).unwrap();
        assert!(!content.is_empty());
        assert!(content.contains("魔王"));
    }

    #[test]
    fn test_validate_after_extract() {
        let dir = TempDir::new().unwrap();
        extract_rituals_to(dir.path()).unwrap();
        assert!(validate_rituals_dir(dir.path()).is_ok());
    }
}
