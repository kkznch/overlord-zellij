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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_returns_six_roles() {
        assert_eq!(Role::all().len(), 6);
    }

    #[test]
    fn test_display_name() {
        assert_eq!(Role::Overlord.display_name(), "魔王 (Overlord)");
        assert_eq!(Role::Strategist.display_name(), "闇の軍師 (Strategist)");
        assert_eq!(Role::Inferno.display_name(), "業火の将 (Inferno)");
        assert_eq!(Role::Glacier.display_name(), "氷結の将 (Glacier)");
        assert_eq!(Role::Shadow.display_name(), "常闇の将 (Shadow)");
        assert_eq!(Role::Storm.display_name(), "疾風の将 (Storm)");
    }

    #[test]
    fn test_display_trait() {
        let role = Role::Overlord;
        assert_eq!(format!("{}", role), "魔王 (Overlord)");
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
}
