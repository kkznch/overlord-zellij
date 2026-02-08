use anyhow::{Context, Result};
use std::process::Command;
use std::thread;
use std::time::Duration;

/// Pane targeting information derived from the layout structure
struct PaneTarget {
    tab: &'static str,
    /// Navigation steps after switching to the tab to focus the correct pane.
    /// Empty means the only pane in the tab (no navigation needed).
    focus_steps: &'static [&'static str],
}

/// Map role name to its tab and pane position in the layout
fn pane_target(role: &str) -> Option<PaneTarget> {
    match role {
        "overlord" => Some(PaneTarget {
            tab: "command",
            focus_steps: &["left"],
        }),
        "strategist" => Some(PaneTarget {
            tab: "command",
            focus_steps: &["right"],
        }),
        "inferno" => Some(PaneTarget {
            tab: "battlefield",
            focus_steps: &[],
        }),
        "glacier" => Some(PaneTarget {
            tab: "support",
            focus_steps: &["up", "up"], // go to top
        }),
        "shadow" => Some(PaneTarget {
            tab: "support",
            focus_steps: &["up", "up", "down"], // top then one down
        }),
        "storm" => Some(PaneTarget {
            tab: "support",
            focus_steps: &["down", "down"], // go to bottom
        }),
        _ => None,
    }
}

/// Inject a notification message into a target pane via zellij write-chars.
///
/// This simulates keyboard input in the target pane, causing the Claude instance
/// to process it as a user message.
pub fn notify_pane(session: &str, target_role: &str, from_role: &str) -> Result<()> {
    let target = pane_target(target_role)
        .with_context(|| format!("Unknown target role: {}", target_role))?;

    // Switch to the target tab
    zellij_action(session, &["go-to-tab-name", target.tab])?;

    // Navigate to the correct pane within the tab
    for direction in target.focus_steps {
        zellij_action(session, &["move-focus", direction])?;
    }

    // Inject the trigger text
    let notification = format!(
        "[MESSAGE from {}] check_inbox ツールで受信メッセージを確認して作業を開始してください。",
        from_role
    );
    zellij_action(session, &["write-chars", &notification])?;

    // Small delay to ensure text is fully written before sending Enter
    thread::sleep(Duration::from_millis(200));

    // Send Enter key via write-chars with a literal carriage return
    zellij_action(session, &["write-chars", "\r"])?;

    Ok(())
}

fn zellij_action(session: &str, args: &[&str]) -> Result<()> {
    let mut cmd_args = vec!["--session", session, "action"];
    cmd_args.extend_from_slice(args);

    Command::new("zellij")
        .args(&cmd_args)
        .status()
        .with_context(|| format!("Failed to run zellij action: {:?}", args))?;

    Ok(())
}
