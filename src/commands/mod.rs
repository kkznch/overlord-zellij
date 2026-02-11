pub mod dashboard;
pub mod init;
pub mod unsummon;
pub mod status;
pub mod summon;

use crate::config::{delete_session_metadata, relay_dir};
use crate::relay::store::MessageStore;

/// Clean up session metadata and relay data (best-effort)
pub fn cleanup_session_data() {
    let _ = delete_session_metadata();
    if let Ok(relay) = relay_dir() {
        let store = MessageStore::new(relay);
        let _ = store.cleanup();
    }
}
