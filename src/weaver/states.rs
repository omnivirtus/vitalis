//! Thread States - Temporal modifications to Thread behavior
//!
//! The 10 states that can modify any Thread's weaving capabilities over time.
//! States flow between Thread types creating temporal continuity.

/// Thread States modify how Threads interact through The Weaver
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThreadStates {
    // Negative states (0.0 = none, 1.0 = maximum)
    pub damaged: f32,
    pub corrupted: f32,
    pub stressed: f32,
    pub neglected: f32,

    // Positive states (0.0 = none, 1.0 = maximum)
    pub enhanced: f32,
    pub experienced: f32,
    pub connected: f32,
    pub prestigious: f32,
    pub blessed: f32,
    pub adapted: f32,
}

impl ThreadStates {
    /// Create new ThreadStates with all states at zero
    pub fn new() -> Self {
        Self {
            damaged: 0.0,
            corrupted: 0.0,
            stressed: 0.0,
            neglected: 0.0,
            enhanced: 0.0,
            experienced: 0.0,
            connected: 0.0,
            prestigious: 0.0,
            blessed: 0.0,
            adapted: 0.0,
        }
    }

    /// Clamp all state values to [0.0, 1.0] range
    pub fn clamp(&mut self) {
        self.damaged = self.damaged.clamp(0.0, 1.0);
        self.corrupted = self.corrupted.clamp(0.0, 1.0);
        self.stressed = self.stressed.clamp(0.0, 1.0);
        self.neglected = self.neglected.clamp(0.0, 1.0);
        self.enhanced = self.enhanced.clamp(0.0, 1.0);
        self.experienced = self.experienced.clamp(0.0, 1.0);
        self.connected = self.connected.clamp(0.0, 1.0);
        self.prestigious = self.prestigious.clamp(0.0, 1.0);
        self.blessed = self.blessed.clamp(0.0, 1.0);
        self.adapted = self.adapted.clamp(0.0, 1.0);
    }
}

impl Default for ThreadStates {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thread_states_default_all_zero() {
        let states = ThreadStates::default();
        assert_eq!(states.damaged, 0.0);
        assert_eq!(states.blessed, 0.0);
    }

    #[test]
    fn clamp_enforces_valid_range() {
        let mut states = ThreadStates {
            damaged: 1.5,
            enhanced: -0.5,
            ..Default::default()
        };
        states.clamp();
        assert_eq!(states.damaged, 1.0);
        assert_eq!(states.enhanced, 0.0);
    }
}
