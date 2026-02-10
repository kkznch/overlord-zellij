use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::OnceLock;

use chrono::Utc;

static LOG_FILE: OnceLock<PathBuf> = OnceLock::new();

pub fn init(name: &str) {
    let Ok(dir) = crate::config::config_dir().map(|d| d.join("logs")) else {
        return;
    };
    let _ = fs::create_dir_all(&dir);
    let path = dir.join(format!("{}.log", name));
    LOG_FILE.set(path).ok();
}

fn write_log(level: &str, msg: &str) {
    let Some(path) = LOG_FILE.get() else { return };
    let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) else {
        return;
    };
    let _ = writeln!(
        file,
        "[{}] [{}] {}",
        Utc::now().format("%Y-%m-%d %H:%M:%S%.3f"),
        level,
        msg
    );
}

pub fn debug(msg: &str) {
    write_log("DEBUG", msg);
}
pub fn info(msg: &str) {
    write_log("INFO", msg);
}
#[allow(dead_code)]
pub fn warn(msg: &str) {
    write_log("WARN", msg);
}
pub fn error(msg: &str) {
    write_log("ERROR", msg);
}
