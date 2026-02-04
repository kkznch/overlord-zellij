use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Overlord,
    Strategist,
    LegionImpl,
    LegionDebug,
    LegionDocs,
}

impl Role {
    pub fn pane_name(&self) -> &'static str {
        match self {
            Role::Overlord => "overlord",
            Role::Strategist => "strategist",
            Role::LegionImpl => "legion-impl",
            Role::LegionDebug => "legion-debug",
            Role::LegionDocs => "legion-docs",
        }
    }

    pub fn ritual_file(&self) -> &'static str {
        match self {
            Role::Overlord => "overlord.md",
            Role::Strategist => "strategist.md",
            Role::LegionImpl => "legion_impl.md",
            Role::LegionDebug => "legion_debug.md",
            Role::LegionDocs => "legion_docs.md",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Role::Overlord => "魔王 (Overlord)",
            Role::Strategist => "闇の軍師 (Dark Strategist)",
            Role::LegionImpl => "第一兵団・剛腕 (Legion: Brute Force)",
            Role::LegionDebug => "第二兵団・処刑 (Legion: Executioner)",
            Role::LegionDocs => "第三兵団・記録 (Legion: Scribe)",
        }
    }

    pub fn all() -> Vec<Role> {
        vec![
            Role::Overlord,
            Role::Strategist,
            Role::LegionImpl,
            Role::LegionDebug,
            Role::LegionDocs,
        ]
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}
