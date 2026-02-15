use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

use crate::army::roles::Role;

/// Pane name for the dashboard (not a Role, so defined separately).
pub const DASHBOARD_PANE_NAME: &str = "dashboard";

/// Terminal pane appearance order in the KDL layout.
/// Zellij assigns Terminal IDs 0..N in this order.
/// **Must match the order of terminal panes in `generate_layout()`.**
const PANE_ORDER: &[&str] = &[
    DASHBOARD_PANE_NAME,            // 0
    Role::Overlord.as_str(),        // 1
    Role::Strategist.as_str(),      // 2
    Role::Glacier.as_str(),         // 3
    Role::Inferno.as_str(),         // 4
    Role::Shadow.as_str(),          // 5
    Role::Storm.as_str(),           // 6
];

/// Look up the terminal pane ID for a given role name.
/// Returns `None` if the role is not in the layout (should never happen for valid roles).
pub fn pane_id_for_role(role: &str) -> Option<u32> {
    PANE_ORDER
        .iter()
        .position(|&name| name == role)
        .map(|i| i as u32)
}

/// Generate KDL layout with absolute paths to ritual files, MCP configs, and working directory.
/// If `sandbox_profile` is provided, wraps `claude` invocations with `sandbox-exec -f <profile>`.
pub fn generate_layout(
    rituals_dir: &Path,
    mcp_dir: &Path,
    cwd: &Path,
    plugin_path: &Path,
    sandbox_profile: Option<&Path>,
) -> String {
    let cwd_str = cwd.display();
    let plugin_str = plugin_path.display();
    let ovld_path = env::current_exe().unwrap_or_else(|_| PathBuf::from("ovld"));
    let ovld_str = ovld_path.display();

    let pane_config = |name: &str, size: Option<&str>, focus: bool| -> String {
        let size_attr = size.map(|s| format!(" size=\"{}\"", s)).unwrap_or_default();
        let focus_attr = if focus { " focus=true" } else { "" };
        let ritual_path = rituals_dir.join(format!("{}.md", name));
        let mcp_config_path = mcp_dir.join(format!("{}.json", name));
        let claude_args = format!(
            "\"--dangerously-skip-permissions\" \"--system-prompt-file\" \"{}\" \"--mcp-config\" \"{}\" \"--setting-sources\" \"user,project,local\" \"--allowedTools\" \"mcp__ovld-relay__*\"",
            ritual_path.display(),
            mcp_config_path.display()
        );
        let (cmd, args) = if let Some(profile) = sandbox_profile {
            (
                "sandbox-exec".to_string(),
                format!("\"-f\" \"{}\" \"claude\" {}", profile.display(), claude_args),
            )
        } else {
            ("claude".to_string(), claude_args)
        };
        format!(
            r#"            pane name="{name}"{size_attr}{focus_attr} cwd="{cwd_str}" {{
                command "{cmd}"
                args {args}
            }}"#,
        )
    };

    format!(
        r#"layout {{
    // Command tab: Dashboard + Overlord + Strategist (司令部)
    tab name="command" focus=true {{
        pane split_direction="vertical" {{
            pane name="{dashboard_name}" size="50%" cwd="{cwd_str}" {{
                command "{ovld_str}"
                args "dashboard"
            }}
            pane split_direction="horizontal" size="50%" {{
{overlord}
{strategist}
            }}
        }}
        // Notify plugin: single instance for pipe message routing
        pane size=1 borderless=true {{
            plugin location="file:{plugin_str}"
        }}
    }}

    // Battlefield tab: Four Heavenly Kings (四天王)
    tab name="battlefield" {{
        pane split_direction="horizontal" {{
            pane split_direction="vertical" size="50%" {{
{glacier}
{inferno}
            }}
            pane split_direction="vertical" size="50%" {{
{shadow}
{storm}
            }}
        }}
    }}

    default_tab_template {{
        pane size=1 borderless=true {{
            plugin location="compact-bar"
        }}
        children
    }}
}}
"#,
        dashboard_name = DASHBOARD_PANE_NAME,
        overlord = pane_config(Role::Overlord.as_str(), Some("50%"), true),
        strategist = pane_config(Role::Strategist.as_str(), Some("50%"), false),
        glacier = pane_config(Role::Glacier.as_str(), Some("50%"), false),
        inferno = pane_config(Role::Inferno.as_str(), Some("50%"), false),
        shadow = pane_config(Role::Shadow.as_str(), Some("50%"), false),
        storm = pane_config(Role::Storm.as_str(), Some("50%"), false),
        plugin_str = plugin_str,
    )
}

/// Create a temporary file with the generated layout
/// Returns the temp file (keeps it alive) and its path
pub fn create_temp_layout(
    rituals_dir: &Path,
    mcp_dir: &Path,
    cwd: &Path,
    plugin_path: &Path,
    sandbox_profile: Option<&Path>,
) -> Result<(NamedTempFile, PathBuf)> {
    let content = generate_layout(rituals_dir, mcp_dir, cwd, plugin_path, sandbox_profile);

    let temp_file = NamedTempFile::with_suffix(".kdl")
        .context("Failed to create temporary layout file")?;

    let path = temp_file.path().to_path_buf();

    fs::write(&path, content)
        .with_context(|| format!("Failed to write temporary layout to {:?}", path))?;

    Ok((temp_file, path))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_generate_layout_contains_all_roles() {
        let rituals_dir = PathBuf::from("/tmp/rituals");
        let mcp_dir = PathBuf::from("/tmp/mcp");
        let cwd = PathBuf::from("/tmp/project");
        let plugin_path = PathBuf::from("/tmp/plugin.wasm");
        let layout = generate_layout(&rituals_dir, &mcp_dir, &cwd, &plugin_path, None);

        for role in &["overlord", "strategist", "inferno", "glacier", "shadow", "storm"] {
            assert!(
                layout.contains(&format!("pane name=\"{}\"", role)),
                "Layout should contain pane for {}",
                role
            );
            assert!(
                layout.contains(&format!("{}.md", role)),
                "Layout should reference ritual file for {}",
                role
            );
        }
    }

    #[test]
    fn test_generate_layout_contains_tab_names() {
        let rituals_dir = PathBuf::from("/tmp/rituals");
        let mcp_dir = PathBuf::from("/tmp/mcp");
        let cwd = PathBuf::from("/tmp/project");
        let plugin_path = PathBuf::from("/tmp/plugin.wasm");
        let layout = generate_layout(&rituals_dir, &mcp_dir, &cwd, &plugin_path, None);

        assert!(layout.contains("tab name=\"command\""));
        assert!(layout.contains("tab name=\"battlefield\""));
        assert!(!layout.contains("tab name=\"support\""));
        assert!(!layout.contains("tab name=\"dashboard\""));
    }

    #[test]
    fn test_generate_layout_dashboard_in_command_tab() {
        let rituals_dir = PathBuf::from("/tmp/rituals");
        let mcp_dir = PathBuf::from("/tmp/mcp");
        let cwd = PathBuf::from("/tmp/project");
        let plugin_path = PathBuf::from("/tmp/plugin.wasm");
        let layout = generate_layout(&rituals_dir, &mcp_dir, &cwd, &plugin_path, None);

        assert!(layout.contains("pane name=\"dashboard\""));
        // Dashboard, overlord, and strategist should all be in the command tab
        let command_tab_start = layout.find("tab name=\"command\"").unwrap();
        let battlefield_tab_start = layout.find("tab name=\"battlefield\"").unwrap();
        let command_section = &layout[command_tab_start..battlefield_tab_start];
        assert!(command_section.contains("pane name=\"dashboard\""));
        assert!(command_section.contains("pane name=\"overlord\""));
        assert!(command_section.contains("pane name=\"strategist\""));
    }

    #[test]
    fn test_generate_layout_four_kings_in_battlefield() {
        let rituals_dir = PathBuf::from("/tmp/rituals");
        let mcp_dir = PathBuf::from("/tmp/mcp");
        let cwd = PathBuf::from("/tmp/project");
        let plugin_path = PathBuf::from("/tmp/plugin.wasm");
        let layout = generate_layout(&rituals_dir, &mcp_dir, &cwd, &plugin_path, None);

        let battlefield_start = layout.find("tab name=\"battlefield\"").unwrap();
        let battlefield_section = &layout[battlefield_start..];
        assert!(battlefield_section.contains("pane name=\"glacier\""));
        assert!(battlefield_section.contains("pane name=\"inferno\""));
        assert!(battlefield_section.contains("pane name=\"shadow\""));
        assert!(battlefield_section.contains("pane name=\"storm\""));
    }

    #[test]
    fn test_create_temp_layout_creates_file() {
        let rituals_dir = PathBuf::from("/tmp/rituals");
        let mcp_dir = PathBuf::from("/tmp/mcp");
        let cwd = PathBuf::from("/tmp/project");
        let plugin_path = PathBuf::from("/tmp/plugin.wasm");
        let (temp_file, path) = create_temp_layout(&rituals_dir, &mcp_dir, &cwd, &plugin_path, None).unwrap();

        assert!(path.exists());
        assert!(path.to_string_lossy().ends_with(".kdl"));
        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("layout {"));
        assert!(content.contains("command \"claude\""));
        drop(temp_file); // Cleanup
    }

    #[test]
    fn test_generate_layout_with_cwd() {
        let rituals_dir = PathBuf::from("/home/user/.config/ovld/rituals");
        let mcp_dir = PathBuf::from("/home/user/.config/ovld/mcp");
        let cwd = PathBuf::from("/home/user/projects/myproject");
        let plugin_path = PathBuf::from("/home/user/.config/ovld/plugins/ovld-notify-plugin.wasm");
        let layout = generate_layout(&rituals_dir, &mcp_dir, &cwd, &plugin_path, None);

        assert!(layout.contains("command \"claude\""));
        assert!(layout.contains("--system-prompt-file"));
        assert!(layout.contains("/home/user/.config/ovld/rituals/overlord.md"));
        assert!(layout.contains("--mcp-config"));
        assert!(layout.contains("/home/user/.config/ovld/mcp/overlord.json"));
        assert!(layout.contains("cwd=\"/home/user/projects/myproject\""));
        assert!(layout.contains("\"--setting-sources\" \"user,project,local\""));
        assert!(layout.contains("file:/home/user/.config/ovld/plugins/ovld-notify-plugin.wasm"));
    }

    #[test]
    fn test_generate_layout_contains_skip_permissions() {
        let rituals_dir = PathBuf::from("/tmp/rituals");
        let mcp_dir = PathBuf::from("/tmp/mcp");
        let cwd = PathBuf::from("/tmp/project");
        let plugin_path = PathBuf::from("/tmp/plugin.wasm");
        let layout = generate_layout(&rituals_dir, &mcp_dir, &cwd, &plugin_path, None);

        assert!(layout.contains("--dangerously-skip-permissions"));
    }

    #[test]
    fn test_generate_layout_with_sandbox() {
        let rituals_dir = PathBuf::from("/tmp/rituals");
        let mcp_dir = PathBuf::from("/tmp/mcp");
        let cwd = PathBuf::from("/tmp/project");
        let plugin_path = PathBuf::from("/tmp/plugin.wasm");
        let sandbox = PathBuf::from("/tmp/sandbox.sb");
        let layout = generate_layout(&rituals_dir, &mcp_dir, &cwd, &plugin_path, Some(&sandbox));

        assert!(layout.contains("command \"sandbox-exec\""));
        assert!(layout.contains("\"-f\" \"/tmp/sandbox.sb\" \"claude\""));
        assert!(!layout.contains("command \"claude\""));
    }

    #[test]
    fn test_pane_order_matches_layout() {
        let rituals_dir = PathBuf::from("/tmp/rituals");
        let mcp_dir = PathBuf::from("/tmp/mcp");
        let cwd = PathBuf::from("/tmp/project");
        let plugin_path = PathBuf::from("/tmp/plugin.wasm");
        let layout = generate_layout(&rituals_dir, &mcp_dir, &cwd, &plugin_path, None);

        // Collect pane name="..." occurrences in order from the layout.
        // This must match PANE_ORDER exactly so pane IDs stay in sync.
        let mut actual_order = Vec::new();
        for segment in layout.split("pane name=\"").skip(1) {
            if let Some(end) = segment.find('"') {
                actual_order.push(&segment[..end]);
            }
        }
        assert_eq!(
            actual_order,
            PANE_ORDER,
            "PANE_ORDER must match the pane appearance order in generate_layout()"
        );
    }

    #[test]
    fn test_pane_id_for_role() {
        assert_eq!(pane_id_for_role("dashboard"), Some(0));
        assert_eq!(pane_id_for_role("nonexistent"), None);

        // Every role defined in Role::all() must be present in PANE_ORDER
        for role in Role::all() {
            assert!(
                pane_id_for_role(role.as_str()).is_some(),
                "Role '{}' is missing from PANE_ORDER in layout.rs",
                role
            );
        }
    }
}
