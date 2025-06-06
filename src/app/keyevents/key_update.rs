use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::app::Popup;

use super::super::{Creds, CurrentScreen};

impl super::super::App {
    pub fn update_key_handler(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                if let Some(updating) = &self.current_param {
                    match updating {
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
                            self.update_credentials();
                            self.current_popup = Popup::None;
                            self.current_screen = CurrentScreen::Show;
                        }
                    }
                }
            }
            KeyCode::Backspace => {
                if let Some(updating) = &self.current_param {
                    match updating {
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
                            self.masked_pass.pop();
                            self.pass_input.pop();
                        }
                    }
                }
            }
            KeyCode::Esc => {
                self.current_screen = CurrentScreen::Show;
                self.current_param = None;
                self.current_popup = Popup::None;
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
                            self.masked_pass.push('*');
                            self.pass_input.push(value);
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
