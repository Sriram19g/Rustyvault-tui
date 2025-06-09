use super::model::{Entry, Secret};
use super::schema::{entries::dsl::*, secrets::dsl::*};
use diesel::prelude::*;

pub fn get_all_entries(conn: &mut SqliteConnection) -> QueryResult<Vec<Entry>> {
    entries.select(Entry::as_select()).load(conn)
}

pub fn get_secret(conn: &mut SqliteConnection) -> QueryResult<Option<Secret>> {
    secrets.select(Secret::as_select()).first(conn).optional()
}

pub fn get_filtered_entries(
    conn: &mut SqliteConnection,
    site: String,
    url: String,
    gmail: String,
    user: String,
) -> QueryResult<Vec<Entry>> {
    let mut query = entries.into_boxed();
    if !site.is_empty() {
        query = query.filter(sitename.like(format!("%{}%", site)));
    }

    if !url.is_empty() {
        query = query.filter(siteurl.like(format!("%{}%", url)));
    }

    if !gmail.is_empty() {
        query = query.filter(email.like(format!("%{}%", gmail)));
    }

    if !user.is_empty() {
        query = query.filter(username.like(format!("%{}%", user)));
    }
    query.load::<Entry>(conn)
}
