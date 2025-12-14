//! Vitalis - Death-Driven Multi-Generational CLI Roguelike
//!
//! # The Tapestry Framework
//!
//! This library implements The Tapestry, a living mathematical weaving system
//! that enables infinite emergent narrative through Thread interactions.

// Bounded Contexts following The Great Loom Architecture
pub mod weaver;     // Mathematical laws governing Thread interactions
pub mod threads;    // Game elements that get woven together
pub mod tapestry;   // Living story narratives
pub mod patterns;   // Player interface revealing the weaving
pub mod seamstress; // Persistence across generations

/// Foundation Types - Shared domain concepts used across bounded contexts
pub mod foundation {
    /// Position in the game world (used by Threads and Patterns)
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Position {
        pub x: i32,
        pub y: i32,
    }

    impl Position {
        pub fn new(x: i32, y: i32) -> Self {
            Self { x, y }
        }
    }
}
