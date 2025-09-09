use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::io::{Read, Write};
use std::time::{Duration, Instant};

/// Black-box component test for player movement in square pattern
/// Tests the actual vitalis binary by spawning it in a PTY and sending hjkl inputs
#[test]
fn player_moves_in_square_returning_to_origin() {
    // Create a PTY with fixed size (80x24 like classic terminal)
    let pty_system = native_pty_system();
    let pty_pair = pty_system
        .openpty(PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        })
        .expect("Failed to create PTY");

    // Build command to run our vitalis binary
    let mut cmd = CommandBuilder::new(env!("CARGO_BIN_EXE_vitalis"));
    let mut child = pty_pair
        .slave
        .spawn_command(cmd)
        .expect("Failed to spawn vitalis");

    let mut reader = pty_pair.master.try_clone_reader().unwrap();
    let mut writer = pty_pair.master.take_writer().unwrap();

    // Give the app time to initialize
    std::thread::sleep(Duration::from_millis(100));

    // Read initial screen state
    let initial_output = read_with_timeout(&mut reader, Duration::from_millis(500));
    println!("Initial screen:\n{}", initial_output);

    // Parse the screen output into lines for position-based testing
    let screen_lines: Vec<&str> = initial_output.lines().collect();

    // Terminal layout: 80x24 with 3-line status bar at bottom
    // Game area: lines 0-20 (21 lines), Status area: lines 21-23 (3 lines)
    // Player starts at center of game area: (40, 10) - middle of 80-wide, 21-high area
    let game_area_width = 80;
    let game_area_height = 24 - 3; // 24 total - 3 status lines
    let center_x = game_area_width / 2; // 40 (0-indexed: 39)
    let center_y = game_area_height / 2; // 10 (0-indexed: 10)

    assert!(
        screen_lines.len() >= 24,
        "Expected exactly 24 lines of output (21 game + 3 status). Got: {}",
        screen_lines.len()
    );

    // Verify player '@' symbol appears at center of game area
    let game_line = screen_lines[center_y];
    assert!(
        game_line.len() > center_x,
        "Expected line {} to have at least {} characters for center position. Got: '{}'",
        center_y,
        center_x + 1,
        game_line
    );

    let char_at_center = game_line.chars().nth(center_x);
    assert!(
        char_at_center == Some('@'),
        "Expected '@' at center position ({},{}) but found {:?}. Line: '{}'",
        center_x,
        center_y,
        char_at_center,
        game_line
    );

    // Verify 3-line status bar: middle line (line 22) contains mode and coordinates
    // Mode on far left, coordinates on far right
    let status_line = screen_lines[22]; // Middle of 3-line status (lines 21, 22, 23)
    assert!(
        status_line.starts_with("-- NORMAL --"),
        "Expected status line to start with '-- NORMAL --' on far left. Got: '{}'",
        status_line
    );
    assert!(
        status_line.trim_end().ends_with("[0,0]"),
        "Expected status line to end with '[0,0]' on far right. Got: '{}'",
        status_line
    );

    // Execute square movement from center: left(h) -> down(j) -> right(l) -> up(k)
    // Player starts at center (40, 10) with game coordinates (0, 0)
    let moves = [
        ('h', "left", center_x - 1, center_y, "[-1,0]"), // Move left: @ at screen (39, 10)
        ('j', "down", center_x - 1, center_y + 1, "[-1,1]"), // Move down: @ at screen (39, 11)
        ('l', "right", center_x, center_y + 1, "[0,1]"), // Move right: @ at screen (40, 11)
        ('k', "up", center_x, center_y, "[0,0]"),        // Move up: @ back to screen (40, 10)
    ];

    for (i, (key, direction, expected_screen_x, expected_screen_y, expected_coords)) in
        moves.iter().enumerate()
    {
        println!("\nMove {}: {} ({})", i + 1, direction, key);

        // Send the movement key
        writer.write_all(&[*key as u8]).expect("Failed to send key");
        writer.flush().expect("Failed to flush");

        // Wait for screen update and read output
        std::thread::sleep(Duration::from_millis(50));
        let output = read_with_timeout(&mut reader, Duration::from_millis(200));

        println!("Screen after move {}:\n{}", i + 1, output);

        // Parse screen into lines for position testing
        let screen_lines: Vec<&str> = output.lines().collect();

        // Verify player '@' symbol appears at expected screen position
        assert!(
            screen_lines.len() > *expected_screen_y,
            "After move {} ({}): expected at least {} lines for row {}. Got: {}",
            i + 1,
            direction,
            expected_screen_y + 1,
            expected_screen_y,
            screen_lines.len()
        );

        let target_line = screen_lines[*expected_screen_y];
        assert!(
            target_line.len() > *expected_screen_x,
            "After move {} ({}): expected line {} to have at least {} characters. Got: '{}'",
            i + 1,
            direction,
            expected_screen_y,
            expected_screen_x + 1,
            target_line
        );

        let char_at_position = target_line.chars().nth(*expected_screen_x);
        assert!(
            char_at_position == Some('@'),
            "After move {} ({}): expected '@' at screen position ({},{}) but found {:?}. Line: '{}'", 
            i + 1, direction, expected_screen_x, expected_screen_y, char_at_position, target_line
        );

        // Verify status line (middle of 3-line status bar) shows correct game coordinates
        assert!(
            screen_lines.len() > 22,
            "After move {} ({}): expected at least 23 lines for status. Got: {}",
            i + 1,
            direction,
            screen_lines.len()
        );
        let status_line = screen_lines[22]; // Middle status line (line 22 of lines 21, 22, 23)
        assert!(
            status_line.trim_end().ends_with(expected_coords),
            "After move {} ({}): expected status line to end with '{}'. Got: '{}'",
            i + 1,
            direction,
            expected_coords,
            status_line
        );

        // Verify NORMAL mode maintained on far left of status line
        assert!(
            status_line.starts_with("-- NORMAL --"),
            "After move {} ({}): expected status to start with '-- NORMAL --'. Got: '{}'",
            i + 1,
            direction,
            status_line
        );
    }

    // Send :quit command to cleanly exit
    writer.write_all(b":quit\r").expect("Failed to send :quit");
    writer.flush().expect("Failed to flush");

    // Wait for clean exit (or timeout after 1 second)
    let exit_timeout = Duration::from_secs(1);
    let start_time = Instant::now();

    loop {
        if let Ok(Some(_)) = child.try_wait() {
            println!("✅ Vitalis exited cleanly");
            break;
        }
        if start_time.elapsed() > exit_timeout {
            println!("⚠️  Timeout waiting for exit, killing process");
            child.kill().ok();
            break;
        }
        std::thread::sleep(Duration::from_millis(10));
    }

    println!("✅ Test passed: Player successfully moved in square pattern and returned to origin!");
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
