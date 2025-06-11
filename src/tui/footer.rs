use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::{App, Creds, CurrentScreen, LoginPopup, Popup};

pub fn footer_layout(frame: &mut Frame, area: Rect, app: &App) {
    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

    let current_navigation_text = vec![
        match app.current_screen {
            CurrentScreen::Main => Span::styled("Main Screen", Style::default().fg(Color::Green)),
            CurrentScreen::Add => Span::styled("Add Screen", Style::default().fg(Color::Green)),
            CurrentScreen::Show => match app.current_popup {
                Popup::None => Span::styled("Display Screen", Style::default().fg(Color::Green)),
                Popup::Update => Span::styled("Update Screen", Style::default().fg(Color::Green)),
                Popup::Confirm => Span::styled("Confirm Screen", Style::default().fg(Color::Green)),
                Popup::Filter => Span::styled("Filter Screen", Style::default().fg(Color::Green)),
            },
            // CurrentScreen::Update => {
            //     Span::styled("Update Screen", Style::default().fg(Color::Gray))
            // }
            CurrentScreen::Login => match app.login_state {
                LoginPopup::Login => {
                    Span::styled("Login Screen", Style::default().fg(Color::Green))
                }
                LoginPopup::New => {
                    Span::styled("Sign Up Screen", Style::default().fg(Color::Green))
                }
                LoginPopup::Reset => {
                    Span::styled("Reset Screen", Style::default().fg(Color::Green))
                }
            },
        }
        .to_owned(),
        Span::styled(" | ", Style::default().fg(Color::White)),
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "Welcome to Secure Password manager",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Add => {
                if let Some(adding) = &app.current_param {
                    match adding {
                        Creds::Sitename => {
                            Span::styled("Adding Sitename", Style::default().fg(Color::Red))
                        }
                        Creds::Siteurl => {
                            Span::styled("Adding Siteurl", Style::default().fg(Color::Red))
                        }
                        Creds::Gmail => {
                            Span::styled("Adding Email", Style::default().fg(Color::Red))
                        }
                        Creds::Username => {
                            Span::styled("Adding Username", Style::default().fg(Color::Red))
                        }
                        Creds::Password => {
                            Span::styled("Adding Password", Style::default().fg(Color::Red))
                        }
                    }
                } else {
                    Span::styled("", Style::default())
                }
            }

            CurrentScreen::Show => match app.current_popup {
                Popup::None => Span::styled("", Style::default().fg(Color::Green)),
                Popup::Update => {
                    if let Some(updating) = &app.current_param {
                        match updating {
                            Creds::Sitename => {
                                Span::styled("Updating Sitename", Style::default().fg(Color::Red))
                            }
                            Creds::Siteurl => {
                                Span::styled("Updating Siteurl", Style::default().fg(Color::Red))
                            }
                            Creds::Gmail => {
                                Span::styled("Updating Email", Style::default().fg(Color::Red))
                            }
                            Creds::Username => {
                                Span::styled("Updating Username", Style::default().fg(Color::Red))
                            }
                            Creds::Password => {
                                Span::styled("Updaing Password", Style::default().fg(Color::Red))
                            }
                        }
                    } else {
                        Span::styled("", Style::default())
                    }
                }
                _ => Span::styled("", Style::default()),
            },
            _ => Span::styled("", Style::default()),
        }
        .to_owned(),
    ];
    let nav_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "Created by Cyberhunter   |  https://github.com/Sriram19g  |  gsriram200@gmail.com ",
                Style::default().fg(Color::Yellow),
            ),
            CurrentScreen::Add => Span::styled(
                "                (ESC) to cancel    |    (Tab) to switch boxes    |    (enter) to complete                ",
                Style::default().fg(Color::Yellow),
            ),

            CurrentScreen::Show => match app.current_popup {
                Popup::None => Span::styled(
                    "(ESC / q) Main  |  (j / ↓) Down  |  (k / ↑) Up  |  (c) Copy Password  |  (d) Delete Entry  |  (f) Filter ",
                    Style::default().fg(Color::Yellow),
                ),
                Popup::Update => Span::styled(
                    "                (ESC) to cancel    |    (Tab) to switch boxes    |    (enter) to complete                ",
                    Style::default().fg(Color::Yellow),
                ),
                Popup::Confirm => Span::styled(
                    "          (n) No   |   (y) YES  |   (Esc) Cancel   |   (Enter) Action   |   (Tab) Switch button          ",
                    Style::default().fg(Color::Yellow),
                ),
                Popup::Filter => Span::styled(
                    "                (ESC) to cancel    |    (Tab) to switch boxes    |    (enter) to complete                ",
                    Style::default().fg(Color::Yellow),
                ),
            },

            CurrentScreen::Login => match app.login_state {
                LoginPopup::Login => Span::styled(
                    "               (ESC) to cancel    |    (Tab) to switch button    |    (enter) to complete                ",
                    Style::default().fg(Color::Yellow),
                ),
                _ => Span::styled(
                    "                (ESC) to cancel    |    (Tab) to switch boxes    |    (enter) to complete                ",
                    Style::default().fg(Color::Yellow),
                ),
            },
        }
    };

    let key_notes_footer = Paragraph::new(Line::from(current_keys_hint))
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);

    frame.render_widget(nav_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);
}
