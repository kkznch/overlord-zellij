use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::i18n::Lang;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    #[serde(default = "default_poll_interval_secs")]
    pub poll_interval_secs: u64,
    #[serde(default = "default_stale_threshold_secs")]
    pub stale_threshold_secs: i64,
}

fn default_poll_interval_secs() -> u64 {
    2
}

fn default_stale_threshold_secs() -> i64 {
    300
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            poll_interval_secs: default_poll_interval_secs(),
            stale_threshold_secs: default_stale_threshold_secs(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub lang: Lang,
    #[serde(default)]
    pub dashboard: DashboardConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            lang: Lang::En,
            dashboard: DashboardConfig::default(),
        }
    }
}

/// Load config from ~/.config/ovld/config.toml, falling back to defaults.
pub fn load_config() -> AppConfig {
    load_config_inner().unwrap_or_default()
}

fn load_config_inner() -> Result<AppConfig> {
    let path = config_dir()?.join("config.toml");
    let content = fs::read_to_string(&path)?;
    Ok(toml::from_str(&content)?)
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

const SKILL_FILES: [(&str, &str); 1] = [
    ("ovld-grow/SKILL.md", include_str!("../claude/skills/ovld-grow/SKILL.md")),
];

const RITUAL_FILES: [(&str, &str); 6] = [
    ("overlord.md", include_str!("../claude/rituals/overlord.md")),
    ("strategist.md", include_str!("../claude/rituals/strategist.md")),
    ("inferno.md", include_str!("../claude/rituals/inferno.md")),
    ("glacier.md", include_str!("../claude/rituals/glacier.md")),
    ("shadow.md", include_str!("../claude/rituals/shadow.md")),
    ("storm.md", include_str!("../claude/rituals/storm.md")),
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEntry {
    pub cwd: PathBuf,
    pub started_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionRegistry {
    pub sessions: HashMap<String, SessionEntry>,
}

pub fn config_dir() -> Result<PathBuf> {
    let home = env::var("HOME").context("HOME environment variable not set")?;
    Ok(PathBuf::from(home).join(".config").join("ovld"))
}

pub fn session_dir(session_name: &str) -> Result<PathBuf> {
    Ok(config_dir()?.join("sessions").join(session_name))
}

pub fn relay_dir(session_name: &str) -> Result<PathBuf> {
    Ok(session_dir(session_name)?.join("relay"))
}

pub fn knowledge_dir() -> Result<PathBuf> {
    Ok(config_dir()?.join("knowledge"))
}

fn registry_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("registry.json"))
}

pub fn load_registry() -> Result<SessionRegistry> {
    let path = registry_path()?;
    if !path.exists() {
        return Ok(SessionRegistry::default());
    }
    let content = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read session registry from {:?}", path))?;
    let registry: SessionRegistry = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse session registry from {:?}", path))?;
    Ok(registry)
}

pub fn save_registry(registry: &SessionRegistry) -> Result<()> {
    let path = registry_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create config directory: {:?}", parent))?;
    }
    let content = serde_json::to_string_pretty(registry)
        .context("Failed to serialize session registry")?;
    fs::write(&path, content)
        .with_context(|| format!("Failed to write session registry to {:?}", path))?;
    Ok(())
}

pub fn register_session(name: &str, cwd: &Path) -> Result<()> {
    let mut registry = load_registry()?;
    registry.sessions.insert(
        name.to_string(),
        SessionEntry {
            cwd: cwd.to_path_buf(),
            started_at: Utc::now(),
        },
    );
    save_registry(&registry)
}

pub fn unregister_session(name: &str) -> Result<()> {
    let mut registry = load_registry()?;
    registry.sessions.remove(name);
    save_registry(&registry)
}

pub fn find_session_by_cwd(cwd: &Path) -> Result<Option<(String, SessionEntry)>> {
    let registry = load_registry()?;
    Ok(registry
        .sessions
        .into_iter()
        .find(|(_, entry)| entry.cwd == cwd))
}

/// Derive a session name from the working directory.
/// Pattern: `ovld-{sanitized_dirname}` (lowercase, [a-z0-9_-] only).
/// If a collision exists in the registry (same name, different cwd), appends `-2`, `-3`, etc.
pub fn derive_session_name(cwd: &Path) -> Result<String> {
    let dirname = cwd
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unnamed".to_string());

    let sanitized: String = dirname
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
        .collect();

    let base = if sanitized.is_empty() {
        "unnamed".to_string()
    } else {
        sanitized
    };

    let base_name = format!("ovld-{}", base);
    let registry = load_registry()?;

    // No collision
    if !registry.sessions.contains_key(&base_name)
        || registry.sessions[&base_name].cwd == cwd
    {
        return Ok(base_name);
    }

    // Collision: try suffixes
    for i in 2.. {
        let candidate = format!("{}-{}", base_name, i);
        if !registry.sessions.contains_key(&candidate)
            || registry.sessions[&candidate].cwd == cwd
        {
            return Ok(candidate);
        }
    }

    unreachable!()
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

/// Deploy embedded skills to ~/.claude/skills/
pub fn deploy_skills() -> Result<()> {
    let home = env::var("HOME").context("HOME environment variable not set")?;
    let base = PathBuf::from(home).join(".claude").join("skills");
    for (rel_path, content) in SKILL_FILES {
        let path = base.join(rel_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create skill directory: {:?}", parent))?;
        }
        fs::write(&path, content)
            .with_context(|| format!("Failed to write skill file: {:?}", path))?;
    }
    Ok(())
}

pub fn generate_mcp_configs(mcp_dir: &Path, relay_dir: &Path, session_name: &str, plugin_path: &Path, debug: bool) -> Result<()> {
    let knowledge_dir = knowledge_dir()?;
    fs::create_dir_all(mcp_dir)
        .with_context(|| format!("Failed to create MCP config directory: {:?}", mcp_dir))?;

    let ovld_path = env::current_exe().unwrap_or_else(|_| PathBuf::from("ovld"));

    for (filename, _) in RITUAL_FILES {
        let role = filename.trim_end_matches(".md");
        let mut env_vars = serde_json::json!({
            "OVLD_ROLE": role,
            "OVLD_RELAY_DIR": relay_dir.to_string_lossy(),
            "OVLD_KNOWLEDGE_DIR": knowledge_dir.to_string_lossy(),
            "OVLD_SESSION": session_name,
            "OVLD_PLUGIN_PATH": plugin_path.to_string_lossy(),
        });
        if debug {
            env_vars["OVLD_DEBUG"] = serde_json::json!("1");
        }
        let config = serde_json::json!({
            "mcpServers": {
                "ovld-relay": {
                    "command": ovld_path.to_string_lossy(),
                    "args": ["relay"],
                    "env": env_vars
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
    fn test_app_config_default() {
        let config = AppConfig::default();
        assert!(matches!(config.lang, Lang::En));
    }

    #[test]
    fn test_save_default_config() {
        let dir = TempDir::new().unwrap();
        save_default_config(dir.path()).unwrap();
        let path = dir.path().join("config.toml");
        assert!(path.exists());
        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("lang"));
    }

    #[test]
    fn test_save_and_read_config_roundtrip() {
        let dir = TempDir::new().unwrap();
        save_default_config(dir.path()).unwrap();
        let content = fs::read_to_string(dir.path().join("config.toml")).unwrap();
        let config: AppConfig = toml::from_str(&content).unwrap();
        assert!(matches!(config.lang, Lang::En));
    }

    #[test]
    fn test_config_toml_parse_ja() {
        let toml_str = r#"lang = "ja""#;
        let config: AppConfig = toml::from_str(toml_str).unwrap();
        assert!(matches!(config.lang, Lang::Ja));
    }

    #[test]
    fn test_config_toml_parse_default_on_empty() {
        let toml_str = "";
        let config: AppConfig = toml::from_str(toml_str).unwrap();
        assert!(matches!(config.lang, Lang::En));
    }

    #[test]
    fn test_generate_mcp_configs() {
        let dir = TempDir::new().unwrap();
        let mcp_dir = dir.path().join("mcp");
        let relay_dir = dir.path().join("relay");
        let plugin_path = PathBuf::from("/tmp/test-plugin.wasm");
        generate_mcp_configs(&mcp_dir, &relay_dir, "test-session", &plugin_path, false).unwrap();

        // Should create 6 config files (one per role)
        for (filename, _) in RITUAL_FILES {
            let role = filename.trim_end_matches(".md");
            let config_path = mcp_dir.join(format!("{}.json", role));
            assert!(config_path.exists(), "MCP config for {} should exist", role);
            let content = fs::read_to_string(&config_path).unwrap();
            assert!(content.contains("ovld-relay"));
            assert!(content.contains(role));
        }
    }

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
        assert!(content.contains("Overlord"));
    }

    #[test]
    fn test_validate_after_extract() {
        let dir = TempDir::new().unwrap();
        extract_rituals_to(dir.path()).unwrap();
        assert!(validate_rituals_dir(dir.path()).is_ok());
    }

    #[test]
    fn test_app_config_toml_roundtrip() {
        let config = AppConfig { lang: Lang::Ja, ..Default::default() };
        let toml_str = toml::to_string_pretty(&config).unwrap();
        let deserialized: AppConfig = toml::from_str(&toml_str).unwrap();
        assert!(matches!(deserialized.lang, Lang::Ja));
    }

    #[test]
    fn test_lang_all_variants_via_config() {
        for lang in [Lang::En, Lang::Ja] {
            let config = AppConfig { lang, ..Default::default() };
            let toml_str = toml::to_string_pretty(&config).unwrap();
            let deserialized: AppConfig = toml::from_str(&toml_str).unwrap();
            assert_eq!(config.lang, deserialized.lang);
        }
    }

    #[test]
    fn test_session_registry_roundtrip() {
        let dir = TempDir::new().unwrap();
        let registry_file = dir.path().join("registry.json");

        let mut registry = SessionRegistry::default();
        registry.sessions.insert(
            "ovld-myproject".to_string(),
            SessionEntry {
                cwd: PathBuf::from("/tmp/myproject"),
                started_at: Utc::now(),
            },
        );

        let content = serde_json::to_string_pretty(&registry).unwrap();
        fs::write(&registry_file, &content).unwrap();
        let loaded: SessionRegistry =
            serde_json::from_str(&fs::read_to_string(&registry_file).unwrap()).unwrap();
        assert_eq!(loaded.sessions.len(), 1);
        assert!(loaded.sessions.contains_key("ovld-myproject"));
        assert_eq!(
            loaded.sessions["ovld-myproject"].cwd,
            PathBuf::from("/tmp/myproject")
        );
    }

    #[test]
    fn test_derive_session_name_basic() {
        // derive_session_name calls load_registry() which reads from HOME.
        // We test the sanitization logic directly here.
        let sanitize = |s: &str| -> String {
            let lower = s.to_lowercase();
            let clean: String = lower
                .chars()
                .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
                .collect();
            let base = if clean.is_empty() {
                "unnamed".to_string()
            } else {
                clean
            };
            format!("ovld-{}", base)
        };
        assert_eq!(sanitize("my-project"), "ovld-my-project");
        assert_eq!(sanitize("MyApp.v2"), "ovld-myappv2");
        assert_eq!(sanitize("HELLO_world"), "ovld-hello_world");
        assert_eq!(sanitize("..."), "ovld-unnamed");
    }

    #[test]
    fn test_session_dir_path_structure() {
        let dir = session_dir("ovld-myproject").unwrap();
        assert!(
            dir.ends_with("sessions/ovld-myproject"),
            "session_dir should be <config>/sessions/<name>, got {:?}",
            dir
        );
    }

    #[test]
    fn test_relay_dir_path_structure() {
        let dir = relay_dir("ovld-myproject").unwrap();
        assert!(
            dir.ends_with("sessions/ovld-myproject/relay"),
            "relay_dir should be <config>/sessions/<name>/relay, got {:?}",
            dir
        );
    }

    #[test]
    fn test_register_and_unregister_logic() {
        let mut registry = SessionRegistry::default();

        // Register two sessions
        registry.sessions.insert(
            "ovld-alpha".to_string(),
            SessionEntry {
                cwd: PathBuf::from("/tmp/alpha"),
                started_at: Utc::now(),
            },
        );
        registry.sessions.insert(
            "ovld-beta".to_string(),
            SessionEntry {
                cwd: PathBuf::from("/tmp/beta"),
                started_at: Utc::now(),
            },
        );
        assert_eq!(registry.sessions.len(), 2);

        // Unregister one
        registry.sessions.remove("ovld-alpha");
        assert_eq!(registry.sessions.len(), 1);
        assert!(!registry.sessions.contains_key("ovld-alpha"));
        assert!(registry.sessions.contains_key("ovld-beta"));

        // Unregister nonexistent is a no-op
        registry.sessions.remove("ovld-nonexistent");
        assert_eq!(registry.sessions.len(), 1);
    }

    #[test]
    fn test_derive_session_name_collision_suffix() {
        let mut registry = SessionRegistry::default();
        registry.sessions.insert(
            "ovld-app".to_string(),
            SessionEntry {
                cwd: PathBuf::from("/tmp/project-a/app"),
                started_at: Utc::now(),
            },
        );

        // Same derived name, different cwd → needs suffix
        let base_name = "ovld-app";
        let new_cwd = PathBuf::from("/tmp/project-b/app");

        // Simulate derive_session_name collision logic
        let result = if !registry.sessions.contains_key(base_name)
            || registry.sessions[base_name].cwd == new_cwd
        {
            base_name.to_string()
        } else {
            let mut found = None;
            for i in 2..100 {
                let candidate = format!("{}-{}", base_name, i);
                if !registry.sessions.contains_key(&candidate)
                    || registry.sessions[&candidate].cwd == new_cwd
                {
                    found = Some(candidate);
                    break;
                }
            }
            found.unwrap()
        };

        assert_eq!(result, "ovld-app-2");

        // Same cwd → no suffix needed
        let same_cwd = PathBuf::from("/tmp/project-a/app");
        let result_same = if !registry.sessions.contains_key(base_name)
            || registry.sessions[base_name].cwd == same_cwd
        {
            base_name.to_string()
        } else {
            unreachable!()
        };
        assert_eq!(result_same, "ovld-app");
    }

    #[test]
    fn test_find_session_by_cwd_logic() {
        let mut registry = SessionRegistry::default();
        registry.sessions.insert(
            "ovld-foo".to_string(),
            SessionEntry {
                cwd: PathBuf::from("/tmp/foo"),
                started_at: Utc::now(),
            },
        );
        registry.sessions.insert(
            "ovld-bar".to_string(),
            SessionEntry {
                cwd: PathBuf::from("/tmp/bar"),
                started_at: Utc::now(),
            },
        );

        let found = registry
            .sessions
            .iter()
            .find(|(_, e)| e.cwd == PathBuf::from("/tmp/foo"));
        assert!(found.is_some());
        assert_eq!(found.unwrap().0, "ovld-foo");

        let not_found = registry
            .sessions
            .iter()
            .find(|(_, e)| e.cwd == PathBuf::from("/tmp/baz"));
        assert!(not_found.is_none());
    }
}
