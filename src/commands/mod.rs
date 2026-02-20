pub mod dashboard;
pub mod init;
pub mod unsummon;
pub mod status;
pub mod summon;

use crate::config::{unregister_session, relay_dir};
use crate::relay::store::MessageStore;
use crate::zellij::ZellijSession;
use crate::logging;

/// Clean up session: kill Zellij session, unregister, and delete relay data (best-effort)
pub fn cleanup_session_data(session_name: &str) {
    let session = ZellijSession::new(session_name);

    // Unconditionally kill session (best-effort)
    if let Err(e) = session.kill() {
        logging::debug(&format!("cleanup: kill failed for {}: {}", session_name, e));
    }
    if let Err(e) = session.delete(true) {
        logging::debug(&format!("cleanup: delete failed for {}: {}", session_name, e));
    }

    // Registry + relay cleanup
    if let Err(e) = unregister_session(session_name) {
        logging::debug(&format!("cleanup: unregister failed for {}: {}", session_name, e));
    }
    if let Ok(relay) = relay_dir(session_name) {
        let store = MessageStore::new(relay);
        if let Err(e) = store.cleanup() {
            logging::debug(&format!("cleanup: relay cleanup failed for {}: {}", session_name, e));
        }
    }
}
