//! The Great Loom - Application entry point and game loop coordination
//!
//! This is the bootstrap system that wires together all bounded contexts
//! and runs the main game loop.

use crossterm::event::{self, Event, KeyCode};
use std::io;
use vitalis::{
    foundation::Position,
    patterns::{
        commands::{parse_ex_command, parse_ex_input, parse_normal_command, Command, ExCommand},
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
    mode: &mut Mode,
) -> io::Result<()> {
    loop {
        // Render current state
        render(terminal, tapestry, player_id, mode)?;

        // Handle input
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                let command = match key.code {
                    KeyCode::Char(c) => {
                        if mode.is_ex() {
                            // Check if Enter key sent as '\n' or '\r'
                            if c == '\n' || c == '\r' {
                                // Execute Ex command
                                if let Some(cmd_str) = mode.command_buffer() {
                                    if let Some(ex_cmd) = parse_ex_command(cmd_str) {
                                        Command::ExCommand(ex_cmd)
                                    } else {
                                        Command::Unknown
                                    }
                                } else {
                                    Command::Unknown
                                }
                            } else {
                                parse_ex_input(c)
                            }
                        } else {
                            parse_normal_command(c)
                        }
                    }
                    KeyCode::Enter if mode.is_ex() => {
                        // Execute Ex command
                        if let Some(cmd_str) = mode.command_buffer() {
                            if let Some(ex_cmd) = parse_ex_command(cmd_str) {
                                Command::ExCommand(ex_cmd)
                            } else {
                                Command::Unknown
                            }
                        } else {
                            Command::Unknown
                        }
                    }
                    KeyCode::Esc => {
                        if mode.is_ex() {
                            Command::CancelEx
                        } else {
                            continue; // In normal mode, ESC does nothing
                        }
                    }
                    _ => Command::Unknown,
                };

                if !execute_command(command, tapestry, player_id, mode) {
                    break; // Quit command received
                }
            }
        }
    }

    Ok(())
}

fn execute_command(
    command: Command,
    tapestry: &mut Tapestry,
    player_id: ThreadId,
    mode: &mut Mode,
) -> bool {
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
        Command::EnterExMode => {
            *mode = Mode::Ex {
                command_buffer: String::new(),
            };
            true
        }
        Command::ExInput(c) => {
            if let Mode::Ex { command_buffer } = mode {
                command_buffer.push(c);
            }
            true
        }
        Command::CancelEx => {
            *mode = Mode::Normal;
            true
        }
        Command::ExCommand(ex_cmd) => {
            *mode = Mode::Normal; // Return to normal mode after command
            match ex_cmd {
                ExCommand::Quit => false, // Exit game
            }
        }
        Command::Unknown => true, // Ignore unknown commands
    }
}
