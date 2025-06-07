use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::app::{App, Confirmval, CurrentScreen, Popup};

pub fn confirm_popup(frame: &mut Frame, app: &App) {
    let popup_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::LightBlue));

    let area = centered_rect(35, 22, frame.area());
    frame.render_widget(Clear, area);
    frame.render_widget(popup_block, area);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .split(area);

    header(frame, chunks[0]);
    content(frame, chunks[1], app);
    buttons(frame, chunks[2], app);
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
    let heading = Paragraph::new(vec!["".into(), "Confirmation".into()])
        .style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(add_block);

    frame.render_widget(heading, area);
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
    let mut yes_block = Block::default().borders(Borders::ALL);
    let mut no_block = Block::default().borders(Borders::ALL);

    let active_style = Style::default().bg(Color::White).fg(Color::LightBlue);

    match app.confirm_state {
        Confirmval::Yes => yes_block = yes_block.style(active_style),
        Confirmval::No => no_block = no_block.style(active_style),
    }

    let yes = Paragraph::new("Yes")
        .alignment(Alignment::Center)
        .block(yes_block);

    let no = Paragraph::new("No")
        .alignment(Alignment::Center)
        .block(no_block);

    frame.render_widget(yes, chunks[1]);
    frame.render_widget(no, chunks[3]);
}

fn content(frame: &mut Frame, area: Rect, app: &App) {
    let text_context = match app.current_screen {
        CurrentScreen::Main => {
            Span::styled("Are you sure to quit?", Style::default().fg(Color::White))
        }
        CurrentScreen::Add => Span::styled(
            "Are you sure to add this entry?",
            Style::default().fg(Color::White),
        ),
        CurrentScreen::Show => match app.prev_popup {
            Popup::None => Span::styled(
                "Are you sure to delete this entry?",
                Style::default().fg(Color::White),
            ),
            Popup::Update => Span::styled(
                "Are you sure to update this entry?",
                Style::default().fg(Color::White),
            ),
            _ => Span::default(),
        },
        _ => Span::default(),
    };
    let content_paragraph = Paragraph::new(text_context).alignment(Alignment::Center);
    frame.render_widget(content_paragraph, area);
}
