use std::rc::Rc;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::{App, Creds};

pub fn add_page(frame: &mut Frame, area: Rect, app: &App) {
    let block1 = get_area(area);

    header(frame, block1[0]);
    body(frame, block1[1], app);
}

fn header(frame: &mut Frame, area: Rect) {
    let add_block = Block::default().borders(Borders::ALL);
    //.style(Style::default().bg(Color::LightCyan));
    let heading = Paragraph::new(vec!["".into(), "Add Entries".into()])
        .style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(add_block);

    frame.render_widget(heading, area);
}

fn body(frame: &mut Frame, area: Rect, app: &App) {
    let content = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(area);
    //let add_block = Block::default().borders(Borders::ALL);

    let site_block = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(content[1]);

    let url_block = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(content[2]);

    let gmail_block = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(content[3]);

    let user_block = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(content[4]);

    let pass_block = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(content[5]);
    //.style(Style::default().bg(Color::LightCyan));

    let site_in = Block::default().borders(Borders::NONE);
    let mut site_in = Paragraph::new(vec!["".into(), app.site_input.clone().into()]).block(site_in);
    let site_txt = Block::default().borders(Borders::NONE);
    let mut site_txt = Paragraph::new(vec![
        "".into(),
        Line::from("  Enter Sitename       :").into(),
    ])
    .block(site_txt);

    let url_in = Block::default().borders(Borders::NONE);
    let mut url_in = Paragraph::new(vec!["".into(), app.url_input.clone().into()]).block(url_in);
    let url_txt = Block::default().borders(Borders::NONE);
    let mut url_txt = Paragraph::new(vec![
        "".into(),
        Line::from("  Enter Siteurl        :").into(),
    ])
    .block(url_txt);

    let gmail_in = Block::default().borders(Borders::NONE);
    let mut gmail_in =
        Paragraph::new(vec!["".into(), app.gmail_input.clone().into()]).block(gmail_in);
    let gmail_txt = Block::default().borders(Borders::NONE);
    let mut gmail_txt = Paragraph::new(vec![
        "".into(),
        Line::from("  Enter Gmail Id       :").into(),
    ])
    .block(gmail_txt);

    let user_in = Block::default().borders(Borders::NONE);
    let mut user_in = Paragraph::new(vec!["".into(), app.user_input.clone().into()]).block(user_in);
    let user_txt = Block::default().borders(Borders::NONE);
    let mut user_txt = Paragraph::new(vec![
        "".into(),
        Line::from("  Enter Username       :").into(),
    ])
    .block(user_txt);

    let pass_in = Block::default().borders(Borders::NONE);
    let mut pass_in =
        Paragraph::new(vec!["".into(), app.masked_pass.clone().into()]).block(pass_in);
    let pass_txt = Block::default().borders(Borders::NONE);
    let mut pass_txt = Paragraph::new(vec![
        "".into(),
        Line::from("  Enter Password       :").into(),
    ])
    .block(pass_txt);

    let active_style = Style::default().bg(Color::Black).fg(Color::White);

    if let Some(param) = &app.current_param {
        match param {
            Creds::Sitename => {
                site_in = site_in.style(active_style);
                site_txt = site_txt.style(active_style);
            }
            Creds::Siteurl => {
                url_in = url_in.style(active_style);
                url_txt = url_txt.style(active_style);
            }
            Creds::Gmail => {
                gmail_in = gmail_in.style(active_style);
                gmail_txt = gmail_txt.style(active_style);
            }
            Creds::Username => {
                user_in = user_in.style(active_style);
                user_txt = user_txt.style(active_style);
            }
            Creds::Password => {
                pass_in = pass_in.style(active_style);
                pass_txt = pass_txt.style(active_style);
            }
        }
    }

    frame.render_widget(&site_txt, site_block[0]);
    frame.render_widget(&site_in, site_block[1]);
    frame.render_widget(&url_txt, url_block[0]);
    frame.render_widget(&url_in, url_block[1]);
    frame.render_widget(&gmail_txt, gmail_block[0]);
    frame.render_widget(&gmail_in, gmail_block[1]);
    frame.render_widget(&user_txt, user_block[0]);
    frame.render_widget(&user_in, user_block[1]);
    frame.render_widget(&pass_txt, pass_block[0]);
    frame.render_widget(&pass_in, pass_block[1]);
}

fn get_area(area: Rect) -> Rc<[Rect]> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(area);

    let rect_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(45),
            Constraint::Percentage(30),
        ])
        .split(chunks[1]);

    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(rect_area[1])
}
