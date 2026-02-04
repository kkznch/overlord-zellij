use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use tempfile::NamedTempFile;

/// Generate KDL layout with absolute paths to ritual files
/// If `no_rituals` is true, panes will run bash instead of claude
pub fn generate_layout(rituals_dir: &PathBuf, no_rituals: bool) -> String {
    let cmd = if no_rituals { "bash" } else { "claude" };

    let pane_config = |name: &str, size: Option<&str>| -> String {
        let size_attr = size.map(|s| format!(" size=\"{}\"", s)).unwrap_or_default();
        if no_rituals {
            format!(
                r#"            pane name="{name}"{size_attr} {{
                command "{cmd}"
            }}"#
            )
        } else {
            let ritual_path = rituals_dir.join(format!("{}.md", name));
            format!(
                r#"            pane name="{name}"{size_attr} {{
                command "{cmd}"
                args ["--system-prompt-file", "{}"]
            }}"#,
                ritual_path.display()
            )
        }
    };

    format!(
        r#"layout {{
    // Command tab: Overlord + Strategist (司令部)
    tab name="command" {{
        pane split_direction="vertical" {{
{overlord}
{strategist}
        }}
    }}

    // Battlefield tab: Inferno (主戦場)
    tab name="battlefield" focus=true {{
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
pub fn create_temp_layout(rituals_dir: &PathBuf, no_rituals: bool) -> Result<(NamedTempFile, PathBuf)> {
    let content = generate_layout(rituals_dir, no_rituals);

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
    fn test_generate_layout_with_rituals() {
        let rituals_dir = PathBuf::from("/home/user/.config/ovld/rituals");
        let layout = generate_layout(&rituals_dir, false);

        assert!(layout.contains("command \"claude\""));
        assert!(layout.contains("--system-prompt-file"));
        assert!(layout.contains("/home/user/.config/ovld/rituals/overlord.md"));
    }

    #[test]
    fn test_generate_layout_no_rituals() {
        let rituals_dir = PathBuf::from("/home/user/.config/ovld/rituals");
        let layout = generate_layout(&rituals_dir, true);

        assert!(layout.contains("command \"bash\""));
        assert!(!layout.contains("--system-prompt-file"));
    }
}
