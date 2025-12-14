//! Tapestry - Living story narratives that grow and evolve
//!
//! This bounded context coordinates the game world state and manages
//! the collection of Threads that comprise the current weaving.

use crate::foundation::Position;
use crate::threads::{Thread, ThreadId};
use std::collections::HashMap;

/// The Tapestry holds all Threads and coordinates their interactions
pub struct Tapestry {
    threads: HashMap<ThreadId, Thread>,
    next_id: u64,
}

impl Tapestry {
    pub fn new() -> Self {
        Self {
            threads: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn add_thread(&mut self, thread: Thread) -> ThreadId {
        let id = thread.id;
        self.threads.insert(id, thread);
        id
    }

    pub fn next_id(&mut self) -> ThreadId {
        let id = ThreadId(self.next_id);
        self.next_id += 1;
        id
    }

    pub fn get_thread(&self, id: ThreadId) -> Option<&Thread> {
        self.threads.get(&id)
    }

    pub fn get_thread_mut(&mut self, id: ThreadId) -> Option<&mut Thread> {
        self.threads.get_mut(&id)
    }

    pub fn get_thread_at(&self, position: Position) -> Option<&Thread> {
        self.threads.values().find(|t| t.position == Some(position))
    }
}

impl Default for Tapestry {
    fn default() -> Self {
        Self::new()
    }
}
