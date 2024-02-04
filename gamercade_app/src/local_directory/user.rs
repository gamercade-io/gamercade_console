use rusqlite::types::FromSql;

use super::{Dictionary, DictionaryTrait};

#[derive(Hash, Eq, PartialEq, Debug, Default)]
pub struct UserId(pub u64);

#[derive(Default, Debug)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub avatar_last_updated: Option<u64>,
}

impl From<&rusqlite::Row<'_>> for User {
    fn from(value: &rusqlite::Row<'_>) -> Self {
        Self {
            id: value.get::<usize, i64>(0).unwrap() as u64,
            username: value.get(1).unwrap(),
            avatar_last_updated: value.get(2).unwrap(),
        }
    }
}

impl FromSql for UserId {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        value.as_i64().map(|num| Self(num as u64))
    }
}

impl DictionaryTrait<UserId, User> for Dictionary<UserId, User> {
    fn fetch_all_query() -> &'static str {
        "SELECT * FROM users"
    }

    fn upsert_table_query() -> &'static str {
        "CREATE TABLE IF NOT EXISTS users(id INT PRIMARY KEY, username STRING NOT NULL, avatar_last_updated INT) STRICT;"
    }

    fn drop_table_query() -> &'static str {
        "DROP TABLE users"
    }
}
