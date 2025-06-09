use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{app::Popup, database::retrive::get_all_entries};

use super::super::{Creds, CurrentScreen};

impl super::super::App {
    pub fn main_key_handler(&mut self, key: KeyEvent) {
        self.clear_input();
        match key.code {
            KeyCode::Char('a') => {
                self.current_screen = CurrentScreen::Add;
                self.current_param = Some(Creds::Sitename);
            }
            KeyCode::Char('s') => {
                self.credentials = match get_all_entries(&mut self.conn) {
                    Ok(data) => data,
                    Err(_) => Vec::new(),
                };
                self.current_screen = CurrentScreen::Show;
            }
            KeyCode::Char('q') => {
                self.prev_popup = self.current_popup;
                self.current_popup = Popup::Confirm;
            }
            _ => {}
        }
    }
}
