use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

pub fn main_pg(frame: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(12), Constraint::Min(1)])
        .split(area);

    head_banner(frame, chunks[0]);
    content_block(frame, chunks[1]);
}

fn head_banner(frame: &mut Frame, area: Rect) {
    let banner = "

██████╗ ██╗   ██╗███████╗████████╗██╗   ██╗██╗   ██╗ █████╗ ██╗   ██╗██╗  ████████╗
██╔══██╗██║   ██║██╔════╝╚══██╔══╝╚██╗ ██╔╝██║   ██║██╔══██╗██║   ██║██║  ╚══██╔══╝
██████╔╝██║   ██║███████╗   ██║    ╚████╔╝ ██║   ██║███████║██║   ██║██║     ██║   
██╔══██╗██║   ██║╚════██║   ██║     ╚██╔╝  ╚██╗ ██╔╝██╔══██║██║   ██║██║     ██║   
██║  ██║╚██████╔╝███████║   ██║      ██║    ╚████╔╝ ██║  ██║╚██████╔╝███████╗██║   
╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝      ╚═╝     ╚═══╝  ╚═╝  ╚═╝ ╚═════╝ ╚══════╝╚═╝   
                                                                                   
";
    let banner_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default());

    let banner_paragraph = Paragraph::new(banner)
        .style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(banner_block);
    frame.render_widget(banner_paragraph, area);
}

fn content_block(frame: &mut Frame, area: Rect) {
    let text = vec![
        Line::from(" • Add entries to the vault                    [a]")
            .alignment(Alignment::Center)
            .blue()
            .into(),
        "".into(),
        Line::from(" • Show all entries in the vault               [s]")
            .alignment(Alignment::Center)
            .blue()
            .into(),
        "".into(),
        Line::from(" • Quit from the vault                         [q]")
            .alignment(Alignment::Center)
            .blue()
            .into(),
        "".into(),
    ];

    let dashboard = Paragraph::new(text).block(Block::default().borders(Borders::NONE));

    frame.render_widget(dashboard, area);
}
