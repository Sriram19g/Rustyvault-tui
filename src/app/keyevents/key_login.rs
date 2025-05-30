use super::super::CurrentScreen;
use ratatui::crossterm::event::{KeyCode, KeyEvent};

impl super::super::App {
    pub fn login_key_handler(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Enter => {
                self.check_password();
                if self.is_login {
                    self.current_screen = CurrentScreen::Main;
                } else {
                    return false;
                }
            }
            KeyCode::Backspace => {
                self.entry_key.pop();
                self.masked_pass.pop();
            }
            KeyCode::Char(value) => {
                self.entry_key.push(value);
                self.masked_pass.push('*');
            }
            _ => {}
        }
        true
    }
}
