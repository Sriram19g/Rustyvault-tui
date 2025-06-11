use super::model::{UpdateEntry, UpdateSecret};
use super::schema::entries::dsl::{entries, id, password};
use super::schema::secrets::dsl::secrets;
use diesel::prelude::*;

pub fn update_entry(
    con: &mut SqliteConnection,
    entry_id: i32,
    data: UpdateEntry,
) -> QueryResult<usize> {
    diesel::update(entries.filter(id.eq(entry_id)))
        .set(&data)
        .execute(con)
}

pub fn update_secret(con: &mut SqliteConnection, data: UpdateSecret) -> QueryResult<usize> {
    diesel::update(secrets).set(&data).execute(con)
}

pub fn update_password(
    con: &mut SqliteConnection,
    entry_id: i32,
    pass: &str,
) -> QueryResult<usize> {
    diesel::update(entries.filter(id.eq(entry_id)))
        .set(password.eq(pass))
        .execute(con)
}
