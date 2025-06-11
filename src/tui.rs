use add_tui::add_page;
use footer::footer_layout;
use header::header_layout;
use login_popup::login_pop_up;
use main_page::main_pg;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};
use show_tui::show_page;

use crate::{
    app::{App, CurrentScreen, LoginPopup, Popup},
    tui::{
        confirm_tui::confirm_popup, filter_tui::filter_pop_up, reset_popup::reset_popup,
        signup_popup::signup_popup, update_tui::update_page,
    },
};
mod add_tui;
mod confirm_tui;
mod filter_tui;
mod footer;
mod header;
mod login_popup;
mod main_page;
mod reset_popup;
mod show_tui;
mod signup_popup;
mod update_tui;

pub fn ui(frame: &mut Frame, app: &mut App) {
    if let CurrentScreen::Login = app.current_screen {
        match app.login_state {
            LoginPopup::Login => login_pop_up(frame, app),
            LoginPopup::New => signup_popup(frame, app),
            LoginPopup::Reset => reset_popup(frame, app),
        }
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
        if let Popup::Confirm = app.current_popup {
            confirm_popup(frame, app);
        }
    }
    if let CurrentScreen::Add = app.current_screen {
        add_page(frame, chunks[1], app);
        if let Popup::Confirm = app.current_popup {
            confirm_popup(frame, app);
        }
    }
    if let CurrentScreen::Show = app.current_screen {
        show_page(frame, chunks[1], app);

        match app.current_popup {
            Popup::Update => update_page(frame, chunks[1], app),
            Popup::Confirm => confirm_popup(frame, app),
            Popup::Filter => filter_pop_up(frame, app, chunks[1]),
            _ => {}
        }
    }
}
