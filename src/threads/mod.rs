//! Threads - Game elements that get woven together
//!
//! This bounded context defines the various Thread types (player, NPCs, regions, items)
//! that interact through The Weaver's mathematical laws.

use crate::foundation::Position;
use crate::weaver::properties::ThreadProperties;
use crate::weaver::states::ThreadStates;

/// A Thread is any entity in the game world that can interact with other Threads
#[derive(Debug, Clone)]
pub struct Thread {
    pub id: ThreadId,
    pub kind: ThreadKind,
    pub properties: ThreadProperties,
    pub states: ThreadStates,
    pub position: Option<Position>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThreadId(pub u64);

#[derive(Debug, Clone, PartialEq)]
pub enum ThreadKind {
    Player { name: String },
    Region { description: String },
    Npc { name: String },
}
