use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::app::CurrentScreen;

impl super::super::App {
    pub fn show_key_handler(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.current_screen = CurrentScreen::Main,
            KeyCode::Char('j') | KeyCode::Down => self.next_row(),
            KeyCode::Char('k') | KeyCode::Up => self.previous_row(),
            KeyCode::Char('c') => {}
            KeyCode::Char('d') => self.delete_entry(),
            KeyCode::Char('u') => {}
            _ => {}
        }
    }
}
