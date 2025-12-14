//! Vi-style command parsing and execution
//!
//! This module handles parsing player input into game actions following
//! the universal action grammar: [count][which][type][target]

use crate::foundation::Position;

/// Directions for movement and targeting (vi-style hjkl)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,   // h
    Down,   // j
    Up,     // k
    Right,  // l
}

impl Direction {
    /// Convert direction to position delta
    pub fn to_delta(self) -> (i32, i32) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Down => (0, 1),
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
        }
    }

    /// Apply direction to a position, returning new position
    pub fn apply_to(self, pos: Position) -> Position {
        let (dx, dy) = self.to_delta();
        Position::new(pos.x + dx, pos.y + dy)
    }
}

/// Player commands parsed from input
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    /// Move in a direction
    Move(Direction),
    /// Quit the game
    Quit,
    /// Unknown/invalid command
    Unknown,
}

/// Parse a character into a command
pub fn parse_command(c: char) -> Command {
    match c {
        'h' => Command::Move(Direction::Left),
        'j' => Command::Move(Direction::Down),
        'k' => Command::Move(Direction::Up),
        'l' => Command::Move(Direction::Right),
        'q' => Command::Quit,
        _ => Command::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_movement_commands() {
        assert_eq!(parse_command('h'), Command::Move(Direction::Left));
        assert_eq!(parse_command('j'), Command::Move(Direction::Down));
        assert_eq!(parse_command('k'), Command::Move(Direction::Up));
        assert_eq!(parse_command('l'), Command::Move(Direction::Right));
    }

    #[test]
    fn direction_to_delta() {
        assert_eq!(Direction::Left.to_delta(), (-1, 0));
        assert_eq!(Direction::Right.to_delta(), (1, 0));
    }

    #[test]
    fn direction_apply_to_position() {
        let pos = Position::new(5, 5);
        assert_eq!(Direction::Left.apply_to(pos), Position::new(4, 5));
        assert_eq!(Direction::Up.apply_to(pos), Position::new(5, 4));
    }
}
