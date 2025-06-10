use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::app::{Creds, CurrentScreen, Popup};

impl super::super::App {
    pub fn show_key_handler(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.current_screen = CurrentScreen::Main,
            KeyCode::Char('j') | KeyCode::Down => self.next_row(),
            KeyCode::Char('k') | KeyCode::Up => self.previous_row(),
            KeyCode::Char('c') => self.copy_password(),
            KeyCode::Char('d') => {
                self.prev_popup = self.current_popup;
                self.current_popup = Popup::Confirm;
            }
            KeyCode::Char('f') => {
                self.current_param = Some(Creds::Sitename);
                self.prev_popup = self.current_popup;
                self.current_popup = Popup::Filter;
            }
            KeyCode::Char('u') => {
                self.current_param = Some(Creds::Sitename);
                self.load_values();
                self.current_popup = Popup::Update;
            }
            _ => {}
        }
    }
}
