//! Modal state machine for vi-style interaction
//!
//! Handles switching between Normal, Insert, and Ex modes following vi conventions.

/// The current mode of the interface
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode {
    /// Normal mode - movement and commands (default)
    Normal { count_buffer: String },
    /// Insert mode - text entry (future)
    Insert,
    /// Ex command mode - colon commands with buffer
    Ex { command_buffer: String },
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Normal {
            count_buffer: String::new(),
        }
    }
}

impl Mode {
    /// Get display string for mode indicator
    pub fn mode_name(&self) -> &str {
        match self {
            Mode::Normal { .. } => "-- NORMAL --",
            Mode::Insert => "-- INSERT --",
            Mode::Ex { .. } => "-- COMMAND --",
        }
    }

    /// Get pending keys to display (count buffer in Normal mode, empty otherwise)
    pub fn pending_keys(&self) -> &str {
        match self {
            Mode::Normal { count_buffer } => count_buffer,
            _ => "",
        }
    }

    /// Get command line text (for Ex mode, shown below mode indicator without border)
    pub fn command_line(&self) -> Option<String> {
        match self {
            Mode::Ex { command_buffer } => Some(format!(":{}", command_buffer)),
            _ => None,
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

    /// Get count buffer if in Normal mode
    pub fn count_buffer(&self) -> Option<&str> {
        match self {
            Mode::Normal { count_buffer } => Some(count_buffer),
            _ => None,
        }
    }
}
