use super::model::{NewEntry, NewSecret};
use super::schema::{entries, secrets};
use diesel::prelude::*;

pub fn add_entry(conn: &mut SqliteConnection, entry: NewEntry) -> QueryResult<usize> {
    diesel::insert_into(entries::table)
        .values(&entry)
        .execute(conn)
}

pub fn add_secret(conn: &mut SqliteConnection, secret: NewSecret) -> QueryResult<usize> {
    diesel::insert_into(secrets::table)
        .values(&secret)
        .execute(conn)
}
