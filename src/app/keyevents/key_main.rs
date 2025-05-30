use ratatui::crossterm::event::{KeyCode, KeyEvent};

use super::super::{Creds, CurrentScreen};

impl super::super::App {
    pub fn main_key_handler(&mut self, key: KeyEvent) {
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
}
