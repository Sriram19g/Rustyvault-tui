use super::model::UpdateEntry;
use super::schema::entries::dsl::{entries, id};
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
