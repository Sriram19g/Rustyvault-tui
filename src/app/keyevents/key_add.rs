use ratatui::crossterm::event::{KeyCode, KeyEvent};

use super::super::{Creds, CurrentScreen};

impl super::super::App {
    pub fn add_key_handler(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                if let Some(adding) = &self.current_param {
                    match adding {
                        Creds::Sitename => {
                            self.current_param = Some(Creds::Siteurl);
                        }
                        Creds::Siteurl => {
                            self.current_param = Some(Creds::Gmail);
                        }
                        Creds::Gmail => {
                            self.current_param = Some(Creds::Username);
                        }
                        Creds::Username => {
                            self.current_param = Some(Creds::Password);
                        }
                        Creds::Password => {
                            self.save_credentials();
                            self.current_screen = CurrentScreen::Main;
                        }
                    }
                }
            }
            KeyCode::Backspace => {
                if let Some(adding) = &self.current_param {
                    match adding {
                        Creds::Sitename => {
                            self.site_input.pop();
                        }
                        Creds::Siteurl => {
                            self.url_input.pop();
                        }
                        Creds::Gmail => {
                            self.gmail_input.pop();
                        }
                        Creds::Username => {
                            self.user_input.pop();
                        }
                        Creds::Password => {
                            self.pass_input.pop();
                            self.masked_pass.pop();
                        }
                    }
                }
            }
            KeyCode::Esc => {
                self.current_screen = CurrentScreen::Main;
                self.current_param = None;
            }
            KeyCode::Tab => {
                self.toggle_params();
            }
            KeyCode::Char(value) => {
                if let Some(adding) = &self.current_param {
                    match adding {
                        Creds::Sitename => {
                            self.site_input.push(value);
                        }
                        Creds::Siteurl => {
                            self.url_input.push(value);
                        }
                        Creds::Gmail => {
                            self.gmail_input.push(value);
                        }
                        Creds::Username => {
                            self.user_input.push(value);
                        }
                        Creds::Password => {
                            self.pass_input.push(value);
                            self.masked_pass.push('*');
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
