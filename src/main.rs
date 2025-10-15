use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::prelude::*;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  loop {
    terminal.draw(|f| ui(f))?;

    if let Event::Key(key) = event::read()? {
      if key.code == KeyCode::Char('q') {
        break;
      }
    }
  }

  disable_raw_mode()?;
  execute!(
    terminal.backend_mut(),
    LeaveAlternateScreen,
    DisableMouseCapture
  )?;
  terminal.show_cursor()?;

  Ok(())
}

fn ui(f: &mut Frame) {
  f.render_widget(
    ratatui::widgets::Paragraph::new("Rust TUI Starter - Press 'q' to quit"),
    f.area(),
  );
}