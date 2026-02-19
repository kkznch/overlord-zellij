pub mod dashboard;
pub mod init;
pub mod unsummon;
pub mod status;
pub mod summon;

use crate::config::{unregister_session, relay_dir};
use crate::relay::store::MessageStore;

/// Clean up session registry entry and relay data (best-effort)
pub fn cleanup_session_data(session_name: &str) {
    let _ = unregister_session(session_name);
    if let Ok(relay) = relay_dir(session_name) {
        let store = MessageStore::new(relay);
        let _ = store.cleanup();
    }
}
