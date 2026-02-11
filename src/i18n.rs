use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Lang {
    En,
    Ja,
}

impl Default for Lang {
    fn default() -> Self {
        Lang::En
    }
}

/// All translatable messages in the CLI.
/// Each variant returns a &'static str for the given language.
pub fn t(key: &str, lang: Lang) -> &'static str {
    match (key, lang) {
        // === summon ===
        ("summon.already_exists_with_cwd", Lang::En) => "Already summoned at {cwd}.\nRun `ovld unsummon` first.",
        ("summon.already_exists_with_cwd", Lang::Ja) => "既に {cwd} で召喚されています。\n`ovld unsummon` で還送してから再召喚してください。",
        ("summon.already_exists", Lang::En) => "Session '{name}' already exists.\nRun `ovld unsummon` first.",
        ("summon.already_exists", Lang::Ja) => "既存セッション '{name}' があります。\n`ovld unsummon` で還送してから再召喚してください。",
        ("summon.starting", Lang::En) => "Summoning the army at {cwd}...",
        ("summon.starting", Lang::Ja) => "{cwd} で魔王軍を召喚中...",
        ("summon.ritual_files", Lang::En) => "Ritual files: {path}",
        ("summon.ritual_files", Lang::Ja) => "儀式ファイル: {path}",
        ("summon.session_ended", Lang::En) => "Session '{name}' has ended.",
        ("summon.session_ended", Lang::Ja) => "セッション '{name}' が終了しました。",

        // === unsummon ===
        ("unsummon.not_found", Lang::En) => "Session '{name}' not found. Nothing to unsummon.",
        ("unsummon.not_found", Lang::Ja) => "セッション '{name}' が見つかりません。還送対象なし。",
        ("unsummon.confirm", Lang::En) => "Unsummon session '{name}'? [y/N] ",
        ("unsummon.confirm", Lang::Ja) => "セッション '{name}' を還送しますか？ [y/N] ",
        ("unsummon.cancelled", Lang::En) => "Cancelled. The army stands strong.",
        ("unsummon.cancelled", Lang::Ja) => "中止しました。魔王軍は健在です。",
        ("unsummon.in_progress", Lang::En) => "Unsummoning session '{name}'...",
        ("unsummon.in_progress", Lang::Ja) => "セッション '{name}' を還送中...",
        ("unsummon.success", Lang::En) => "Army unsummoned. Session '{name}' terminated.",
        ("unsummon.success", Lang::Ja) => "魔王軍を還送しました。セッション '{name}' を終了しました。",

        // === status ===
        ("status.header", Lang::En) => "=== Army Status ===",
        ("status.header", Lang::Ja) => "=== 魔王軍ステータス ===",
        ("status.session", Lang::En) => "Session:",
        ("status.session", Lang::Ja) => "セッション:",
        ("status.state", Lang::En) => "State:",
        ("status.state", Lang::Ja) => "状態:",
        ("status.active", Lang::En) => "ACTIVE",
        ("status.active", Lang::Ja) => "展開中",
        ("status.not_summoned", Lang::En) => "NOT SUMMONED",
        ("status.not_summoned", Lang::Ja) => "未召喚",
        ("status.cwd", Lang::En) => "Working dir:",
        ("status.cwd", Lang::Ja) => "作業場所:",
        ("status.started_at", Lang::En) => "Summoned at:",
        ("status.started_at", Lang::Ja) => "召喚時刻:",
        ("status.hint_summon", Lang::En) => "Run 'ovld summon' to summon the army.",
        ("status.hint_summon", Lang::Ja) => "'ovld summon' で魔王軍を召喚してください。",
        ("status.ranks_header", Lang::En) => "=== Army Ranks ===",
        ("status.ranks_header", Lang::Ja) => "=== 魔王軍階級 ===",
        ("status.hint_unsummon", Lang::En) => "Run 'ovld unsummon' to unsummon the session.",
        ("status.hint_unsummon", Lang::Ja) => "'ovld unsummon' でセッションを還送できます。",

        // === init ===
        ("init.already_exists", Lang::En) => "Global config already deployed: {path}",
        ("init.already_exists", Lang::Ja) => "グローバル設定は既に展開済みです: {path}",
        ("init.hint_force", Lang::En) => "Use `ovld init --force` to overwrite.",
        ("init.hint_force", Lang::Ja) => "上書きするには `ovld init --force` を使用してください。",
        ("init.success", Lang::En) => "Global config deployed: {path}",
        ("init.success", Lang::Ja) => "グローバル設定を展開しました: {path}",

        _ => "[unknown message]",
    }
}

/// Replace `{name}` placeholders in a translated string.
pub fn fmt(template: &str, args: &[(&str, &str)]) -> String {
    let mut result = template.to_string();
    for (key, value) in args {
        result = result.replace(&format!("{{{}}}", key), value);
    }
    result
}

/// Convenience: translate + format in one call.
pub fn tf(key: &str, lang: Lang, args: &[(&str, &str)]) -> String {
    fmt(t(key, lang), args)
}

/// Format a path for display.
pub fn path_str(path: &Path) -> String {
    format!("{:?}", path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_t_known_key_en() {
        assert_eq!(t("summon.starting", Lang::En), "Summoning the army at {cwd}...");
        assert_eq!(t("status.active", Lang::En), "ACTIVE");
        assert_eq!(t("init.success", Lang::En), "Global config deployed: {path}");
    }

    #[test]
    fn test_t_known_key_ja() {
        assert_eq!(t("summon.starting", Lang::Ja), "{cwd} で魔王軍を召喚中...");
        assert_eq!(t("status.active", Lang::Ja), "展開中");
        assert_eq!(t("init.success", Lang::Ja), "グローバル設定を展開しました: {path}");
    }

    #[test]
    fn test_t_unknown_key() {
        assert_eq!(t("nonexistent.key", Lang::En), "[unknown message]");
        assert_eq!(t("nonexistent.key", Lang::Ja), "[unknown message]");
        assert_eq!(t("", Lang::En), "[unknown message]");
    }

    #[test]
    fn test_fmt_placeholder_substitution() {
        let result = fmt("Hello {name}, welcome to {place}!", &[("name", "Alice"), ("place", "Wonderland")]);
        assert_eq!(result, "Hello Alice, welcome to Wonderland!");
    }

    #[test]
    fn test_fmt_no_args() {
        let result = fmt("No placeholders here.", &[]);
        assert_eq!(result, "No placeholders here.");
    }

    #[test]
    fn test_tf_translate_and_format() {
        let result = tf("summon.starting", Lang::En, &[("cwd", "/tmp/project")]);
        assert_eq!(result, "Summoning the army at /tmp/project...");

        let result_ja = tf("summon.starting", Lang::Ja, &[("cwd", "/tmp/project")]);
        assert_eq!(result_ja, "/tmp/project で魔王軍を召喚中...");
    }

    #[test]
    fn test_path_str() {
        let path = PathBuf::from("/home/user/project");
        let result = path_str(&path);
        assert!(result.contains("/home/user/project"));
    }

    #[test]
    fn test_lang_default_is_en() {
        assert_eq!(Lang::default(), Lang::En);
    }
}
