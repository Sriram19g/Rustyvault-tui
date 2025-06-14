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

use crate::{
    app::{LoginPopup, Popup},
    database::{delete::delete_all_entries, retrive::get_secret_len},
};
mod app;
mod database;
mod security;
mod tui;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let _ = run_app(&mut terminal, &mut app);

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
                CurrentScreen::Main => {
                    match app.current_popup {
                        Popup::None => app.main_key_handler(key),
                        Popup::Confirm => {
                            if app.confirm_key_handler(key) {
                                return Ok(true);
                            }
                        }
                        _ => {}
                    };
                }
                CurrentScreen::Add if key.kind == KeyEventKind::Press => {
                    match app.current_popup {
                        Popup::None => app.add_key_handler(key),
                        Popup::Confirm => {
                            let _ = app.confirm_key_handler(key);
                        }
                        _ => {}
                    };
                }

                CurrentScreen::Login if key.kind == KeyEventKind::Press => match app.login_state {
                    LoginPopup::Login => {
                        let siz = match get_secret_len(&mut app.conn) {
                            Ok(val) => val,
                            Err(_) => 0,
                        };
                        if siz > 0 {
                            if !app.login_key_handler(key) {
                                return Ok(false);
                            }
                        } else {
                            app.login_state = LoginPopup::New;
                        }
                    }
                    LoginPopup::New => {
                        let _ = delete_all_entries(&mut app.conn);
                        if !app.new_key_handler(key) {
                            return Ok(false);
                        }
                    }
                    LoginPopup::Reset => {
                        app.reset_key_handler(key);
                    }
                },
                CurrentScreen::Show => {
                    match app.current_popup {
                        Popup::None => app.show_key_handler(key),
                        Popup::Update => app.update_key_handler(key),
                        Popup::Confirm => {
                            let _ = app.confirm_key_handler(key);
                        }
                        Popup::Filter => app.filter_key_handler(key),
                    };
                }
                _ => {}
            }
        }
    }
}
