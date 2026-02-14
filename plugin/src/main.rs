use serde::Deserialize;
use std::collections::BTreeMap;
use zellij_tile::prelude::*;

const CARRIAGE_RETURN: u8 = 13;
const ENTER_DELAY_SECS: f64 = 0.2;

#[derive(Deserialize)]
struct SendKeysPayload {
    pane_id: u32,
    text: String,
    #[serde(default)]
    send_enter: bool,
}

#[derive(Default)]
struct OvldNotifier {
    pending_enter: Option<PaneId>,
}

register_plugin!(OvldNotifier);

impl ZellijPlugin for OvldNotifier {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::WriteToStdin,
            PermissionType::ReadCliPipes,
        ]);
        subscribe(&[EventType::PermissionRequestResult, EventType::Timer]);
    }

    fn update(&mut self, event: Event) -> bool {
        let Event::Timer(_) = event else {
            return false;
        };
        let Some(pane_id) = self.pending_enter.take() else {
            return false;
        };
        eprintln!("[ovld-notify] sending delayed Enter (CR) to {:?}", pane_id);
        write_to_pane_id(vec![CARRIAGE_RETURN], pane_id);
        false
    }

    fn pipe(&mut self, pipe_message: PipeMessage) -> bool {
        eprintln!("[ovld-notify] pipe received: name={}", pipe_message.name);
        if pipe_message.name != "send_keys" {
            return false;
        }

        let Some(payload) = pipe_message.payload else {
            eprintln!("[ovld-notify] no payload in pipe message");
            return false;
        };

        let msg: SendKeysPayload = match serde_json::from_str(&payload) {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("[ovld-notify] failed to parse payload: {}", e);
                return false;
            }
        };

        eprintln!(
            "[ovld-notify] write_to_pane_id: pane={} len={} send_enter={}",
            msg.pane_id,
            msg.text.len(),
            msg.send_enter
        );
        let pane_id = PaneId::Terminal(msg.pane_id);
        write_to_pane_id(msg.text.into_bytes(), pane_id);

        // Delay Enter so it arrives as a separate read() call.
        // Without delay, text+CR are bundled in one stdin chunk
        // and Claude Code treats CR as part of the text, not as submit.
        if msg.send_enter {
            self.pending_enter = Some(pane_id);
            set_timeout(ENTER_DELAY_SECS);
        }

        false
    }
}
