//! Patterns - Player interface revealing the weaving
//!
//! This bounded context handles vi-style input processing and terminal rendering.
//! All external dependencies (ratatui, crossterm) are isolated here.

/// Vi-style command parsing and execution
pub mod commands;

/// Terminal rendering using ratatui (infrastructure concern)
pub mod display;

/// Modal state machine (Normal, Insert, Ex modes)
pub mod modes;
