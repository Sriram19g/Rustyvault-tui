use footer::footer_layout;
use header::header_layout;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

use crate::app::App;
mod footer;
mod header;

pub fn ui(frame: &mut Frame, app: &App) {
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
}
