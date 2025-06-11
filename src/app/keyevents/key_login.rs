use crate::app::{Confirmval, LoginPopup};

use super::super::CurrentScreen;
use ratatui::crossterm::event::{KeyCode, KeyEvent};

impl super::super::App {
    pub fn login_key_handler(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Enter => match self.confirm_state {
                Confirmval::Yes => {
                    self.check_password();
                    if self.is_login {
                        self.current_screen = CurrentScreen::Main;
                    } else {
                        if self.attempt <= 1 {
                            return false;
                        } else {
                            self.entry_key = String::new();
                            self.masked_pass = String::new();
                            self.attempt = self.attempt - 1;
                        }
                    }
                }
                Confirmval::No => {
                    self.check_password();
                    if self.is_login {
                        self.entry_key = String::new();
                        self.masked_pass = String::new();
                        self.login_state = LoginPopup::Reset;
                        self.confirm_state = Confirmval::Yes;
                    } else {
                        if self.attempt <= 1 {
                            return false;
                        } else {
                            self.entry_key = String::new();
                            self.masked_pass = String::new();
                            self.attempt = self.attempt - 1;
                        }
                    }
                }
            },
            KeyCode::Backspace => {
                self.entry_key.pop();
                self.masked_pass.pop();
            }
            KeyCode::Char(value) => {
                self.entry_key.push(value);
                self.masked_pass.push('*');
            }
            KeyCode::Esc => {
                return false;
            }
            KeyCode::Tab => match self.confirm_state {
                Confirmval::Yes => self.confirm_state = Confirmval::No,
                Confirmval::No => self.confirm_state = Confirmval::Yes,
            },
            _ => {}
        }
        true
    }
    pub fn new_key_handler(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Enter => match self.confirm_state {
                Confirmval::Yes => {
                    self.confirm_state = Confirmval::No;
                }
                Confirmval::No => {
                    if self.entry_key == self.entry_key1 {
                        self.add_masterkey();
                        self.login_state = LoginPopup::Login;
                        self.confirm_state = Confirmval::Yes;
                    }
                }
            },
            KeyCode::Backspace => match self.confirm_state {
                Confirmval::Yes => {
                    self.entry_key.pop();
                    self.masked_pass.pop();
                }
                Confirmval::No => {
                    self.entry_key1.pop();
                    self.masked_pass1.pop();
                }
            },
            KeyCode::Char(value) => match self.confirm_state {
                Confirmval::Yes => {
                    self.entry_key.push(value);
                    self.masked_pass.push('*');
                }
                Confirmval::No => {
                    self.entry_key1.push(value);
                    self.masked_pass1.push('*');
                }
            },
            KeyCode::Tab => match self.confirm_state {
                Confirmval::Yes => {
                    self.confirm_state = Confirmval::No;
                }
                Confirmval::No => {
                    self.confirm_state = Confirmval::Yes;
                }
            },

            KeyCode::Esc => {
                return false;
            }
            _ => {}
        }
        true
    }
    pub fn reset_key_handler(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter => match self.confirm_state {
                Confirmval::Yes => {
                    self.confirm_state = Confirmval::No;
                }
                Confirmval::No => {
                    if self.entry_key == self.entry_key1 {
                        self.update_all_passwords();
                        self.login_state = LoginPopup::Login;
                        self.confirm_state = Confirmval::Yes;
                    }
                }
            },
            KeyCode::Backspace => match self.confirm_state {
                Confirmval::Yes => {
                    self.entry_key.pop();
                    self.masked_pass.pop();
                }
                Confirmval::No => {
                    self.entry_key1.pop();
                    self.masked_pass1.pop();
                }
            },
            KeyCode::Char(value) => match self.confirm_state {
                Confirmval::Yes => {
                    self.entry_key.push(value);
                    self.masked_pass.push('*');
                }
                Confirmval::No => {
                    self.entry_key1.push(value);
                    self.masked_pass1.push('*');
                }
            },
            KeyCode::Tab => match self.confirm_state {
                Confirmval::Yes => {
                    self.confirm_state = Confirmval::No;
                }
                Confirmval::No => {
                    self.confirm_state = Confirmval::Yes;
                }
            },

            KeyCode::Esc => {
                self.entry_key = String::new();
                self.entry_key1 = String::new();
                self.masked_pass = String::new();
                self.masked_pass1 = String::new();
                self.login_state = LoginPopup::Login;
            }
            _ => {}
        }
    }
}
