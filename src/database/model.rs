use super::schema::{entries, secrets};
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = entries)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Entry {
    pub id: i32,
    pub sitename: String,
    pub siteurl: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = entries)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct EntryInfo {
    pub id: i32,
    pub sitename: String,
    pub siteurl: String,
    pub email: String,
    pub username: String,
}

impl Into<Entry> for EntryInfo {
    fn into(self) -> Entry {
        Entry {
            id: self.id,
            sitename: self.sitename,
            siteurl: self.siteurl,
            email: self.email,
            username: self.username,
            password: String::from("*****"),
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name=entries)]
pub struct NewEntry<'a> {
    pub sitename: &'a str,
    pub siteurl: &'a str,
    pub email: Option<&'a str>,
    pub username: Option<&'a str>,
    pub password: &'a str,
}
#[derive(Queryable, Debug, Selectable)]
pub struct Secret {
    pub masterkey_hash: String,
    pub device_secret: String,
}

#[derive(Insertable)]
#[diesel(table_name = secrets)]
pub struct NewSecret<'a> {
    pub masterkey_hash: &'a str,
    pub device_secret: &'a str,
}

#[derive(AsChangeset)]
#[diesel(table_name = entries)]
pub struct UpdateEntry<'a> {
    pub sitename: Option<&'a str>,
    pub siteurl: Option<&'a str>,
    pub email: Option<&'a str>,
    pub username: Option<&'a str>,
    pub password: Option<&'a str>,
}

impl Entry {
    pub fn ref_array(&self) -> [&str; 5] {
        [
            &self.sitename,
            &self.siteurl,
            &self.email,
            &self.username,
            &self.password,
        ]
    }
    pub fn site_name(&self) -> &str {
        &self.sitename
    }
    pub fn url(&self) -> &str {
        &self.siteurl
    }
    pub fn gmail(&self) -> &str {
        &self.email
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn password(&self) -> &str {
        &self.password
    }
}
