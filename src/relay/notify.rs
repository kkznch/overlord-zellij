use anyhow::Result;
use std::process::{Command, Stdio};
use std::thread;

use crate::army::roles::Role;
use crate::layout;
use crate::logging;

/// Build the JSON payload for the notify plugin.
fn build_payload(pane_id: u32, from: Role) -> String {
    let notification = format!(
        "[MESSAGE from {}] check_inbox ツールで受信メッセージを確認して作業を開始してください。",
        from
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
pub fn notify_pane(session: &str, target: Role, from: Role, plugin_path: &str) -> Result<()> {
    let pane_id = layout::pane_id_for_role(target.as_str())
        .unwrap_or_else(|| panic!("role '{}' not found in PANE_ORDER", target));
    let payload = build_payload(pane_id, from);
    let session = session.to_string();
    let plugin = format!("file:{}", plugin_path);

    logging::debug(&format!(
        "zellij pipe: target={} pane_id={} session={} plugin={}",
        target, pane_id, session, plugin_path
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
    fn test_build_payload_is_valid_json() {
        let payload = build_payload(2, Role::Strategist);
        let parsed: serde_json::Value = serde_json::from_str(&payload).unwrap();
        assert_eq!(parsed["pane_id"], 2);
        assert_eq!(parsed["send_enter"], true);
        assert!(parsed["text"].as_str().unwrap().contains("[MESSAGE from strategist]"));
    }
}
