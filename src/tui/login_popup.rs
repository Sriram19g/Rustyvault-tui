use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::{App, Confirmval};

pub fn login_pop_up(frame: &mut Frame, app: &App) {
    let popup_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::DarkGray));

    let area = centered_rect(35, 22, frame.area());

    frame.render_widget(popup_block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .split(area);
    header(frame, chunks[0]);
    let box1 = input_box(chunks[1]);

    let mut block1 = Block::default().title("Password").borders(Borders::ALL);

    let active_style = Style::default().bg(Color::Black).fg(Color::White);

    block1 = block1.style(active_style);

    let block1_text = Paragraph::new(app.masked_pass.clone())
        .block(block1)
        .alignment(Alignment::Center);

    frame.render_widget(block1_text, box1);

    let text = input_box(chunks[2]);

    let text_content = Span::styled(
        format!("Attempt : {}", app.attempt),
        Style::default().fg(Color::Red),
    );

    let content_paragraph = Paragraph::new(vec!["".into(), text_content.into()]);

    frame.render_widget(content_paragraph, text);
    buttons(frame, chunks[3], app);
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
fn header(frame: &mut Frame, area: Rect) {
    let add_block = Block::default().borders(Borders::NONE);
    let heading = Paragraph::new(vec!["".into(), "Login".into()])
        .style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(add_block);

    frame.render_widget(heading, area);
}

fn input_box(area: Rect) -> Rect {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(area)[1]
}
fn buttons(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ])
        .split(area);
    let mut ok_block = Block::default().borders(Borders::ALL);
    let mut reset_block = Block::default().borders(Borders::ALL);

    let active_style = Style::default().bg(Color::Black).fg(Color::White);

    match app.confirm_state {
        Confirmval::Yes => ok_block = ok_block.style(active_style),
        Confirmval::No => reset_block = reset_block.style(active_style),
    }

    let ok = Paragraph::new("Enter")
        .alignment(Alignment::Center)
        .block(ok_block);

    let reset = Paragraph::new("Reset")
        .alignment(Alignment::Center)
        .block(reset_block);

    frame.render_widget(ok, chunks[3]);
    frame.render_widget(reset, chunks[1]);
}
