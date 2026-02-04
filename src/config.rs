use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

/// Session metadata stored in ~/.config/ovld/session.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMetadata {
    pub cwd: PathBuf,
    pub started_at: DateTime<Utc>,
}

/// Path to session metadata file: ~/.config/ovld/session.json
pub fn session_metadata_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("session.json"))
}

/// Save session metadata to file
pub fn save_session_metadata(metadata: &SessionMetadata) -> Result<()> {
    let path = session_metadata_path()?;

    // Ensure config directory exists
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

/// Load session metadata from file (returns None if file doesn't exist)
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

/// Delete session metadata file
pub fn delete_session_metadata() -> Result<()> {
    let path = session_metadata_path()?;

    if path.exists() {
        fs::remove_file(&path)
            .with_context(|| format!("Failed to delete session metadata at {:?}", path))?;
    }

    Ok(())
}

/// Global config directory: ~/.config/ovld/
pub fn config_dir() -> Result<PathBuf> {
    let home = env::var("HOME").context("HOME environment variable not set")?;
    Ok(PathBuf::from(home).join(".config").join("ovld"))
}

/// Global rituals directory: ~/.config/ovld/rituals/
pub fn global_rituals_dir() -> Result<PathBuf> {
    Ok(config_dir()?.join("rituals"))
}

/// Resolve rituals directory with local-first priority
/// 1. Check ./rituals/ in current directory
/// 2. Fall back to ~/.config/ovld/rituals/
pub fn resolve_rituals_dir() -> Result<PathBuf> {
    let current_dir = env::current_dir()?;
    let local_rituals = current_dir.join("rituals");

    if local_rituals.exists() && local_rituals.is_dir() {
        return Ok(local_rituals);
    }

    let global = global_rituals_dir()?;
    if global.exists() && global.is_dir() {
        return Ok(global);
    }

    // If neither exists, return global path (will be created by ensure_default_config)
    Ok(global)
}

/// Embedded default ritual files
const DEFAULT_OVERLORD_RITUAL: &str = include_str!("../rituals/overlord.md");
const DEFAULT_STRATEGIST_RITUAL: &str = include_str!("../rituals/strategist.md");
const DEFAULT_INFERNO_RITUAL: &str = include_str!("../rituals/inferno.md");
const DEFAULT_GLACIER_RITUAL: &str = include_str!("../rituals/glacier.md");
const DEFAULT_SHADOW_RITUAL: &str = include_str!("../rituals/shadow.md");
const DEFAULT_STORM_RITUAL: &str = include_str!("../rituals/storm.md");

/// Ensure default config exists, creating it if necessary
pub fn ensure_default_config() -> Result<()> {
    let rituals_dir = global_rituals_dir()?;

    if rituals_dir.exists() {
        return Ok(());
    }

    // Create directory
    fs::create_dir_all(&rituals_dir)
        .with_context(|| format!("Failed to create config directory: {:?}", rituals_dir))?;

    // Extract default rituals
    extract_default_rituals(&rituals_dir)?;

    Ok(())
}

/// Extract embedded ritual files to the specified directory
fn extract_default_rituals(rituals_dir: &PathBuf) -> Result<()> {
    let rituals = [
        ("overlord.md", DEFAULT_OVERLORD_RITUAL),
        ("strategist.md", DEFAULT_STRATEGIST_RITUAL),
        ("inferno.md", DEFAULT_INFERNO_RITUAL),
        ("glacier.md", DEFAULT_GLACIER_RITUAL),
        ("shadow.md", DEFAULT_SHADOW_RITUAL),
        ("storm.md", DEFAULT_STORM_RITUAL),
    ];

    for (filename, content) in rituals {
        let path = rituals_dir.join(filename);
        fs::write(&path, content)
            .with_context(|| format!("Failed to write ritual file: {:?}", path))?;
    }

    Ok(())
}

/// Validate that all required ritual files exist in the given directory
pub fn validate_rituals_dir(rituals_dir: &PathBuf) -> Result<()> {
    let required_files = [
        "overlord.md",
        "strategist.md",
        "inferno.md",
        "glacier.md",
        "shadow.md",
        "storm.md",
    ];

    let mut missing = Vec::new();
    for filename in required_files {
        let path = rituals_dir.join(filename);
        if !path.exists() {
            missing.push(filename);
        }
    }

    if !missing.is_empty() {
        anyhow::bail!(
            "Missing ritual files in {:?}: {}",
            rituals_dir,
            missing.join(", ")
        );
    }

    Ok(())
}

/// Extract default rituals to the specified directory (public for testing)
pub fn extract_rituals_to(rituals_dir: &PathBuf) -> Result<()> {
    extract_default_rituals(rituals_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_validate_rituals_dir_all_present() {
        let temp_dir = TempDir::new().unwrap();
        let rituals_dir = temp_dir.path().to_path_buf();

        // Create all required files
        let files = [
            "overlord.md",
            "strategist.md",
            "inferno.md",
            "glacier.md",
            "shadow.md",
            "storm.md",
        ];
        for file in files {
            fs::write(rituals_dir.join(file), "test content").unwrap();
        }

        // Should succeed
        assert!(validate_rituals_dir(&rituals_dir).is_ok());
    }

    #[test]
    fn test_validate_rituals_dir_missing_files() {
        let temp_dir = TempDir::new().unwrap();
        let rituals_dir = temp_dir.path().to_path_buf();

        // Create only some files
        fs::write(rituals_dir.join("overlord.md"), "test").unwrap();
        fs::write(rituals_dir.join("strategist.md"), "test").unwrap();

        // Should fail
        let result = validate_rituals_dir(&rituals_dir);
        assert!(result.is_err());

        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("inferno.md"));
        assert!(err_msg.contains("glacier.md"));
    }

    #[test]
    fn test_extract_default_rituals() {
        let temp_dir = TempDir::new().unwrap();
        let rituals_dir = temp_dir.path().to_path_buf();

        // Extract rituals
        extract_rituals_to(&rituals_dir).unwrap();

        // Verify all files exist
        assert!(rituals_dir.join("overlord.md").exists());
        assert!(rituals_dir.join("strategist.md").exists());
        assert!(rituals_dir.join("inferno.md").exists());
        assert!(rituals_dir.join("glacier.md").exists());
        assert!(rituals_dir.join("shadow.md").exists());
        assert!(rituals_dir.join("storm.md").exists());

        // Verify content is not empty
        let overlord_content = fs::read_to_string(rituals_dir.join("overlord.md")).unwrap();
        assert!(!overlord_content.is_empty());
        assert!(overlord_content.contains("魔王"));
    }

    #[test]
    fn test_validate_after_extract() {
        let temp_dir = TempDir::new().unwrap();
        let rituals_dir = temp_dir.path().to_path_buf();

        // Extract and then validate
        extract_rituals_to(&rituals_dir).unwrap();
        assert!(validate_rituals_dir(&rituals_dir).is_ok());
    }
}
