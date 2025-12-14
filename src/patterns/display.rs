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
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Status bar
                Constraint::Min(1),    // Game view
                Constraint::Length(3), // Message bar
            ])
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

        // Message bar
        let mode_display = mode.display();
        let message = Paragraph::new(mode_display)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(message, chunks[2]);
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
