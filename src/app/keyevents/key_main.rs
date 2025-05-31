use ratatui::crossterm::event::{KeyCode, KeyEvent};

use super::super::{Creds, CurrentScreen};

impl super::super::App {
    pub fn main_key_handler(&mut self, key: KeyEvent) {
        self.clear_input();
        match key.code {
            KeyCode::Char('a') => {
                self.current_screen = CurrentScreen::Add;
                self.current_param = Some(Creds::Sitename);
            }
            KeyCode::Char('f') => {
                self.current_screen = CurrentScreen::Filter;
                self.current_param = Some(Creds::Sitename);
            }
            KeyCode::Char('s') => {
                self.current_screen = CurrentScreen::Show;
            }
            KeyCode::Char('q') => {
                self.current_screen = CurrentScreen::Exit;
            }
            _ => {}
        }
    }
    fn clear_input(&mut self) {
        self.entry_key = String::new();
        self.site_input = String::new();
        self.url_input = String::new();
        self.gmail_input = String::new();
        self.user_input = String::new();
        self.pass_input = String::new();
        self.masked_pass = String::new();
    }
}
