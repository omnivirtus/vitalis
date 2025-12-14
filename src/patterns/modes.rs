//! Modal state machine for vi-style interaction
//!
//! Handles switching between Normal, Insert, and Ex modes following vi conventions.

/// The current mode of the interface
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode {
    /// Normal mode - movement and commands (default)
    Normal,
    /// Insert mode - text entry (future)
    Insert,
    /// Ex command mode - colon commands with buffer
    Ex { command_buffer: String },
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Normal
    }
}

impl Mode {
    /// Get display string for mode
    pub fn display(&self) -> String {
        match self {
            Mode::Normal => "-- NORMAL --".to_string(),
            Mode::Insert => "-- INSERT --".to_string(),
            Mode::Ex { command_buffer } => format!(":{}", command_buffer),
        }
    }

    /// Check if in Ex mode
    pub fn is_ex(&self) -> bool {
        matches!(self, Mode::Ex { .. })
    }

    /// Get command buffer if in Ex mode
    pub fn command_buffer(&self) -> Option<&str> {
        match self {
            Mode::Ex { command_buffer } => Some(command_buffer),
            _ => None,
        }
    }
}
