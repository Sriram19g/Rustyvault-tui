use std::{error::Error, io};

use app::{App, CurrentScreen};
use ratatui::{
    Terminal,
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    prelude::{Backend, CrosstermBackend},
};

mod app;
mod tui;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| tui::ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => app.main_key_handler(key),
                CurrentScreen::Add if key.kind == KeyEventKind::Press => app.add_key_handler(key),
                CurrentScreen::Update if key.kind == KeyEventKind::Press => {
                    app.update_key_handler(key)
                }
                CurrentScreen::Login if key.kind == KeyEventKind::Press => {
                    if !app.login_key_handler(key) {
                        return Ok(false);
                    }
                }
                CurrentScreen::Filter if key.kind == KeyEventKind::Press => {
                    app.filter_key_handler(key)
                }
                CurrentScreen::Show => app.show_key_handler(key),
                CurrentScreen::Confirm => {}
                CurrentScreen::Exit => {}
                _ => {}
            }
        }
    }

    Ok(true)
}
