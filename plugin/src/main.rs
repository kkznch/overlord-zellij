use serde::Deserialize;
use std::collections::BTreeMap;
use zellij_tile::prelude::*;

#[derive(Deserialize)]
struct SendKeysPayload {
    pane_id: u32,
    text: String,
    #[serde(default)]
    send_enter: bool,
}

#[derive(Default)]
struct OvldNotifier;

register_plugin!(OvldNotifier);

impl ZellijPlugin for OvldNotifier {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::WriteToStdin,
            PermissionType::ReadCliPipes,
        ]);
        subscribe(&[EventType::PermissionRequestResult]);
    }

    fn pipe(&mut self, pipe_message: PipeMessage) -> bool {
        if pipe_message.name == "send_keys" {
            if let Some(payload) = pipe_message.payload {
                if let Ok(msg) = serde_json::from_str::<SendKeysPayload>(&payload) {
                    let pane_id = PaneId::Terminal(msg.pane_id);
                    write_to_pane_id(msg.text.into_bytes(), pane_id);
                    if msg.send_enter {
                        write_to_pane_id(vec![13], pane_id); // carriage return
                    }
                }
            }
        }
        false
    }

    fn render(&mut self, _rows: usize, _cols: usize) {}
}
