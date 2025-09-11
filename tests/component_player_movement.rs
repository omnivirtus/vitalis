use portable_pty::{CommandBuilder, PtySize, native_pty_system};
use std::io::{Read, Write};
use std::time::{Duration, Instant};

/// Black-box component test for player movement in square pattern
/// Tests the actual vitalis binary by spawning it in a PTY and sending hjkl inputs
#[test]
fn player_moves_in_square_returning_to_origin() {
    const CENTER_X: usize = 39; // 0-indexed center of 80-wide terminal
    const CENTER_Y: usize = 10; // 0-indexed center of 21-line game area
    const STATUS_ROW: usize = 22; // Middle of 3-line status bar

    let mut game = VitalisGame::start();
    let screen = game.get_screen();

    expect_screen_box(
        &screen,
        ((CENTER_X, CENTER_Y), (CENTER_X, CENTER_Y)),
        &[&['@']],
    )
    .expect("Player should start at center position");

    expect_screen_box(
        &screen,
        ((74, STATUS_ROW), (78, STATUS_ROW)),
        &[&"[0,0]".chars().collect::<Vec<_>>()],
    )
    .expect("Initial coordinates should be [0,0]");

    expect_screen_box(
        &screen,
        ((0, STATUS_ROW), (11, STATUS_ROW)),
        &[&"-- NORMAL --".chars().collect::<Vec<_>>()],
    )
    .expect("Should start in NORMAL mode");

    let moves = [
        ('h', CENTER_X - 1, CENTER_Y, "[-1,0]"),
        ('j', CENTER_X - 1, CENTER_Y + 1, "[-1,1]"),
        ('l', CENTER_X, CENTER_Y + 1, "[0,1]"),
        ('k', CENTER_X, CENTER_Y, "[0,0]"),
    ];
    for (key, x, y, coords) in moves.iter() {
        let screen = game.send_key(*key);

        expect_screen_box(&screen, ((*x, *y), (*x, *y)), &[&['@']])
            .unwrap_or_else(|e| panic!("After {}: {}", key, e));

        expect_screen_box(
            &screen,
            ((75, STATUS_ROW), (78, STATUS_ROW)),
            &[&coords.chars().collect::<Vec<_>>()],
        )
        .unwrap_or_else(|e| panic!("After {}: coordinate display error: {}", key, e));

        expect_screen_box(
            &screen,
            ((0, STATUS_ROW), (11, STATUS_ROW)),
            &[&"-- NORMAL --".chars().collect::<Vec<_>>()],
        )
        .unwrap_or_else(|e| panic!("After {}: mode display error: {}", key, e));
    }

    game.quit();
}

// ============================================================================
// Supporting Implementation Details
// ============================================================================

/// Game management wrapper for PTY-based testing
struct VitalisGame {
    _child: Box<dyn portable_pty::Child + Send + Sync>,
    reader: Box<dyn Read + Send>,
    writer: Box<dyn Write + Send>,
}

impl VitalisGame {
    /// Start vitalis game in PTY and return management wrapper
    fn start() -> Self {
        let pty_system = native_pty_system();
        let pty_pair = pty_system
            .openpty(PtySize {
                rows: 24,
                cols: 80,
                pixel_width: 0,
                pixel_height: 0,
            })
            .expect("Failed to create PTY");

        let cmd = CommandBuilder::new(env!("CARGO_BIN_EXE_vitalis"));
        let child = pty_pair
            .slave
            .spawn_command(cmd)
            .expect("Failed to spawn vitalis");

        let reader = pty_pair.master.try_clone_reader().unwrap();
        let writer = pty_pair.master.take_writer().unwrap();

        // Give the app time to initialize
        std::thread::sleep(Duration::from_millis(100));

        Self {
            _child: child,
            reader,
            writer,
        }
    }

    /// Get current screen output
    fn get_screen(&mut self) -> String {
        read_with_timeout(&mut self.reader, Duration::from_millis(500))
    }

    /// Send a single key and wait for response
    fn send_key(&mut self, key: char) -> String {
        self.writer
            .write_all(&[key as u8])
            .expect("Failed to send key");
        self.writer.flush().expect("Failed to flush");
        std::thread::sleep(Duration::from_millis(50));
        read_with_timeout(&mut self.reader, Duration::from_millis(200))
    }

    /// Send quit command and clean up
    fn quit(mut self) {
        self.writer
            .write_all(b":quit\r")
            .expect("Failed to send :quit");
        self.writer.flush().expect("Failed to flush");
        // Note: Child cleanup handled by Drop
    }
}

/// Extract 2D character matrix from screen output for specified bounding box
fn extract_screen_box(
    screen: &str,
    bounds: ((usize, usize), (usize, usize)),
) -> Result<Vec<Vec<char>>, String> {
    let lines: Vec<&str> = screen.lines().collect();
    let ((start_col, start_row), (end_col, end_row)) = bounds;

    if start_row > end_row || start_col > end_col {
        return Err(format!(
            "Invalid bounding box: top_left({},{}) must be <= bottom_right({},{})",
            start_col, start_row, end_col, end_row
        ));
    }

    if end_row >= lines.len() {
        return Err(format!(
            "Row {} out of bounds. Screen has {} lines",
            end_row,
            lines.len(),
        ));
    }

    let mut result = Vec::new();
    for row in start_row..=end_row {
        let line = lines[row];
        if end_col >= line.len() {
            return Err(format!(
                "Column {} out of bounds on row {}. Line has {} characters: '{}'",
                end_col,
                row,
                line.len(),
                line
            ));
        }

        let row_chars: Vec<char> = line
            .chars()
            .skip(start_col)
            .take(end_col - start_col + 1)
            .collect();
        result.push(row_chars);
    }

    Ok(result)
}

/// Verify that a screen region matches expected 2D character matrix
fn expect_screen_box(
    screen: &str,
    bounds: ((usize, usize), (usize, usize)),
    expected: &[&[char]],
) -> Result<(), String> {
    let actual = extract_screen_box(screen, bounds)?;
    let (top_left, bottom_right) = bounds;

    if actual.len() != expected.len() {
        return Err(format!(
            "Row count mismatch at {:?}-{:?}: expected {} rows, got {}",
            top_left,
            bottom_right,
            expected.len(),
            actual.len()
        ));
    }

    for (row_idx, (actual_row, expected_row)) in actual.iter().zip(expected.iter()).enumerate() {
        if actual_row.len() != expected_row.len() {
            return Err(format!(
                "Column count mismatch at row {}: expected {} cols, got {}",
                row_idx,
                expected_row.len(),
                actual_row.len()
            ));
        }

        for (col_idx, (actual_char, expected_char)) in
            actual_row.iter().zip(expected_row.iter()).enumerate()
        {
            if actual_char != expected_char {
                return Err(format!(
                    "Character mismatch at ({},{}) in box {:?}-{:?}: expected '{}', got '{}'",
                    top_left.0 + col_idx,
                    top_left.1 + row_idx,
                    top_left,
                    bottom_right,
                    expected_char,
                    actual_char
                ));
            }
        }
    }

    Ok(())
}

/// Read from PTY with timeout to avoid hanging on incomplete output
fn read_with_timeout(reader: &mut Box<dyn Read + Send>, timeout: Duration) -> String {
    let mut output = String::new();
    let mut buffer = [0u8; 1024];
    let start_time = Instant::now();

    loop {
        if start_time.elapsed() > timeout {
            break;
        }

        // Try to read without blocking
        match reader.read(&mut buffer) {
            Ok(0) => break, // EOF
            Ok(n) => {
                let chunk = String::from_utf8_lossy(&buffer[..n]);
                output.push_str(&chunk);

                // If we got some output, continue reading for a bit more
                std::thread::sleep(Duration::from_millis(10));
            }
            Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(10));
                continue;
            }
            Err(err) => {
                eprintln!("Read error: {}", err);
                break;
            }
        }
    }

    output
}
