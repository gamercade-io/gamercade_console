use rusqlite::types::FromSql;

use super::{Dictionary, DictionaryTrait};

#[derive(Hash, Eq, PartialEq, Debug, Default)]
pub struct TagId(pub usize);

#[derive(Default, Debug)]
pub struct Tag(pub String);

impl From<&rusqlite::Row<'_>> for Tag {
    fn from(value: &rusqlite::Row<'_>) -> Self {
        Self(value.get_unwrap(1))
    }
}

impl FromSql for TagId {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        value.as_i64().map(|num| Self(num as usize))
    }
}

impl DictionaryTrait<TagId, Tag> for Dictionary<TagId, Tag> {
    fn fetch_all_query() -> &'static str {
        "SELECT * FROM tags"
    }

    fn upsert_table_query() -> &'static str {
        "CREATE TABLE IF NOT EXISTS tags(id INTEGER PRIMARY KEY, tag_name TEXT NOT NULL) STRICT;"
    }

    fn drop_table_query() -> &'static str {
        "DROP TABLE IF EXISTS tags"
    }

    fn insert_query() -> &'static str {
        "INSERT INTO tags(id, tag_name) VALUES (?, ?)"
    }

    fn insert_statement(statement: &mut rusqlite::Statement, (key, value): &(TagId, Tag)) {
        statement.execute((key.0 as i32, value.0.clone())).unwrap();
    }
}
