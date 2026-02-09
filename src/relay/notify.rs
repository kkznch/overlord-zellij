use anyhow::{bail, Context, Result};
use std::process::{Command, Stdio};

/// Map role name to its terminal pane ID in the layout.
/// Zellij assigns terminal pane IDs sequentially in layout order.
fn pane_id(role: &str) -> Option<u32> {
    match role {
        "overlord" => Some(0),
        "strategist" => Some(1),
        "inferno" => Some(2),
        "glacier" => Some(3),
        "shadow" => Some(4),
        "storm" => Some(5),
        _ => None,
    }
}

/// Build the JSON payload for the notify plugin.
fn build_payload(pane_id: u32, from_role: &str) -> String {
    let notification = format!(
        "[MESSAGE from {}] check_inbox ツールで受信メッセージを確認して作業を開始してください。",
        from_role
    );
    format!(
        r#"{{"pane_id":{},"text":"{}","send_enter":true}}"#,
        pane_id,
        notification.replace('\\', "\\\\").replace('"', "\\\"")
    )
}

/// Send a notification to a target pane via the Zellij notify plugin.
///
/// Uses `zellij pipe` to send a JSON payload to the plugin,
/// which writes directly to the target pane's STDIN without switching focus.
/// Session targeting is done via ZELLIJ_SESSION_NAME env var.
pub fn notify_pane(session: &str, target_role: &str, from_role: &str, plugin_path: &str) -> Result<()> {
    let id = pane_id(target_role)
        .with_context(|| format!("Unknown target role: {}", target_role))?;

    let payload = build_payload(id, from_role);

    let status = Command::new("zellij")
        .stdin(Stdio::null())
        .env("ZELLIJ_SESSION_NAME", session)
        .args([
            "pipe",
            "--plugin", &format!("file:{}", plugin_path),
            "--name", "send_keys",
            "--", &payload,
        ])
        .status()
        .with_context(|| format!("Failed to execute zellij pipe for {}", target_role))?;

    if !status.success() {
        bail!(
            "zellij pipe failed for {} (exit code: {:?})",
            target_role,
            status.code()
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pane_id_valid_roles() {
        assert_eq!(pane_id("overlord"), Some(0));
        assert_eq!(pane_id("strategist"), Some(1));
        assert_eq!(pane_id("inferno"), Some(2));
        assert_eq!(pane_id("glacier"), Some(3));
        assert_eq!(pane_id("shadow"), Some(4));
        assert_eq!(pane_id("storm"), Some(5));
    }

    #[test]
    fn test_pane_id_invalid_role() {
        assert_eq!(pane_id("unknown"), None);
        assert_eq!(pane_id(""), None);
    }

    #[test]
    fn test_build_payload_is_valid_json() {
        let payload = build_payload(2, "strategist");
        let parsed: serde_json::Value = serde_json::from_str(&payload).unwrap();
        assert_eq!(parsed["pane_id"], 2);
        assert_eq!(parsed["send_enter"], true);
        assert!(parsed["text"].as_str().unwrap().contains("[MESSAGE from strategist]"));
    }

    #[test]
    fn test_build_payload_escapes_quotes() {
        // from_role に引用符が入っても壊れないことを確認
        let payload = build_payload(0, r#"test"role"#);
        let parsed: serde_json::Value = serde_json::from_str(&payload).unwrap();
        assert!(parsed["text"].as_str().unwrap().contains(r#"test"role"#));
    }

    #[test]
    fn test_notify_pane_rejects_invalid_role() {
        let result = notify_pane("test", "invalid_role", "overlord", "/tmp/fake.wasm");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unknown target role"));
    }
}
