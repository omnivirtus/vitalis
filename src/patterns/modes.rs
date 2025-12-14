//! Modal state machine for vi-style interaction
//!
//! Handles switching between Normal, Insert, and Ex modes following vi conventions.

/// The current mode of the interface
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Normal mode - movement and commands (default)
    Normal,
    /// Insert mode - text entry (future)
    Insert,
    /// Ex command mode - colon commands (future)
    Ex,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Normal
    }
}

impl Mode {
    /// Get display string for mode
    pub fn display(&self) -> &str {
        match self {
            Mode::Normal => "-- NORMAL --",
            Mode::Insert => "-- INSERT --",
            Mode::Ex => "-- EX --",
        }
    }
}
