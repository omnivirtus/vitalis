//! The Great Loom - Application entry point and game loop coordination
//!
//! This is the bootstrap system that wires together all bounded contexts
//! and runs the main game loop.

use crossterm::event::{self, Event, KeyCode};
use std::io;
use vitalis::{
    foundation::Position,
    patterns::{
        commands::{parse_command, Command},
        display::{init_terminal, render, restore_terminal},
        modes::Mode,
    },
    tapestry::Tapestry,
    threads::{Thread, ThreadId, ThreadKind},
    weaver::{properties::ThreadProperties, states::ThreadStates},
};

fn main() -> io::Result<()> {
    // Initialize the terminal
    let mut terminal = init_terminal()?;

    // Create the initial Tapestry
    let mut tapestry = Tapestry::new();

    // Create the player Thread
    let player_id = tapestry.next_id();
    let player = Thread {
        id: player_id,
        kind: ThreadKind::Player {
            name: "Wanderer".to_string(),
        },
        properties: ThreadProperties::default(),
        states: ThreadStates::default(),
        position: Some(Position::new(0, 0)),
    };
    tapestry.add_thread(player);

    // Create a simple region Thread
    for x in -5..=5 {
        for y in -5..=5 {
            if x == 0 && y == 0 {
                continue; // Player position
            }
            let region_id = tapestry.next_id();
            let region = Thread {
                id: region_id,
                kind: ThreadKind::Region {
                    description: "Whispering Plains".to_string(),
                },
                properties: ThreadProperties {
                    intelligence: 8,
                    charisma: 15,
                    ..Default::default()
                },
                states: ThreadStates::default(),
                position: Some(Position::new(x, y)),
            };
            tapestry.add_thread(region);
        }
    }

    // Game loop
    let mut mode = Mode::default();
    let result = run_game_loop(&mut terminal, &mut tapestry, player_id, &mut mode);

    // Restore terminal
    restore_terminal(&mut terminal)?;

    result
}

fn run_game_loop(
    terminal: &mut vitalis::patterns::display::TerminalType,
    tapestry: &mut Tapestry,
    player_id: ThreadId,
    _mode: &mut Mode,
) -> io::Result<()> {
    loop {
        // Render current state
        render(terminal, tapestry, player_id)?;

        // Handle input
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => {
                        let command = parse_command(c);
                        if !execute_command(command, tapestry, player_id) {
                            break; // Quit command received
                        }
                    }
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

fn execute_command(command: Command, tapestry: &mut Tapestry, player_id: ThreadId) -> bool {
    match command {
        Command::Move(direction) => {
            if let Some(player) = tapestry.get_thread_mut(player_id) {
                if let Some(current_pos) = player.position {
                    let new_pos = direction.apply_to(current_pos);
                    player.position = Some(new_pos);
                }
            }
            true // Continue game
        }
        Command::Quit => false, // Exit game
        Command::Unknown => true, // Ignore unknown commands
    }
}
