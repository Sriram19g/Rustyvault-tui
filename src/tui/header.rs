use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
};

pub fn header_layout(frame: &mut Frame, area: Rect) {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "RustyVault",
        Style::default().fg(Color::Magenta),
    ))
    .alignment(Alignment::Center)
    .block(title_block);

    frame.render_widget(title, area);
}
