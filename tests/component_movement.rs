//! Component Test: Player Movement
//!
//! Black-box test of the entire application verifying that the player
//! can move around using vi-style hjkl commands.
//!
//! Uses screenshot-based testing: captures the rendered terminal state
//! and validates what users actually see.

use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use std::io::{Read, Write};
use std::time::Duration;

const SCREEN_ROWS: usize = 24;
const SCREEN_COLS: usize = 80;
const INITIAL_RENDER_DELAY: Duration = Duration::from_millis(1000);
const INPUT_PROCESSING_DELAY: Duration = Duration::from_millis(300);
const SCREEN_READ_TIMEOUT: Duration = Duration::from_millis(200);

#[test]
fn player_can_move_with_hjkl_commands() {
    let mut game = VitalisGame::start();

    // Player starts at world origin
    assert_eq!(game.world_position(), (0, 0), "Player should start at origin");

    // Move right with 'l'
    game.press('l');
    assert_eq!(
        game.world_position(),
        (1, 0),
        "Player should move right to (1, 0)"
    );

    // Move down with 'j'
    game.press('j');
    assert_eq!(
        game.world_position(),
        (1, 1),
        "Player should move down to (1, 1)"
    );
}

/// Test harness for running and interacting with the Vitalis game
struct VitalisGame {
    screen: ScreenBuffer,
    writer: Box<dyn Write + Send>,
    reader: Box<dyn Read + Send>,
    _child: Box<dyn portable_pty::Child + Send + Sync>,
}

impl VitalisGame {
    fn start() -> Self {
        let pty_system = NativePtySystem::default();
        let pair = pty_system
            .openpty(PtySize {
                rows: SCREEN_ROWS as u16,
                cols: SCREEN_COLS as u16,
                pixel_width: 0,
                pixel_height: 0,
            })
            .expect("Failed to create PTY");

        let mut cmd = CommandBuilder::new("cargo");
        cmd.arg("run");
        cmd.arg("--quiet");
        cmd.cwd(std::env::current_dir().expect("Failed to get current directory"));

        let child = pair.slave.spawn_command(cmd).expect("Failed to spawn game");
        let mut reader = pair.master.try_clone_reader().expect("Failed to clone reader");
        let writer = pair.master.take_writer().expect("Failed to get writer");

        let mut screen = ScreenBuffer::new();

        // Wait for initial render
        std::thread::sleep(INITIAL_RENDER_DELAY);
        screen.update(&mut reader);

        Self {
            screen,
            writer,
            reader,
            _child: child,
        }
    }

    fn press(&mut self, key: char) {
        self.writer
            .write_all(key.to_string().as_bytes())
            .expect("Failed to send key");
        self.writer.flush().expect("Failed to flush");

        std::thread::sleep(INPUT_PROCESSING_DELAY);
        self.screen.update(&mut self.reader);
    }

    fn world_position(&self) -> (i32, i32) {
        extract_world_position(&self.screen.get_screen())
            .expect("Could not find player position in status bar")
    }
}

impl Drop for VitalisGame {
    fn drop(&mut self) {
        // Attempt graceful quit
        let _ = self.writer.write_all(b"q");
        let _ = self.writer.flush();
        std::thread::sleep(Duration::from_millis(100));
    }
}

/// Persistent screen buffer that accumulates terminal updates
struct ScreenBuffer {
    screen: Vec<Vec<char>>,
}

impl ScreenBuffer {
    fn new() -> Self {
        Self {
            screen: vec![vec![' '; SCREEN_COLS]; SCREEN_ROWS],
        }
    }

    fn update(&mut self, reader: &mut (dyn Read + Send)) {
        let output = read_available_output(reader, SCREEN_READ_TIMEOUT);
        self.apply_ansi_updates(&output);
    }

    fn get_screen(&self) -> Vec<Vec<char>> {
        self.screen.clone()
    }

    fn apply_ansi_updates(&mut self, output: &str) {
        let mut cursor_row = 0;
        let mut cursor_col = 0;
        let mut chars = output.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\x1b' {
                handle_escape_sequence(&mut chars, &mut cursor_row, &mut cursor_col);
            } else if !ch.is_control() || ch == ' ' {
                if cursor_row < SCREEN_ROWS && cursor_col < SCREEN_COLS {
                    self.screen[cursor_row][cursor_col] = ch;
                    cursor_col += 1;
                }
            }
        }
    }
}

fn read_available_output(reader: &mut (dyn Read + Send), timeout: Duration) -> String {
    let mut output = String::new();
    let mut buffer = [0u8; 8192];
    let start = std::time::Instant::now();

    while start.elapsed() < timeout {
        match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                if let Ok(s) = std::str::from_utf8(&buffer[..n]) {
                    output.push_str(s);
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(10));
            }
            Err(_) => break,
        }
    }

    output
}

fn handle_escape_sequence(
    chars: &mut std::iter::Peekable<std::str::Chars>,
    cursor_row: &mut usize,
    cursor_col: &mut usize,
) {
    if chars.peek() == Some(&'[') {
        chars.next();
        let mut seq = String::new();

        while let Some(&ch) = chars.peek() {
            if ch.is_ascii_alphabetic() {
                let cmd = chars.next().unwrap();
                handle_csi_command(&seq, cmd, cursor_row, cursor_col);
                break;
            } else {
                seq.push(chars.next().unwrap());
            }
        }
    } else if chars.peek() == Some(&']') {
        skip_osc_sequence(chars);
    } else if chars.peek() == Some(&'?') {
        skip_dec_private_mode(chars);
    }
}

fn handle_csi_command(seq: &str, cmd: char, row: &mut usize, col: &mut usize) {
    match cmd {
        'H' | 'f' => {
            // Cursor position
            let parts: Vec<&str> = seq.split(';').collect();
            if parts.len() >= 2 {
                *row = parts[0].parse::<usize>().unwrap_or(1).saturating_sub(1);
                *col = parts[1].parse::<usize>().unwrap_or(1).saturating_sub(1);
            } else if parts.len() == 1 && !parts[0].is_empty() {
                *row = parts[0].parse::<usize>().unwrap_or(1).saturating_sub(1);
                *col = 0;
            } else {
                *row = 0;
                *col = 0;
            }
        }
        _ => {} // Ignore other CSI sequences (styling, erase, etc.)
    }
}

fn skip_osc_sequence(chars: &mut std::iter::Peekable<std::str::Chars>) {
    chars.next(); // consume ']'
    while let Some(ch) = chars.next() {
        if ch == '\x07' || (ch == '\x1b' && chars.peek() == Some(&'\\')) {
            if ch == '\x1b' {
                chars.next();
            }
            break;
        }
    }
}

fn skip_dec_private_mode(chars: &mut std::iter::Peekable<std::str::Chars>) {
    chars.next(); // consume '?'
    while let Some(&ch) = chars.peek() {
        if ch.is_ascii_alphabetic() {
            chars.next();
            break;
        } else {
            chars.next();
        }
    }
}

fn extract_world_position(screen: &[Vec<char>]) -> Option<(i32, i32)> {
    // Status bar is in the first few rows
    for row_idx in 0..3 {
        let line: String = screen[row_idx].iter().collect();

        if let Some(x_pos) = line.find("x: ") {
            let after_x = &line[x_pos + 3..];
            let x_str: String = after_x
                .chars()
                .take_while(|c| c.is_numeric() || *c == '-')
                .collect();

            if let Ok(x) = x_str.parse::<i32>() {
                if let Some(y_pos) = after_x.find("y: ") {
                    let after_y = &after_x[y_pos + 3..];
                    let y_str: String = after_y
                        .chars()
                        .take_while(|c| c.is_numeric() || *c == '-')
                        .collect();

                    if let Ok(y) = y_str.parse::<i32>() {
                        return Some((x, y));
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn screen_buffer_handles_cursor_positioning() {
        let mut buffer = ScreenBuffer::new();
        buffer.apply_ansi_updates("\x1b[5;10Hx");

        let screen = buffer.get_screen();
        assert_eq!(screen[4][9], 'x', "Character should be at row 4, col 9");
    }

    #[test]
    fn screen_buffer_accumulates_incremental_updates() {
        let mut buffer = ScreenBuffer::new();

        buffer.apply_ansi_updates("\x1b[6;11H@");
        buffer.apply_ansi_updates("\x1b[6;12H·");

        let screen = buffer.get_screen();
        assert_eq!(screen[5][10], '@', "First character should persist");
        assert_eq!(screen[5][11], '·', "Second character should be added");
    }

    #[test]
    fn extract_world_position_parses_status_bar() {
        let mut screen = vec![vec![' '; SCREEN_COLS]; SCREEN_ROWS];
        let status_line = "│ Wanderer | Pos: Some(Position { x: 42, y: 17 })";
        for (i, ch) in status_line.chars().enumerate() {
            if i < SCREEN_COLS {
                screen[1][i] = ch;
            }
        }

        let pos = extract_world_position(&screen);
        assert_eq!(pos, Some((42, 17)), "Should extract position from status");
    }
}
