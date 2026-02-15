use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

/// Generate a macOS Seatbelt (.sb) profile that restricts file writes
/// to the project directory + relay directory + temp + Claude config.
///
/// Uses `(allow default)` base so all reads are permitted.
/// Only `file-write*` is restricted.
pub fn generate_profile(cwd: &Path, relay_dir: &Path) -> String {
    let home = env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());

    format!(
        r#"(version 1)
(allow default)

;; Deny all file writes by default
(deny file-write* (subpath "/"))

;; Allow writes to project directory
(allow file-write* (subpath "{cwd}"))

;; Allow writes to relay directory (MCP message store)
(allow file-write* (subpath "{relay_dir}"))

;; Allow writes to Claude config (auto memory, settings, ~/.claude.json)
(allow file-write* (subpath "{home}/.claude"))
(allow file-write* (regex #"^{home}/\.claude\.json"))

;; Allow writes to Claude CLI cache (MCP logs, etc.)
(allow file-write* (subpath "{home}/Library/Caches/claude-cli-nodejs"))

;; Allow writes to npm logs
(allow file-write* (subpath "{home}/.npm/_logs"))

;; Allow writes to temp directories
(allow file-write* (subpath "/tmp"))
(allow file-write* (subpath "/private/tmp"))
(allow file-write* (subpath "/var/folders"))
(allow file-write* (subpath "/private/var/folders"))

;; Allow writes to devices (null, tty, dtracehelper, ptmx, etc.)
(allow file-write* (subpath "/dev"))
"#,
        cwd = cwd.display(),
        relay_dir = relay_dir.display(),
        home = home,
    )
}

/// Write the Seatbelt profile to a temporary file.
/// Returns the temp file handle (keeps it alive) and its path.
pub fn create_temp_profile(cwd: &Path, relay_dir: &Path) -> Result<(NamedTempFile, PathBuf)> {
    let content = generate_profile(cwd, relay_dir);

    let temp_file = NamedTempFile::with_suffix(".sb")
        .context("Failed to create temporary sandbox profile")?;

    let path = temp_file.path().to_path_buf();

    fs::write(&path, content)
        .with_context(|| format!("Failed to write sandbox profile to {:?}", path))?;

    Ok((temp_file, path))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_generate_profile_contains_deny_write() {
        let profile = generate_profile(
            &PathBuf::from("/home/user/project"),
            &PathBuf::from("/home/user/.config/ovld/relay"),
        );
        assert!(profile.contains("(deny file-write* (subpath \"/\"))"));
    }

    #[test]
    fn test_generate_profile_allows_cwd() {
        let profile = generate_profile(
            &PathBuf::from("/home/user/project"),
            &PathBuf::from("/home/user/.config/ovld/relay"),
        );
        assert!(profile.contains("(allow file-write* (subpath \"/home/user/project\"))"));
    }

    #[test]
    fn test_generate_profile_allows_relay_dir() {
        let profile = generate_profile(
            &PathBuf::from("/home/user/project"),
            &PathBuf::from("/home/user/.config/ovld/relay"),
        );
        assert!(profile.contains("(allow file-write* (subpath \"/home/user/.config/ovld/relay\"))"));
    }

    #[test]
    fn test_generate_profile_allows_claude_config() {
        let profile = generate_profile(
            &PathBuf::from("/tmp/project"),
            &PathBuf::from("/tmp/relay"),
        );
        assert!(profile.contains("(allow file-write* (subpath \""));
        assert!(profile.contains("/.claude\"))"));
    }

    #[test]
    fn test_generate_profile_allows_tmp() {
        let profile = generate_profile(
            &PathBuf::from("/tmp/project"),
            &PathBuf::from("/tmp/relay"),
        );
        assert!(profile.contains("(allow file-write* (subpath \"/tmp\"))"));
        assert!(profile.contains("(allow file-write* (subpath \"/private/tmp\"))"));
    }

    #[test]
    fn test_generate_profile_allows_var_folders() {
        let profile = generate_profile(
            &PathBuf::from("/tmp/project"),
            &PathBuf::from("/tmp/relay"),
        );
        assert!(profile.contains("(allow file-write* (subpath \"/var/folders\"))"));
        assert!(profile.contains("(allow file-write* (subpath \"/private/var/folders\"))"));
    }

    #[test]
    fn test_generate_profile_allows_claude_cache() {
        let profile = generate_profile(
            &PathBuf::from("/tmp/project"),
            &PathBuf::from("/tmp/relay"),
        );
        assert!(profile.contains("Library/Caches/claude-cli-nodejs"));
    }

    #[test]
    fn test_generate_profile_allows_dev() {
        let profile = generate_profile(
            &PathBuf::from("/tmp/project"),
            &PathBuf::from("/tmp/relay"),
        );
        assert!(profile.contains("(allow file-write* (subpath \"/dev\"))"));
    }

    #[test]
    fn test_generate_profile_allows_npm_logs() {
        let profile = generate_profile(
            &PathBuf::from("/tmp/project"),
            &PathBuf::from("/tmp/relay"),
        );
        assert!(profile.contains(".npm/_logs"));
    }

    #[test]
    fn test_generate_profile_allows_home_claude_json() {
        let profile = generate_profile(
            &PathBuf::from("/tmp/project"),
            &PathBuf::from("/tmp/relay"),
        );
        assert!(profile.contains(".claude.json"));
    }

    #[test]
    fn test_generate_profile_has_allow_default() {
        let profile = generate_profile(
            &PathBuf::from("/tmp/project"),
            &PathBuf::from("/tmp/relay"),
        );
        assert!(profile.contains("(allow default)"));
    }

    #[test]
    fn test_create_temp_profile_creates_file() {
        let (temp_file, path) = create_temp_profile(
            &PathBuf::from("/tmp/project"),
            &PathBuf::from("/tmp/relay"),
        )
        .unwrap();

        assert!(path.exists());
        assert!(path.to_string_lossy().ends_with(".sb"));

        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("(version 1)"));
        assert!(content.contains("(deny file-write*"));

        drop(temp_file);
    }
}
