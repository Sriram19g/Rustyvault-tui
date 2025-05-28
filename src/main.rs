use std::{collections::btree_map::VacantEntry, error::Error, io};

use app::{App, Creds, CurrentScreen};
use ratatui::{
    Terminal,
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    prelude::{Backend, CrosstermBackend},
};

mod app;

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
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('a') => {
                        app.current_screen = CurrentScreen::Add;
                        app.current_param = Some(Creds::Sitename);
                    }
                    KeyCode::Char('f') => {
                        app.current_screen = CurrentScreen::Filter;
                        app.current_param = Some(Creds::Sitename);
                    }
                    KeyCode::Char('s') => {
                        app.current_screen = CurrentScreen::Show;
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exit;
                    }
                    _ => {}
                },
                CurrentScreen::Add | CurrentScreen::Update if key.kind == KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Enter => {
                            if let Some(adding) = &app.current_param {
                                match adding {
                                    Creds::Sitename => {
                                        app.current_param = Some(Creds::Siteurl);
                                    }
                                    Creds::Siteurl => {
                                        app.current_param = Some(Creds::Gmail);
                                    }
                                    Creds::Gmail => {
                                        app.current_param = Some(Creds::Username);
                                    }
                                    Creds::Username => {
                                        app.current_param = Some(Creds::Password);
                                    }
                                    Creds::Password => {
                                        app.save_credentials();
                                        app.current_screen = CurrentScreen::Main;
                                    }
                                }
                            }
                        }
                        KeyCode::Backspace => {
                            if let Some(adding) = &app.current_param {
                                match adding {
                                    Creds::Sitename => {
                                        app.site_input.pop();
                                    }
                                    Creds::Siteurl => {
                                        app.url_input.pop();
                                    }
                                    Creds::Gmail => {
                                        app.gmail_input.pop();
                                    }
                                    Creds::Username => {
                                        app.user_input.pop();
                                    }
                                    Creds::Password => {
                                        app.pass_input.pop();
                                    }
                                }
                            }
                        }
                        KeyCode::Esc => {
                            app.current_screen = CurrentScreen::Main;
                            app.current_param = None;
                        }
                        KeyCode::Tab => {
                            app.toggle_params();
                        }
                        KeyCode::Char(value) => {
                            if let Some(adding) = &app.current_param {
                                match adding {
                                    Creds::Sitename => {
                                        app.site_input.push(value);
                                    }
                                    Creds::Siteurl => {
                                        app.url_input.push(value);
                                    }
                                    Creds::Gmail => {
                                        app.gmail_input.push(value);
                                    }
                                    Creds::Username => {
                                        app.user_input.push(value);
                                    }
                                    Creds::Password => {
                                        app.pass_input.push(value);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                CurrentScreen::Login if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => {
                        app.check_password();
                        if app.is_login {
                            app.current_screen = CurrentScreen::Main;
                        } else {
                            return Ok(false);
                        }
                    }
                    KeyCode::BackTab => {
                        app.entry_key.pop();
                    }
                    KeyCode::Char(value) => {
                        app.entry_key.push('*');
                    }
                    _ => {}
                },
                CurrentScreen::Filter if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => {
                        if let Some(filter_param) = &app.current_param {
                            match filter_param {
                                Creds::Sitename => {
                                    app.current_param = Some(Creds::Siteurl);
                                }
                                Creds::Siteurl => {
                                    app.current_param = Some(Creds::Gmail);
                                }
                                Creds::Gmail => {
                                    app.current_param = Some(Creds::Username);
                                }
                                Creds::Username => {
                                    app.save_credentials();
                                    app.current_screen = CurrentScreen::Main;
                                }
                                _ => {}
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        if let Some(filter_param) = &app.current_param {
                            match filter_param {
                                Creds::Sitename => {
                                    app.site_input.pop();
                                }
                                Creds::Siteurl => {
                                    app.url_input.pop();
                                }
                                Creds::Gmail => {
                                    app.gmail_input.pop();
                                }
                                Creds::Username => {
                                    app.user_input.pop();
                                }
                                _ => {}
                            }
                        }
                    }
                    KeyCode::Tab => {
                        app.toggle_filter_param();
                    }
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                        app.current_param = None;
                    }
                    KeyCode::Char(value) => {
                        if let Some(adding) = &app.current_param {
                            match adding {
                                Creds::Sitename => {
                                    app.site_input.push(value);
                                }
                                Creds::Siteurl => {
                                    app.url_input.push(value);
                                }
                                Creds::Gmail => {
                                    app.gmail_input.push(value);
                                }
                                Creds::Username => {
                                    app.user_input.push(value);
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                },
            }
        }
    }

    Ok(true)
}
