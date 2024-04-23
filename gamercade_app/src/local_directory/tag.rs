use nohash_hasher::IsEnabled;
use rusqlite::types::FromSql;

use super::{Dictionary, DictionaryTrait};

#[derive(Eq, PartialEq, Debug, Default)]
pub struct TagId(pub i32);

#[derive(Default, Debug)]
pub struct Tag(pub String);

impl IsEnabled for TagId {}

impl std::hash::Hash for TagId {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        hasher.write_i32(self.0)
    }
}

impl From<&rusqlite::Row<'_>> for Tag {
    fn from(value: &rusqlite::Row<'_>) -> Self {
        Self(value.get_unwrap(1))
    }
}

impl FromSql for TagId {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        value.as_i64().map(|num| Self(num as i32))
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
        statement.execute((key.0, value.0.clone())).unwrap();
    }
}
