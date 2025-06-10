use crate::database::model::EntryInfo;

use super::model::{Entry, Secret};
use super::schema::entries::dsl::{email, entries, id, password, sitename, siteurl, username};
use super::schema::secrets::dsl::*;
use diesel::prelude::*;

fn get_all_entry_info(conn: &mut SqliteConnection) -> QueryResult<Vec<EntryInfo>> {
    entries
        .select(EntryInfo::as_select())
        .load::<EntryInfo>(conn)
}

pub fn get_all_entries(conn: &mut SqliteConnection) -> QueryResult<Vec<Entry>> {
    let res = get_all_entry_info(conn)?;
    Ok(res.into_iter().map(Into::into).collect::<Vec<_>>())
}

pub fn get_secret(conn: &mut SqliteConnection) -> QueryResult<Option<Secret>> {
    secrets.select(Secret::as_select()).first(conn).optional()
}

pub fn get_password(conn: &mut SqliteConnection, entry_id: i32) -> QueryResult<Option<String>> {
    entries
        .filter(id.eq(entry_id))
        .select(password)
        .first::<String>(conn)
        .optional()
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
