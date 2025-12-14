//! Terminal rendering using ratatui
//!
//! This module isolates all external UI dependencies (ratatui, crossterm)
//! keeping them out of the domain core.

use crate::foundation::Position;
use crate::patterns::modes::Mode;
use crate::tapestry::Tapestry;
use crate::threads::ThreadKind;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;

pub type TerminalType = Terminal<CrosstermBackend<io::Stdout>>;

/// Initialize terminal for rendering
pub fn init_terminal() -> io::Result<TerminalType> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

/// Restore terminal to normal mode
pub fn restore_terminal(terminal: &mut TerminalType) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

/// Render the current game state
pub fn render(
    terminal: &mut TerminalType,
    tapestry: &Tapestry,
    player_id: crate::threads::ThreadId,
    mode: &Mode,
) -> io::Result<()> {
    terminal.draw(|f| {
        // Determine if we need a command line row (Ex mode only)
        let has_command_line = mode.is_ex();

        let mut constraints = vec![
            Constraint::Length(3), // Status bar
            Constraint::Min(1),    // Game view
            Constraint::Length(3), // Mode indicator bar
        ];

        if has_command_line {
            constraints.push(Constraint::Length(1)); // Command line (no border)
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(f.area());

        // Status bar
        let player = tapestry.get_thread(player_id);
        let status_text = if let Some(thread) = player {
            if let ThreadKind::Player { name } = &thread.kind {
                format!(" {} | Pos: {:?}", name, thread.position)
            } else {
                " Unknown".to_string()
            }
        } else {
            " No player".to_string()
        };

        let status = Paragraph::new(status_text)
            .block(Block::default().borders(Borders::ALL).title("Vitalis"));
        f.render_widget(status, chunks[0]);

        // Game view
        let game_view = render_game_view(tapestry, player_id, chunks[1]);
        f.render_widget(game_view, chunks[1]);

        // Mode indicator bar (mode name on left, pending keys on right)
        let mode_name = mode.mode_name();
        let pending_keys = mode.pending_keys();
        let mode_line = if !pending_keys.is_empty() {
            // Calculate spacing to right-align pending keys
            let bar_width = chunks[2].width.saturating_sub(4) as usize; // Account for borders
            let mode_len = mode_name.len();
            let keys_len = pending_keys.len();
            let spacing = bar_width.saturating_sub(mode_len + keys_len);
            format!("{}{:width$}{}", mode_name, "", pending_keys, width = spacing)
        } else {
            mode_name.to_string()
        };

        let mode_bar = Paragraph::new(mode_line)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(mode_bar, chunks[2]);

        // Command line (Ex mode only, no border)
        if let Some(cmd_line) = mode.command_line() {
            let command_paragraph = Paragraph::new(cmd_line);
            f.render_widget(command_paragraph, chunks[3]);
        }
    })?;
    Ok(())
}

/// Render the game world view
fn render_game_view(
    tapestry: &Tapestry,
    player_id: crate::threads::ThreadId,
    area: Rect,
) -> Paragraph<'static> {
    let player = tapestry.get_thread(player_id);
    let player_pos = player.and_then(|t| t.position).unwrap_or(Position::new(0, 0));

    // Create a simple view centered on player
    let view_width = (area.width - 2) as i32; // Account for borders
    let view_height = (area.height - 2) as i32;

    let start_x = player_pos.x - view_width / 2;
    let start_y = player_pos.y - view_height / 2;

    let mut lines = Vec::new();
    for y in 0..view_height {
        let mut line_spans = Vec::new();
        for x in 0..view_width {
            let world_pos = Position::new(start_x + x, start_y + y);

            let symbol = if world_pos == player_pos {
                '@'
            } else if let Some(thread) = tapestry.get_thread_at(world_pos) {
                match thread.kind {
                    ThreadKind::Region { .. } => '·',
                    ThreadKind::Npc { .. } => 'N',
                    ThreadKind::Player { .. } => '@',
                }
            } else {
                ' '
            };

            line_spans.push(Span::styled(
                symbol.to_string(),
                Style::default().fg(Color::White),
            ));
        }
        lines.push(Line::from(line_spans));
    }

    Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title("The Tapestry"))
}
