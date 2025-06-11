use crate::database::schema::entries::dsl::*;

use diesel::prelude::*;

pub fn delete_entry(con: &mut SqliteConnection, entry_id: i32) -> QueryResult<usize> {
    diesel::delete(entries.filter(id.eq(entry_id))).execute(con)
}

pub fn delete_all_entries(con: &mut SqliteConnection) -> QueryResult<usize> {
    diesel::delete(entries).execute(con)
}
