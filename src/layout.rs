use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use tempfile::NamedTempFile;

/// Generate KDL layout with absolute paths to ritual files, MCP configs, and working directory
pub fn generate_layout(rituals_dir: &PathBuf, mcp_dir: &PathBuf, cwd: &PathBuf) -> String {
    let cwd_str = cwd.display();

    let pane_config = |name: &str, size: Option<&str>| -> String {
        let size_attr = size.map(|s| format!(" size=\"{}\"", s)).unwrap_or_default();
        let ritual_path = rituals_dir.join(format!("{}.md", name));
        let mcp_config_path = mcp_dir.join(format!("{}.json", name));
        format!(
            r#"            pane name="{name}"{size_attr} cwd="{cwd_str}" {{
                command "claude"
                args "--system-prompt-file" "{}" "--mcp-config" "{}" "--setting-sources" "project,local"
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
    )
}

/// Create a temporary file with the generated layout
/// Returns the temp file (keeps it alive) and its path
pub fn create_temp_layout(rituals_dir: &PathBuf, mcp_dir: &PathBuf, cwd: &PathBuf) -> Result<(NamedTempFile, PathBuf)> {
    let content = generate_layout(rituals_dir, mcp_dir, cwd);

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
    fn test_generate_layout_with_cwd() {
        let rituals_dir = PathBuf::from("/home/user/.config/ovld/rituals");
        let mcp_dir = PathBuf::from("/home/user/.config/ovld/mcp");
        let cwd = PathBuf::from("/home/user/projects/myproject");
        let layout = generate_layout(&rituals_dir, &mcp_dir, &cwd);

        assert!(layout.contains("command \"claude\""));
        assert!(layout.contains("--system-prompt-file"));
        assert!(layout.contains("/home/user/.config/ovld/rituals/overlord.md"));
        assert!(layout.contains("--mcp-config"));
        assert!(layout.contains("/home/user/.config/ovld/mcp/overlord.json"));
        assert!(layout.contains("cwd=\"/home/user/projects/myproject\""));
        assert!(layout.contains("\"--setting-sources\" \"project,local\""));
    }
}
