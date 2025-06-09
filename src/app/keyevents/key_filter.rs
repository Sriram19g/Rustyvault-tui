use crate::app::Popup;

use super::super::{Creds, CurrentScreen};
use ratatui::crossterm::event::{KeyCode, KeyEvent};

impl super::super::App {
    pub fn filter_key_handler(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                if let Some(filter_param) = &self.current_param {
                    match filter_param {
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
                            self.filter_operation();
                            self.prev_popup = self.current_popup;
                            self.current_popup = Popup::None;
                            self.current_screen = CurrentScreen::Show;
                            self.clear_input();
                            self.current_param = Some(Creds::Sitename);
                        }
                        _ => {}
                    }
                }
            }
            KeyCode::Backspace => {
                if let Some(filter_param) = &self.current_param {
                    match filter_param {
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
                        _ => {}
                    }
                }
            }
            KeyCode::Tab => {
                self.toggle_filter_param();
            }
            KeyCode::Esc => {
                self.current_screen = CurrentScreen::Show;
                self.prev_popup = self.current_popup;
                self.current_popup = Popup::None;
                self.clear_input();
                self.current_param = None;
                self.current_param = Some(Creds::Sitename);
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
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
