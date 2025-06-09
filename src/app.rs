use diesel::SqliteConnection;
use ratatui::widgets::{ScrollbarState, TableState};

mod keyevents;
mod showtable;
use crate::database::{
    self,
    add::add_entry,
    model::{Entry, NewEntry, UpdateEntry},
    retrive::get_filtered_entries,
    update::update_entry,
};

const ITEM_HEIGHT: usize = 10;

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
    pub site_input: String,
    pub url_input: String,
    pub gmail_input: String,
    pub user_input: String,
    pub pass_input: String,
    pub masked_pass: String,
    pub is_login: bool,
    pub current_screen: CurrentScreen,
    pub current_param: Option<Creds>,
    pub credentials: Vec<Entry>,
    pub scroll_state: ScrollbarState,
    pub current_popup: Popup,
    pub confirm_state: Confirmval,
    pub prev_popup: Popup,
    pub conn: SqliteConnection,
}

impl App {
    pub fn new() -> App {
        App {
            state: TableState::default().with_selected(0),
            entry_key: String::new(),
            site_input: String::new(),
            url_input: String::new(),
            gmail_input: String::new(),
            user_input: String::new(),
            pass_input: String::new(),
            masked_pass: String::new(),
            is_login: false,
            current_screen: CurrentScreen::Login,
            current_popup: Popup::None,
            prev_popup: Popup::None,
            current_param: None,
            credentials: Vec::new(),
            scroll_state: ScrollbarState::new(0),
            confirm_state: Confirmval::Yes,
            conn: database::db::establish_connection(),
        }
    }
    pub fn save_credentials(&mut self) {
        let newentry = NewEntry {
            sitename: &self.site_input,
            siteurl: &self.url_input,
            email: Some(&self.gmail_input),
            username: Some(&self.user_input),
            password: &self.pass_input,
        };

        let _ = add_entry(&mut self.conn, newentry);

        self.clear_input();
    }

    pub fn update_credentials(&mut self) {
        if let Some(index) = self.state.selected() {
            self.credentials[index].sitename = self.site_input.clone();
            self.credentials[index].siteurl = self.url_input.clone();
            self.credentials[index].email = self.gmail_input.clone();
            self.credentials[index].username = self.user_input.clone();
            self.credentials[index].password = self.pass_input.clone();

            let id = self.credentials[index].id;

            let data = UpdateEntry {
                sitename: Some(&self.site_input),
                siteurl: Some(&self.url_input),
                email: Some(&self.gmail_input),
                username: Some(&self.user_input),
                password: Some(&self.pass_input),
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

    pub fn check_password(&mut self) {
        let pass = String::from("abcd1234");
        if pass.trim() == self.entry_key.trim() {
            self.is_login = true;
        } else {
            self.is_login = false;
        }
    }
    pub fn toggle_params(&mut self) {
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
    pub fn toggle_filter_param(&mut self) {
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
}
