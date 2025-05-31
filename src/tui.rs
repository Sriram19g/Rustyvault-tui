use footer::footer_layout;
use header::header_layout;
use login_popup::login_pop_up;
use main_page::main_pg;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

use crate::app::{App, CurrentScreen};
mod footer;
mod header;
mod login_popup;
mod main_page;

pub fn ui(frame: &mut Frame, app: &App) {
    if let CurrentScreen::Login = app.current_screen {
        login_pop_up(frame, app);
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    header_layout(frame, chunks[0]);
    footer_layout(frame, chunks[2], app);

    if let CurrentScreen::Main = app.current_screen {
        main_pg(frame, chunks[1]);
    }
}
