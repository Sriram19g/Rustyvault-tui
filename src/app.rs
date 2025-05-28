pub enum CurrentScreen {
    Login,
    Main,
    Show,
    Add,
    Update,
    Filter,
    Confirm,
    Exit,
}

pub enum Creds {
    Sitename,
    Siteurl,
    Gmail,
    Username,
    Password,
}

pub struct Credential {
    pub site_name: String,
    pub url: String,
    pub password: String,
    pub username: String,
    pub gmail: String,
}

pub struct App {
    pub entry_key: String,
    pub site_input: String,
    pub url_input: String,
    pub gmail_input: String,
    pub user_input: String,
    pub pass_input: String,
    pub is_login: bool,
    pub current_screen: CurrentScreen,
    pub current_param: Option<Creds>,
    pub credentials: Vec<Credential>,
}

impl App {
    pub fn new() -> App {
        App {
            entry_key: String::new(),
            site_input: String::new(),
            url_input: String::new(),
            gmail_input: String::new(),
            user_input: String::new(),
            pass_input: String::new(),
            is_login: false,
            current_screen: CurrentScreen::Login,
            current_param: None,
            credentials: Vec::new(),
        }
    }
    pub fn save_credentials(&mut self) {
        self.credentials.push(Credential {
            site_name: self.site_input.clone(),
            url: self.url_input.clone(),
            password: self.pass_input.clone(),
            username: self.user_input.clone(),
            gmail: self.gmail_input.clone(),
        });
        self.site_input = String::new();
        self.url_input = String::new();
        self.pass_input = String::new();
        self.user_input = String::new();
        self.gmail_input = String::new();
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
