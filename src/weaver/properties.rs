//! Thread Properties - The 10-stat foundation shared by all Threads
//!
//! Every Thread in the game (players, NPCs, regions, items, weather) possesses
//! the same 10 properties on a 0-20 scale.

/// The universal 10-stat foundation that all Threads share
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThreadProperties {
    // Physical
    pub strength: u8,
    pub dexterity: u8,
    pub constitution: u8,

    // Mental
    pub intelligence: u8,
    pub wisdom: u8,
    pub charisma: u8,

    // Social
    pub connections: u8,
    pub resources: u8,
    pub reputation: u8,

    // Mystical
    pub luck: u8,
}

impl ThreadProperties {
    /// Create a new ThreadProperties with all stats set to a base value
    pub fn new(base: u8) -> Self {
        Self {
            strength: base,
            dexterity: base,
            constitution: base,
            intelligence: base,
            wisdom: base,
            charisma: base,
            connections: base,
            resources: base,
            reputation: base,
            luck: base,
        }
    }

    /// Get a specific stat by name
    pub fn get_stat(&self, stat: Stat) -> u8 {
        match stat {
            Stat::Strength => self.strength,
            Stat::Dexterity => self.dexterity,
            Stat::Constitution => self.constitution,
            Stat::Intelligence => self.intelligence,
            Stat::Wisdom => self.wisdom,
            Stat::Charisma => self.charisma,
            Stat::Connections => self.connections,
            Stat::Resources => self.resources,
            Stat::Reputation => self.reputation,
            Stat::Luck => self.luck,
        }
    }
}

impl Default for ThreadProperties {
    fn default() -> Self {
        Self::new(10)
    }
}

/// The 10 stats that all Threads possess
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stat {
    // Physical
    Strength,
    Dexterity,
    Constitution,

    // Mental
    Intelligence,
    Wisdom,
    Charisma,

    // Social
    Connections,
    Resources,
    Reputation,

    // Mystical
    Luck,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thread_properties_default_sets_all_to_10() {
        let props = ThreadProperties::default();
        assert_eq!(props.strength, 10);
        assert_eq!(props.luck, 10);
    }

    #[test]
    fn get_stat_returns_correct_value() {
        let props = ThreadProperties {
            strength: 15,
            dexterity: 12,
            ..Default::default()
        };
        assert_eq!(props.get_stat(Stat::Strength), 15);
        assert_eq!(props.get_stat(Stat::Dexterity), 12);
    }
}
