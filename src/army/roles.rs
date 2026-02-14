use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Overlord,
    Strategist,
    Inferno,
    Glacier,
    Shadow,
    Storm,
}

pub const ALL: &[Role] = &[
    Role::Overlord,
    Role::Strategist,
    Role::Inferno,
    Role::Glacier,
    Role::Shadow,
    Role::Storm,
];

const SHITENNOH: &[Role] = &[Role::Inferno, Role::Glacier, Role::Shadow, Role::Storm];

impl Role {
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::Overlord => "overlord",
            Role::Strategist => "strategist",
            Role::Inferno => "inferno",
            Role::Glacier => "glacier",
            Role::Shadow => "shadow",
            Role::Storm => "storm",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Role::Overlord => "魔王 (Overlord)",
            Role::Strategist => "軍師 (Strategist)",
            Role::Inferno => "業火の将 (Inferno)",
            Role::Glacier => "氷結の将 (Glacier)",
            Role::Shadow => "常闇の将 (Shadow)",
            Role::Storm => "疾風の将 (Storm)",
        }
    }

    /// Emoji icon for the role.
    /// All emoji are from supplementary planes (U+1xxxx) = consistently 2-cell wide in terminals.
    pub fn icon(&self) -> &'static str {
        match self {
            Role::Overlord => "\u{1F451}",     // 👑
            Role::Strategist => "\u{1F9E0}",   // 🧠
            Role::Inferno => "\u{1F525}",      // 🔥
            Role::Glacier => "\u{1F9CA}",      // 🧊
            Role::Shadow => "\u{1F311}",       // 🌑
            Role::Storm => "\u{1F4A8}",        // 💨
        }
    }

    pub fn is_shitennoh(&self) -> bool {
        SHITENNOH.contains(self)
    }

    /// Check if this role is allowed to send a message to the target role.
    /// Only overlord <-> strategist <-> shitennoh routes are permitted.
    pub fn can_send_to(&self, target: Role) -> bool {
        match (*self, target) {
            (Role::Overlord, Role::Strategist) | (Role::Strategist, Role::Overlord) => true,
            (Role::Strategist, t) if t.is_shitennoh() => true,
            (f, Role::Strategist) if f.is_shitennoh() => true,
            _ => false,
        }
    }

    /// Return the list of roles this role is allowed to send messages to.
    pub fn allowed_targets(&self) -> Vec<Role> {
        match self {
            Role::Overlord => vec![Role::Strategist],
            Role::Strategist => vec![
                Role::Overlord,
                Role::Inferno,
                Role::Glacier,
                Role::Shadow,
                Role::Storm,
            ],
            r if r.is_shitennoh() => vec![Role::Strategist],
            _ => vec![],
        }
    }

    /// Map role to its terminal pane ID in the Zellij layout.
    pub fn pane_id(&self) -> u32 {
        match self {
            Role::Overlord => 0,
            Role::Strategist => 1,
            Role::Inferno => 2,
            Role::Glacier => 3,
            Role::Shadow => 4,
            Role::Storm => 5,
        }
    }

    pub fn all() -> Vec<Role> {
        ALL.to_vec()
    }
}

impl FromStr for Role {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "overlord" => Ok(Role::Overlord),
            "strategist" => Ok(Role::Strategist),
            "inferno" => Ok(Role::Inferno),
            "glacier" => Ok(Role::Glacier),
            "shadow" => Ok(Role::Shadow),
            "storm" => Ok(Role::Storm),
            _ => Err(format!("Invalid role '{}'. Valid: {:?}", s, ALL.iter().map(|r| r.as_str()).collect::<Vec<_>>())),
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_returns_six_roles() {
        assert_eq!(Role::all().len(), 6);
        assert_eq!(ALL.len(), 6);
    }

    #[test]
    fn test_display_name() {
        assert_eq!(Role::Overlord.display_name(), "魔王 (Overlord)");
        assert_eq!(Role::Strategist.display_name(), "軍師 (Strategist)");
        assert_eq!(Role::Inferno.display_name(), "業火の将 (Inferno)");
        assert_eq!(Role::Glacier.display_name(), "氷結の将 (Glacier)");
        assert_eq!(Role::Shadow.display_name(), "常闇の将 (Shadow)");
        assert_eq!(Role::Storm.display_name(), "疾風の将 (Storm)");
    }

    #[test]
    fn test_display_trait() {
        assert_eq!(format!("{}", Role::Overlord), "overlord");
        assert_eq!(format!("{}", Role::Strategist), "strategist");
    }

    #[test]
    fn test_as_str() {
        assert_eq!(Role::Overlord.as_str(), "overlord");
        assert_eq!(Role::Strategist.as_str(), "strategist");
        assert_eq!(Role::Inferno.as_str(), "inferno");
        assert_eq!(Role::Glacier.as_str(), "glacier");
        assert_eq!(Role::Shadow.as_str(), "shadow");
        assert_eq!(Role::Storm.as_str(), "storm");
    }

    #[test]
    fn test_from_str() {
        assert_eq!("overlord".parse::<Role>().unwrap(), Role::Overlord);
        assert_eq!("strategist".parse::<Role>().unwrap(), Role::Strategist);
        assert_eq!("inferno".parse::<Role>().unwrap(), Role::Inferno);
        assert_eq!("glacier".parse::<Role>().unwrap(), Role::Glacier);
        assert_eq!("shadow".parse::<Role>().unwrap(), Role::Shadow);
        assert_eq!("storm".parse::<Role>().unwrap(), Role::Storm);
        assert!("invalid".parse::<Role>().is_err());
    }

    #[test]
    fn test_serde_roundtrip() {
        for role in ALL {
            let json = serde_json::to_string(role).unwrap();
            let deserialized: Role = serde_json::from_str(&json).unwrap();
            assert_eq!(*role, deserialized);
        }
    }

    #[test]
    fn test_serde_lowercase() {
        let json = serde_json::to_string(&Role::Overlord).unwrap();
        assert_eq!(json, "\"overlord\"");
    }

    #[test]
    fn test_all_roles_are_unique() {
        let roles = Role::all();
        for (i, a) in roles.iter().enumerate() {
            for (j, b) in roles.iter().enumerate() {
                if i != j {
                    assert_ne!(a, b);
                }
            }
        }
    }

    #[test]
    fn test_is_shitennoh() {
        assert!(!Role::Overlord.is_shitennoh());
        assert!(!Role::Strategist.is_shitennoh());
        assert!(Role::Inferno.is_shitennoh());
        assert!(Role::Glacier.is_shitennoh());
        assert!(Role::Shadow.is_shitennoh());
        assert!(Role::Storm.is_shitennoh());
    }

    #[test]
    fn test_can_send_to_allowed() {
        assert!(Role::Overlord.can_send_to(Role::Strategist));
        assert!(Role::Strategist.can_send_to(Role::Overlord));
        for role in SHITENNOH {
            assert!(Role::Strategist.can_send_to(*role));
            assert!(role.can_send_to(Role::Strategist));
        }
    }

    #[test]
    fn test_can_send_to_forbidden() {
        for role in SHITENNOH {
            assert!(!Role::Overlord.can_send_to(*role));
            assert!(!role.can_send_to(Role::Overlord));
        }
        // shitennoh to shitennoh
        assert!(!Role::Inferno.can_send_to(Role::Shadow));
        assert!(!Role::Glacier.can_send_to(Role::Storm));
    }

    #[test]
    fn test_allowed_targets() {
        assert_eq!(Role::Overlord.allowed_targets(), vec![Role::Strategist]);
        assert_eq!(
            Role::Strategist.allowed_targets(),
            vec![Role::Overlord, Role::Inferno, Role::Glacier, Role::Shadow, Role::Storm]
        );
        for role in SHITENNOH {
            assert_eq!(role.allowed_targets(), vec![Role::Strategist]);
        }
    }

    #[test]
    fn test_pane_id() {
        assert_eq!(Role::Overlord.pane_id(), 0);
        assert_eq!(Role::Strategist.pane_id(), 1);
        assert_eq!(Role::Inferno.pane_id(), 2);
        assert_eq!(Role::Glacier.pane_id(), 3);
        assert_eq!(Role::Shadow.pane_id(), 4);
        assert_eq!(Role::Storm.pane_id(), 5);
    }
}
