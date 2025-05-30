use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::{App, CurrentScreen};

pub fn footer_layout(frame: &mut Frame, area: Rect, app: &App) {
    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let current_navigation_text = vec![
        match app.current_screen {
            CurrentScreen::Main => Span::styled("Main Screen", Style::default().fg(Color::Green)),
            CurrentScreen::Add => Span::styled("Add Screen", Style::default().fg(Color::White)),
            CurrentScreen::Show => {
                Span::styled("Display Screen", Style::default().fg(Color::Yellow))
            }
            CurrentScreen::Update => {
                Span::styled("Update Screen", Style::default().fg(Color::Gray))
            }
            CurrentScreen::Exit => Span::styled("Exit Screen", Style::default().fg(Color::Cyan)),
            CurrentScreen::Filter => {
                Span::styled("Filter Screen", Style::default().fg(Color::DarkGray))
            }
            CurrentScreen::Login => Span::styled("Login Screen", Style::default().fg(Color::Green)),
            CurrentScreen::Confirm => Span::styled("Confirm", Style::default().fg(Color::Green)),
        }
        .to_owned(),
        Span::styled(" | ", Style::default().fg(Color::White)),
        Span::styled("Messages", Style::default().fg(Color::Red)),
    ];
    let nav_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "(q) to quit / (e) to make new pair",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Add => Span::styled(
                "(ESC) to cancel/(Tab) to switch boxes/enter to complete",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Update => Span::styled(
                "(q) to quit / (e) to make new pair",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Show => Span::styled(
                "(q) to quit / (e) to make new pair",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Filter => Span::styled(
                "(ESC) to cancel/(Tab) to switch boxes/enter to complete",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Exit => {
                Span::styled("(y) yes / (n) no", Style::default().fg(Color::Red))
            }
            CurrentScreen::Confirm => Span::styled(
                "(q) to quit / (e) to make new pair",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Login => Span::styled(
                "(ESC) to cancel/(Tab) to switch boxes/enter to complete",
                Style::default().fg(Color::Red),
            ),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    frame.render_widget(nav_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);
}
