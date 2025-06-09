use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::app::{App, Creds};

pub fn filter_pop_up(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(area);

    let popup_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::LightMagenta));

    frame.render_widget(Clear, chunks[0]);
    frame.render_widget(popup_block, chunks[0]);
    fields(frame, app, chunks[0]);
}

fn fields(frame: &mut Frame, app: &App, area: Rect) {
    let popup_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(area);

    let mut site_block = Block::default().title("Sitename").borders(Borders::ALL);
    let mut url_block = Block::default().title("Siteurl").borders(Borders::ALL);
    let mut email_block = Block::default().title("Email").borders(Borders::ALL);
    let mut user_block = Block::default().title("Username").borders(Borders::ALL);

    let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

    if let Some(param) = &app.current_param {
        match param {
            Creds::Sitename => {
                site_block = site_block.style(active_style);
            }
            Creds::Siteurl => {
                url_block = url_block.style(active_style);
            }
            Creds::Gmail => {
                email_block = email_block.style(active_style);
            }
            Creds::Username => {
                user_block = user_block.style(active_style);
            }
            _ => {}
        }
    }
    let site_text = Paragraph::new(app.site_input.clone()).block(site_block);
    let url_text = Paragraph::new(app.url_input.clone()).block(url_block);
    let email_text = Paragraph::new(app.gmail_input.clone()).block(email_block);
    let user_text = Paragraph::new(app.user_input.clone()).block(user_block);

    frame.render_widget(site_text, popup_chunks[0]);
    frame.render_widget(url_text, popup_chunks[1]);
    frame.render_widget(email_text, popup_chunks[2]);
    frame.render_widget(user_text, popup_chunks[3]);
}
