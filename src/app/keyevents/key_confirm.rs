use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::app::{Confirmval, CurrentScreen, Popup};

impl super::super::App {
    pub fn confirm_key_handler(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Enter => {
                if let screen = &self.current_screen {
                    match screen {
                        CurrentScreen::Main => {
                            if let Confirmval::Yes = &self.confirm_state {
                                return true;
                            } else if let Confirmval::No = &self.confirm_state {
                                self.prev_popup = self.current_popup;
                                self.current_popup = Popup::None;
                                self.confirm_state = Confirmval::Yes;
                            }
                        }
                        CurrentScreen::Add => {
                            if let Confirmval::Yes = &self.confirm_state {
                                self.save_credentials();
                                self.prev_popup = self.current_popup;
                                self.current_popup = Popup::None;
                                self.current_screen = CurrentScreen::Main;
                            } else if let Confirmval::No = &self.confirm_state {
                                self.prev_popup = self.current_popup;
                                self.current_popup = Popup::None;
                                self.current_screen = CurrentScreen::Main;
                                self.confirm_state = Confirmval::Yes;
                            }
                        }
                        CurrentScreen::Show => {
                            if let popup = &self.prev_popup {
                                match popup {
                                    Popup::None => {
                                        if let Confirmval::Yes = &self.confirm_state {
                                            self.delete_entry();
                                            self.prev_popup = self.current_popup;
                                            self.current_popup = Popup::None;
                                        } else if let Confirmval::No = &self.confirm_state {
                                            self.current_popup = self.prev_popup;
                                            self.confirm_state = Confirmval::Yes;
                                        }
                                    }
                                    Popup::Update => {
                                        if let Confirmval::Yes = &self.confirm_state {
                                            self.update_credentials();
                                            self.prev_popup = self.current_popup;
                                            self.current_popup = Popup::None;
                                            self.current_screen = CurrentScreen::Show;
                                        } else if let Confirmval::No = &self.confirm_state {
                                            self.current_popup = Popup::None;
                                            self.clear_input();
                                            self.confirm_state = Confirmval::Yes;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            KeyCode::Esc => {
                if let screen = &self.current_screen {
                    match screen {
                        CurrentScreen::Main => {
                            self.prev_popup = self.current_popup;
                            self.current_popup = Popup::None;
                            self.confirm_state = Confirmval::Yes;
                        }
                        CurrentScreen::Add => {
                            self.prev_popup = self.current_popup;
                            self.current_popup = Popup::None;
                            self.confirm_state = Confirmval::Yes;
                            self.current_screen = CurrentScreen::Add;
                        }
                        CurrentScreen::Show => {
                            if let popup = &self.prev_popup {
                                match popup {
                                    Popup::None => {
                                        self.prev_popup = self.current_popup;
                                        self.current_popup = Popup::None;
                                    }
                                    Popup::Update => {
                                        self.prev_popup = self.current_popup;
                                        self.current_popup = Popup::Update;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            KeyCode::Char('y') => {
                if let screen = &self.current_screen {
                    match screen {
                        CurrentScreen::Main => {
                            return true;
                        }
                        CurrentScreen::Add => {
                            self.save_credentials();
                            self.prev_popup = self.current_popup;
                            self.current_popup = Popup::None;
                            self.current_screen = CurrentScreen::Main;
                        }
                        CurrentScreen::Show => {
                            if let popup = &self.prev_popup {
                                match popup {
                                    Popup::None => {
                                        self.delete_entry();
                                        self.prev_popup = self.current_popup;
                                        self.current_popup = Popup::None;
                                    }
                                    Popup::Update => {
                                        self.update_credentials();
                                        self.prev_popup = self.current_popup;
                                        self.current_popup = Popup::None;
                                        self.current_screen = CurrentScreen::Show;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            KeyCode::Char('n') => {
                if let screen = &self.current_screen {
                    match screen {
                        CurrentScreen::Main => {
                            self.prev_popup = self.current_popup;
                            self.current_popup = Popup::None;
                            self.confirm_state = Confirmval::Yes;
                        }
                        CurrentScreen::Add => {
                            self.prev_popup = self.current_popup;
                            self.current_popup = Popup::None;
                            self.confirm_state = Confirmval::Yes;
                            self.current_screen = CurrentScreen::Main;
                            self.clear_input();
                        }
                        CurrentScreen::Show => {
                            if let popup = &self.prev_popup {
                                match popup {
                                    Popup::None => {
                                        self.prev_popup = self.current_popup;
                                        self.current_popup = Popup::None;
                                        self.confirm_state = Confirmval::Yes;
                                    }
                                    Popup::Update => {
                                        self.prev_popup = self.current_popup;
                                        self.current_popup = Popup::None;
                                        self.confirm_state = Confirmval::Yes;
                                        self.clear_input();
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            KeyCode::Tab => match self.confirm_state {
                Confirmval::No => self.confirm_state = Confirmval::Yes,
                Confirmval::Yes => self.confirm_state = Confirmval::No,
            },

            _ => {}
        }
        false
    }
}
