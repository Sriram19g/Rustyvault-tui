use diesel::SqliteConnection;
use ratatui::widgets::{ScrollbarState, TableState};

mod keyevents;
mod showtable;
use crate::{
    database::{
        self,
        add::{add_entry, add_secret},
        model::{Entry, NewEntry, NewSecret, UpdateEntry, UpdateSecret},
        retrive::{get_all_passwords, get_filtered_entries, get_secret},
        update::{update_entry, update_password, update_secret},
    },
    security::{
        generator::generate_device_secret,
        hashing::{generate_hash, generate_key_derivation, verify_password},
        xchacha::{decrypt, encrypt},
    },
};

const ITEM_HEIGHT: usize = 10;

pub enum LoginPopup {
    Login,
    Reset,
    New,
}

pub enum CurrentScreen {
    Login,
    Main,
    Show,
    Add,
}

#[derive(Clone, Copy)]
pub enum Popup {
    None,
    Update,
    Confirm,
    Filter,
}
pub enum Confirmval {
    Yes,
    No,
}

pub enum Creds {
    Sitename,
    Siteurl,
    Gmail,
    Username,
    Password,
}

pub struct App {
    pub state: TableState,
    pub entry_key: String,
    pub entry_key1: String,
    pub site_input: String,
    pub url_input: String,
    pub gmail_input: String,
    pub user_input: String,
    pub pass_input: String,
    pub masked_pass: String,
    pub masked_pass1: String,
    pub is_login: bool,
    pub current_screen: CurrentScreen,
    pub current_param: Option<Creds>,
    pub credentials: Vec<Entry>,
    pub scroll_state: ScrollbarState,
    pub current_popup: Popup,
    pub confirm_state: Confirmval,
    pub prev_popup: Popup,
    pub conn: SqliteConnection,
    pub key: [u8; 32],
    pub login_state: LoginPopup,
    pub attempt: u8,
}

impl App {
    pub fn new() -> App {
        App {
            state: TableState::default().with_selected(0),
            entry_key: String::new(),
            entry_key1: String::new(),
            site_input: String::new(),
            url_input: String::new(),
            gmail_input: String::new(),
            user_input: String::new(),
            pass_input: String::new(),
            masked_pass: String::new(),
            masked_pass1: String::new(),
            is_login: false,
            current_screen: CurrentScreen::Login,
            current_popup: Popup::None,
            prev_popup: Popup::None,
            current_param: None,
            credentials: Vec::new(),
            scroll_state: ScrollbarState::new(0),
            confirm_state: Confirmval::Yes,
            conn: database::db::establish_connection(),
            key: [0u8; 32],
            login_state: LoginPopup::Login,
            attempt: 3,
        }
    }
    fn save_credentials(&mut self) {
        let newentry = NewEntry {
            sitename: &self.site_input,
            siteurl: &self.url_input,
            email: Some(&self.gmail_input),
            username: Some(&self.user_input),
            password: &encrypt(&self.pass_input, &self.key),
        };

        let _ = add_entry(&mut self.conn, newentry);

        self.clear_input();
    }

    fn update_credentials(&mut self) {
        if let Some(index) = self.state.selected() {
            self.credentials[index].sitename = self.site_input.clone();
            self.credentials[index].siteurl = self.url_input.clone();
            self.credentials[index].email = self.gmail_input.clone();
            self.credentials[index].username = self.user_input.clone();
            self.credentials[index].password = "*****".to_owned();

            let id = self.credentials[index].id;

            let data = UpdateEntry {
                sitename: Some(&self.site_input),
                siteurl: Some(&self.url_input),
                email: Some(&self.gmail_input),
                username: Some(&self.user_input),
                password: match self.pass_input.as_ref() {
                    "*****" => None,
                    val => {
                        self.pass_input = encrypt(val, &self.key);
                        Some(&self.pass_input)
                    }
                },
            };

            let _ = update_entry(&mut self.conn, id, data);

            self.clear_input();
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

    fn filter_operation(&mut self) {
        let filter_creds = get_filtered_entries(
            &mut self.conn,
            self.site_input.clone(),
            self.url_input.clone(),
            self.gmail_input.clone(),
            self.user_input.clone(),
        );
        self.credentials = match filter_creds {
            Ok(data) => data,
            Err(_) => Vec::new(),
        };
        self.clear_input();
    }

    fn check_password(&mut self) {
        match get_secret(&mut self.conn) {
            Ok(Some(secret)) => {
                let mk_hash = secret.masterkey_hash;
                let ds = secret.device_secret;
                self.is_login = verify_password(&mk_hash, &ds, &self.entry_key.trim().to_owned());
                self.key = generate_key_derivation(&self.entry_key.to_owned(), &ds);
                self.masked_pass = String::new();
                self.entry_key = String::new();
            }
            Ok(None) => {
                // No secret found in DB
                self.is_login = false;
            }
            Err(_) => {
                // DB error occurred
                self.is_login = false;
            }
        }
    }
    fn toggle_params(&mut self) {
        if let Some(current_param) = &self.current_param {
            match current_param {
                Creds::Sitename => self.current_param = Some(Creds::Siteurl),
                Creds::Siteurl => self.current_param = Some(Creds::Gmail),
                Creds::Gmail => self.current_param = Some(Creds::Username),
                Creds::Username => self.current_param = Some(Creds::Password),
                Creds::Password => self.current_param = Some(Creds::Sitename),
            };
        } else {
            self.current_param = Some(Creds::Sitename);
        }
    }
    fn toggle_filter_param(&mut self) {
        if let Some(current_param) = &self.current_param {
            match current_param {
                Creds::Sitename => self.current_param = Some(Creds::Siteurl),
                Creds::Siteurl => self.current_param = Some(Creds::Gmail),
                Creds::Gmail => self.current_param = Some(Creds::Username),
                Creds::Username => self.current_param = Some(Creds::Sitename),
                _ => {}
            };
        } else {
            self.current_param = Some(Creds::Sitename);
        }
    }
    fn add_masterkey(&mut self) {
        let ds = generate_device_secret();
        let data = NewSecret {
            masterkey_hash: &generate_hash(&self.entry_key, &ds),
            device_secret: &ds,
        };
        let _ = add_secret(&mut self.conn, data);
        self.entry_key = String::new();
        self.masked_pass = String::new();
    }

    fn update_all_passwords(&mut self) {
        let ds = generate_device_secret();
        let mk_hash = generate_hash(&self.entry_key, &ds);
        let data = UpdateSecret {
            masterkey_hash: Some(&mk_hash),
            device_secret: Some(&ds),
        };
        let _ = update_secret(&mut self.conn, data);
        let new_key_derivation = generate_key_derivation(&self.entry_key, &ds);
        let data = get_all_passwords(&mut self.conn).unwrap();

        for (entry_id, pass) in data {
            let plain_text = decrypt(&pass, &self.key);
            let cipher_text = encrypt(&plain_text, &new_key_derivation);
            let _ = update_password(&mut self.conn, entry_id, &cipher_text);
        }
        self.entry_key = String::new();
        self.masked_pass = String::new();
        self.entry_key1 = String::new();
        self.masked_pass1 = String::new();
    }
}
