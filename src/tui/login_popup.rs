use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;

pub fn login_pop_up(frame: &mut Frame, app: &App) {
    let popup_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::DarkGray));

    let area = centered_rect(30, 12, frame.area());
    //frame.render_widget(popup_block, area);

    let title_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::DarkGray));

    let title = Paragraph::new(Text::styled(
        "MASTER KEY",
        Style::default().fg(Color::Green),
    ))
    .block(title_block)
    .alignment(Alignment::Center);

    let password_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::DarkGray));

    let chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

    let masked_password = Paragraph::new(app.masked_pass.clone()).block(password_block);

    frame.render_widget(title, chunk[0]);

    frame.render_widget(masked_password, chunk[1]);
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage((percent_y)),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage((percent_x)),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
