use anyhow::{Context, Result};
use std::process::{Command, Stdio};
use std::thread;

use crate::logging;

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
    serde_json::json!({
        "pane_id": pane_id,
        "text": notification,
        "send_enter": true,
    })
    .to_string()
}

/// Send a notification to a target pane via the Zellij notify plugin.
///
/// Uses `zellij pipe` to send a JSON payload to the plugin,
/// which writes directly to the target pane's STDIN without switching focus.
/// Inherits parent ZELLIJ env to connect to the current session via IPC.
///
/// Runs in a background thread because `zellij pipe` blocks for several minutes.
pub fn notify_pane(session: &str, target_role: &str, from_role: &str, plugin_path: &str) -> Result<()> {
    let id = pane_id(target_role)
        .with_context(|| format!("Unknown target role: {}", target_role))?;

    let payload = build_payload(id, from_role);
    let target = target_role.to_string();
    let session = session.to_string();
    let plugin = format!("file:{}", plugin_path);

    logging::debug(&format!(
        "zellij pipe: target={} pane_id={} session={} plugin={}",
        target, id, session, plugin_path
    ));

    // Keep parent env (ZELLIJ, ZELLIJ_SESSION_NAME) so zellij pipe
    // connects to the current session directly via IPC.
    // zellij pipe does not create sessions, so no nested-session risk.
    thread::spawn(move || {
        let result = Command::new("zellij")
            .stdin(Stdio::null())
            .args(["pipe", "--plugin", &plugin, "--name", "send_keys", "--", &payload])
            .output();

        match result {
            Ok(output) if output.status.success() => {
                logging::debug(&format!("zellij pipe: success target={}", target));
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                logging::error(&format!(
                    "zellij pipe failed: target={} exit={:?} stderr={}",
                    target, output.status.code(), stderr.trim()
                ));
            }
            Err(e) => {
                logging::error(&format!("zellij pipe exec failed: target={} err={}", target, e));
            }
        }
    });

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
