use ratatui::{
    Frame,
    layout::{Constraint, Margin, Rect},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Cell, HighlightSpacing, Row, Scrollbar, ScrollbarOrientation, Table},
};
use unicode_width::UnicodeWidthStr;

use crate::app::App;
use crate::app::Credential;

pub fn show_page(frame: &mut Frame, area: Rect, app: &mut App) {
    render_table(frame, area, app);
    render_scrollbar(frame, area, app);
}

fn render_table(frame: &mut Frame, area: Rect, app: &mut App) {
    let header_style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD);

    let selected_row_style = Style::default()
        .bg(Color::Blue)
        .fg(Color::Black)
        .add_modifier(Modifier::ITALIC);
    let normal_row_style = Style::default().bg(Color::Black).fg(Color::White);
    let alt_row_style = Style::default().bg(Color::DarkGray).fg(Color::White);

    let header = ["Sitename", "Siteurl", "Email", "Username", "Password"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(header_style)
        .height(1);

    let rows = app.credentials.iter().enumerate().map(|(i, data)| {
        let styles = match i % 2 {
            0 => normal_row_style,
            _ => alt_row_style,
        };

        let item = data.ref_array();
        item.into_iter()
            .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
            .collect::<Row>()
            .style(styles)
            .height(4)
    });

    let bar = " █ ";
    let longest_item_lens = constraint_len_calculator(&app.credentials);
    let t = Table::new(
        rows,
        [
            Constraint::Length(longest_item_lens.0 + 1),
            Constraint::Min(longest_item_lens.1 + 1),
            Constraint::Min(longest_item_lens.2 + 1),
            Constraint::Min(longest_item_lens.3 + 1),
            Constraint::Min(longest_item_lens.4),
        ],
    )
    .header(header)
    .row_highlight_style(selected_row_style)
    .highlight_symbol(Text::from(vec![
        "".into(),
        bar.into(),
        bar.into(),
        "".into(),
    ]))
    .highlight_spacing(HighlightSpacing::Always);
    frame.render_stateful_widget(t, area, &mut app.state);
}

fn render_scrollbar(frame: &mut Frame, area: Rect, app: &mut App) {
    frame.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None),
        area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        }),
        &mut app.scroll_state,
    );
}

fn constraint_len_calculator(items: &[Credential]) -> (u16, u16, u16, u16, u16) {
    let sitename_len = items
        .iter()
        .map(Credential::site_name)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    let siteurl_len = items
        .iter()
        .map(Credential::url)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    let gmail_len = items
        .iter()
        .map(Credential::gmail)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    let user_len = items
        .iter()
        .map(Credential::username)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    let pass_len = items
        .iter()
        .map(Credential::password)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    (
        sitename_len as u16,
        siteurl_len as u16,
        gmail_len as u16,
        user_len as u16,
        pass_len as u16,
    )
}
