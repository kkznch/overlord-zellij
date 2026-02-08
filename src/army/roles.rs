use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Overlord,
    Strategist,
    Inferno,
    Glacier,
    Shadow,
    Storm,
}

impl Role {
    pub fn display_name(&self) -> &'static str {
        match self {
            Role::Overlord => "魔王 (Overlord)",
            Role::Strategist => "闇の軍師 (Strategist)",
            Role::Inferno => "業火の将 (Inferno)",
            Role::Glacier => "氷結の将 (Glacier)",
            Role::Shadow => "常闇の将 (Shadow)",
            Role::Storm => "疾風の将 (Storm)",
        }
    }

    pub fn all() -> Vec<Role> {
        vec![
            Role::Overlord,
            Role::Strategist,
            Role::Inferno,
            Role::Glacier,
            Role::Shadow,
            Role::Storm,
        ]
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}
