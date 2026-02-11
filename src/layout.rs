use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

/// Generate KDL layout with absolute paths to ritual files, MCP configs, and working directory
pub fn generate_layout(rituals_dir: &Path, mcp_dir: &Path, cwd: &Path, plugin_path: &Path) -> String {
    let cwd_str = cwd.display();
    let plugin_str = plugin_path.display();
    let ovld_path = env::current_exe().unwrap_or_else(|_| PathBuf::from("ovld"));
    let ovld_str = ovld_path.display();

    let pane_config = |name: &str, size: Option<&str>| -> String {
        let size_attr = size.map(|s| format!(" size=\"{}\"", s)).unwrap_or_default();
        let ritual_path = rituals_dir.join(format!("{}.md", name));
        let mcp_config_path = mcp_dir.join(format!("{}.json", name));
        format!(
            r#"            pane name="{name}"{size_attr} cwd="{cwd_str}" {{
                command "claude"
                args "--system-prompt-file" "{}" "--mcp-config" "{}" "--setting-sources" "project,local" "--allowedTools" "mcp__ovld-relay__*"
            }}"#,
            ritual_path.display(),
            mcp_config_path.display()
        )
    };

    format!(
        r#"layout {{
    // Command tab: Overlord + Strategist (司令部)
    tab name="command" focus=true {{
        pane split_direction="vertical" {{
{overlord}
{strategist}
        }}
        // Notify plugin: single instance for pipe message routing
        pane size=1 borderless=true {{
            plugin location="file:{plugin_str}"
        }}
    }}

    // Battlefield tab: Inferno (主戦場)
    tab name="battlefield" {{
{inferno}
    }}

    // Support tab: Glacier + Shadow + Storm (補助)
    tab name="support" {{
        pane split_direction="horizontal" {{
{glacier}
{shadow}
{storm}
        }}
    }}

    // Dashboard tab: real-time army status (TUI)
    tab name="dashboard" {{
        pane name="dashboard" cwd="{cwd_str}" {{
            command "{ovld_str}"
            args "dashboard"
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
        overlord = pane_config("overlord", Some("30%")),
        strategist = pane_config("strategist", Some("70%")),
        inferno = pane_config("inferno", None),
        glacier = pane_config("glacier", Some("33%")),
        shadow = pane_config("shadow", Some("33%")),
        storm = pane_config("storm", Some("34%")),
        plugin_str = plugin_str,
    )
}

/// Create a temporary file with the generated layout
/// Returns the temp file (keeps it alive) and its path
pub fn create_temp_layout(rituals_dir: &Path, mcp_dir: &Path, cwd: &Path, plugin_path: &Path) -> Result<(NamedTempFile, PathBuf)> {
    let content = generate_layout(rituals_dir, mcp_dir, cwd, plugin_path);

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
        let layout = generate_layout(&rituals_dir, &mcp_dir, &cwd, &plugin_path);

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
        let layout = generate_layout(&rituals_dir, &mcp_dir, &cwd, &plugin_path);

        assert!(layout.contains("tab name=\"command\""));
        assert!(layout.contains("tab name=\"battlefield\""));
        assert!(layout.contains("tab name=\"support\""));
        assert!(layout.contains("tab name=\"dashboard\""));
    }

    #[test]
    fn test_create_temp_layout_creates_file() {
        let rituals_dir = PathBuf::from("/tmp/rituals");
        let mcp_dir = PathBuf::from("/tmp/mcp");
        let cwd = PathBuf::from("/tmp/project");
        let plugin_path = PathBuf::from("/tmp/plugin.wasm");
        let (temp_file, path) = create_temp_layout(&rituals_dir, &mcp_dir, &cwd, &plugin_path).unwrap();

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
        let layout = generate_layout(&rituals_dir, &mcp_dir, &cwd, &plugin_path);

        assert!(layout.contains("command \"claude\""));
        assert!(layout.contains("--system-prompt-file"));
        assert!(layout.contains("/home/user/.config/ovld/rituals/overlord.md"));
        assert!(layout.contains("--mcp-config"));
        assert!(layout.contains("/home/user/.config/ovld/mcp/overlord.json"));
        assert!(layout.contains("cwd=\"/home/user/projects/myproject\""));
        assert!(layout.contains("\"--setting-sources\" \"project,local\""));
        assert!(layout.contains("file:/home/user/.config/ovld/plugins/ovld-notify-plugin.wasm"));
    }
}
