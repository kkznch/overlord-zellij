use anyhow::{Context as _, Result};
use minijinja::{context, Environment};
use serde::Serialize;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

use crate::army::roles::Role;

/// Pane name for the dashboard (not a Role, so defined separately).
pub const DASHBOARD_PANE_NAME: &str = "dashboard";

/// Terminal pane appearance order in the KDL layout.
/// Zellij assigns Terminal IDs 0..N in this order.
/// **Must match the order of terminal panes in the template (`layout.kdl.j2`).**
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

/// Context for a single agent pane in the template.
#[derive(Serialize)]
struct AgentContext {
    name: String,
    size: String,
    focus: bool,
    claude_args: String,
}

/// Build an agent context for template rendering.
fn build_agent(
    name: &str,
    size: &str,
    focus: bool,
    rituals_dir: &Path,
    mcp_dir: &Path,
) -> AgentContext {
    let ritual = rituals_dir.join(format!("{}.md", name));
    let mcp = mcp_dir.join(format!("{}.json", name));
    let claude_args = format!(
        "\"--dangerously-skip-permissions\" \"--system-prompt-file\" \"{}\" \"--mcp-config\" \"{}\" \"--setting-sources\" \"user,project,local\" \"--allowedTools\" \"mcp__ovld-relay__*\"",
        ritual.display(),
        mcp.display()
    );
    AgentContext {
        name: name.to_string(),
        size: size.to_string(),
        focus,
        claude_args,
    }
}

/// Generate KDL layout with absolute paths to ritual files, MCP configs, and working directory.
/// If `sandbox_profile` is provided, wraps `claude` invocations with `sandbox-exec -f <profile>`.
pub fn generate_layout(
    rituals_dir: &Path,
    mcp_dir: &Path,
    cwd: &Path,
    plugin_path: &Path,
    sandbox_profile: Option<&Path>,
) -> Result<String> {
    let ovld_path = env::current_exe().unwrap_or_else(|_| PathBuf::from("ovld"));

    let command_agents = vec![
        build_agent(Role::Overlord.as_str(), "50%", true, rituals_dir, mcp_dir),
        build_agent(Role::Strategist.as_str(), "50%", false, rituals_dir, mcp_dir),
    ];
    let battlefield_rows = vec![
        vec![
            build_agent(Role::Glacier.as_str(), "50%", false, rituals_dir, mcp_dir),
            build_agent(Role::Inferno.as_str(), "50%", false, rituals_dir, mcp_dir),
        ],
        vec![
            build_agent(Role::Shadow.as_str(), "50%", false, rituals_dir, mcp_dir),
            build_agent(Role::Storm.as_str(), "50%", false, rituals_dir, mcp_dir),
        ],
    ];

    let mut env = Environment::new();
    env.add_template("layout", include_str!("layout.kdl.j2"))
        .context("failed to add layout template")?;
    let tmpl = env
        .get_template("layout")
        .context("failed to get layout template")?;

    tmpl.render(context! {
        dashboard_name => DASHBOARD_PANE_NAME,
        cwd => cwd.display().to_string(),
        ovld_path => ovld_path.display().to_string(),
        plugin_path => plugin_path.display().to_string(),
        sandbox_profile => sandbox_profile.map(|p| p.display().to_string()),
        command_agents,
        battlefield_rows,
    })
    .context("failed to render layout template")
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
    let content = generate_layout(rituals_dir, mcp_dir, cwd, plugin_path, sandbox_profile)?;

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
        let layout = generate_layout(&rituals_dir, &mcp_dir, &cwd, &plugin_path, None).unwrap();

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
        let layout = generate_layout(&rituals_dir, &mcp_dir, &cwd, &plugin_path, None).unwrap();

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
        let layout = generate_layout(&rituals_dir, &mcp_dir, &cwd, &plugin_path, None).unwrap();

        assert!(layout.contains("pane name=\"dashboard\""));
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
        let layout = generate_layout(&rituals_dir, &mcp_dir, &cwd, &plugin_path, None).unwrap();

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
        let (temp_file, path) =
            create_temp_layout(&rituals_dir, &mcp_dir, &cwd, &plugin_path, None).unwrap();

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
        let plugin_path =
            PathBuf::from("/home/user/.config/ovld/plugins/ovld-notify-plugin.wasm");
        let layout = generate_layout(&rituals_dir, &mcp_dir, &cwd, &plugin_path, None).unwrap();

        assert!(layout.contains("command \"claude\""));
        assert!(layout.contains("--system-prompt-file"));
        assert!(layout.contains("/home/user/.config/ovld/rituals/overlord.md"));
        assert!(layout.contains("--mcp-config"));
        assert!(layout.contains("/home/user/.config/ovld/mcp/overlord.json"));
        assert!(layout.contains("cwd=\"/home/user/projects/myproject\""));
        assert!(layout.contains("\"--setting-sources\" \"user,project,local\""));
        assert!(layout.contains(
            "file:/home/user/.config/ovld/plugins/ovld-notify-plugin.wasm"
        ));
    }

    #[test]
    fn test_generate_layout_contains_skip_permissions() {
        let rituals_dir = PathBuf::from("/tmp/rituals");
        let mcp_dir = PathBuf::from("/tmp/mcp");
        let cwd = PathBuf::from("/tmp/project");
        let plugin_path = PathBuf::from("/tmp/plugin.wasm");
        let layout = generate_layout(&rituals_dir, &mcp_dir, &cwd, &plugin_path, None).unwrap();

        assert!(layout.contains("--dangerously-skip-permissions"));
    }

    #[test]
    fn test_generate_layout_with_sandbox() {
        let rituals_dir = PathBuf::from("/tmp/rituals");
        let mcp_dir = PathBuf::from("/tmp/mcp");
        let cwd = PathBuf::from("/tmp/project");
        let plugin_path = PathBuf::from("/tmp/plugin.wasm");
        let sandbox = PathBuf::from("/tmp/sandbox.sb");
        let layout = generate_layout(
            &rituals_dir,
            &mcp_dir,
            &cwd,
            &plugin_path,
            Some(&sandbox),
        )
        .unwrap();

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
        let layout = generate_layout(&rituals_dir, &mcp_dir, &cwd, &plugin_path, None).unwrap();

        // Collect pane name="..." occurrences in order from the layout.
        // This must match PANE_ORDER exactly so pane IDs stay in sync.
        let mut actual_order = Vec::new();
        for segment in layout.split("pane name=\"").skip(1) {
            if let Some(end) = segment.find('"') {
                actual_order.push(&segment[..end]);
            }
        }
        assert_eq!(
            actual_order, PANE_ORDER,
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
