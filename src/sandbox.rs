use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

/// Discover the git repository root from `cwd`, walking up parent directories.
/// Returns `None` if not inside a git repo, or if the repo root equals `cwd`.
/// For worktrees, returns the main repository root (not the worktree itself).
fn resolve_git_repo_root(cwd: &Path) -> Option<PathBuf> {
    use gix_discover::repository::Path as RepoPath;
    let cwd = cwd.canonicalize().ok()?;
    let (repo_path, _trust) = gix_discover::upwards(&cwd).ok()?;
    let repo_root = match repo_path {
        RepoPath::LinkedWorkTree { git_dir, .. } => {
            // git_dir = /main-repo/.git/worktrees/<name> → 3 levels up
            git_dir.canonicalize().ok()?.ancestors().nth(3).map(Path::to_path_buf)?
        }
        RepoPath::WorkTree(work_dir) => work_dir.canonicalize().ok()?,
        RepoPath::Repository(_) => return None, // bare repo
    };
    if repo_root == cwd {
        return None;
    }
    Some(repo_root)
}

/// Generate a macOS Seatbelt (.sb) profile that restricts file writes
/// to the project directory + ovld config directory + temp + Claude config.
///
/// Uses `(allow default)` base so all reads are permitted.
/// Only `file-write*` is restricted.
///
/// If `cwd` is inside a git worktree, the main repository's root directory
/// is also permitted for writes (git needs access to shared objects/refs).
pub fn generate_profile(cwd: &Path, config_dir: &Path) -> String {
    let home = env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());

    let git_repo_root_rule = resolve_git_repo_root(cwd)
        .map(|root| {
            format!(
                "\n;; Allow writes to git repository root (worktree support)\n(allow file-write* (subpath \"{}\"))\n",
                root.display()
            )
        })
        .unwrap_or_default();

    format!(
        r#"(version 1)
(allow default)

;; Deny all file writes by default
(deny file-write* (subpath "/"))

;; Allow writes to project directory
(allow file-write* (subpath "{cwd}"))
{git_repo_root_rule}
;; Allow writes to ovld config directory (relay, knowledge, etc.)
(allow file-write* (subpath "{config_dir}"))

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
        git_repo_root_rule = git_repo_root_rule,
        config_dir = config_dir.display(),
        home = home,
    )
}

/// Write the Seatbelt profile to a temporary file.
/// Returns the temp file handle (keeps it alive) and its path.
pub fn create_temp_profile(cwd: &Path, config_dir: &Path) -> Result<(NamedTempFile, PathBuf)> {
    let content = generate_profile(cwd, config_dir);

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
    use std::process::Command;

    fn git(dir: &Path, args: &[&str]) {
        let out = Command::new("git").args(args).current_dir(dir).output().unwrap();
        assert!(out.status.success(), "git {:?} failed: {}", args, String::from_utf8_lossy(&out.stderr));
    }

    fn test_profile() -> String {
        generate_profile(
            &PathBuf::from("/home/user/project"),
            &PathBuf::from("/home/user/.config/ovld"),
        )
    }

    #[test]
    fn test_generate_profile_contains_deny_write() {
        assert!(test_profile().contains("(deny file-write* (subpath \"/\"))"));
    }

    #[test]
    fn test_generate_profile_allows_cwd() {
        assert!(test_profile().contains("(allow file-write* (subpath \"/home/user/project\"))"));
    }

    #[test]
    fn test_generate_profile_allows_config_dir() {
        assert!(test_profile().contains("(allow file-write* (subpath \"/home/user/.config/ovld\"))"));
    }

    #[test]
    fn test_generate_profile_allows_claude_config() {
        let profile = test_profile();
        assert!(profile.contains("(allow file-write* (subpath \""));
        assert!(profile.contains("/.claude\"))"));
    }

    #[test]
    fn test_generate_profile_allows_tmp() {
        let profile = test_profile();
        assert!(profile.contains("(allow file-write* (subpath \"/tmp\"))"));
        assert!(profile.contains("(allow file-write* (subpath \"/private/tmp\"))"));
    }

    #[test]
    fn test_generate_profile_allows_var_folders() {
        let profile = test_profile();
        assert!(profile.contains("(allow file-write* (subpath \"/var/folders\"))"));
        assert!(profile.contains("(allow file-write* (subpath \"/private/var/folders\"))"));
    }

    #[test]
    fn test_generate_profile_allows_claude_cache() {
        assert!(test_profile().contains("Library/Caches/claude-cli-nodejs"));
    }

    #[test]
    fn test_generate_profile_allows_dev() {
        assert!(test_profile().contains("(allow file-write* (subpath \"/dev\"))"));
    }

    #[test]
    fn test_generate_profile_allows_npm_logs() {
        assert!(test_profile().contains(".npm/_logs"));
    }

    #[test]
    fn test_generate_profile_allows_home_claude_json() {
        assert!(test_profile().contains(".claude.json"));
    }

    #[test]
    fn test_generate_profile_has_allow_default() {
        assert!(test_profile().contains("(allow default)"));
    }

    #[test]
    fn test_create_temp_profile_creates_file() {
        let (temp_file, path) = create_temp_profile(
            &PathBuf::from("/tmp/project"),
            &PathBuf::from("/tmp/ovld"),
        )
        .unwrap();

        assert!(path.exists());
        assert!(path.to_string_lossy().ends_with(".sb"));

        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("(version 1)"));
        assert!(content.contains("(deny file-write*"));

        drop(temp_file);
    }

    #[test]
    fn test_resolve_git_repo_root_at_repo_root() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path().join("repo");
        fs::create_dir_all(&repo).unwrap();
        git(&repo, &["init"]);
        // cwd == repo root → None (already covered)
        assert!(resolve_git_repo_root(&repo).is_none());
    }

    #[test]
    fn test_resolve_git_repo_root_in_subdir() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path().join("repo");
        let subdir = repo.join("src").join("deep");
        fs::create_dir_all(&subdir).unwrap();
        git(&repo, &["init"]);
        // cwd is subdir → returns repo root
        let result = resolve_git_repo_root(&subdir);
        assert!(result.is_some());
        assert_eq!(
            result.unwrap().canonicalize().unwrap(),
            repo.canonicalize().unwrap()
        );
    }

    #[test]
    fn test_resolve_git_repo_root_no_git() {
        let tmp = tempfile::tempdir().unwrap();
        assert!(resolve_git_repo_root(tmp.path()).is_none());
    }

    #[test]
    fn test_resolve_git_repo_root_worktree() {
        let tmp = tempfile::tempdir().unwrap();
        let main_repo = tmp.path().join("main");
        fs::create_dir_all(&main_repo).unwrap();
        git(&main_repo, &["init"]);
        git(&main_repo, &["commit", "--allow-empty", "-m", "init"]);

        let wt = tmp.path().join("wt");
        git(&main_repo, &["worktree", "add", wt.to_str().unwrap(), "-b", "wt-branch"]);

        let result = resolve_git_repo_root(&wt);
        assert!(result.is_some());
        assert_eq!(
            result.unwrap().canonicalize().unwrap(),
            main_repo.canonicalize().unwrap()
        );
    }

    #[test]
    fn test_generate_profile_no_git_repo_root_rule() {
        let profile = test_profile();
        assert!(!profile.contains("worktree support"));
    }
}
