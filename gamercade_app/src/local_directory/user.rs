use nohash_hasher::IsEnabled;
use rusqlite::{types::FromSql, ToSql};

use super::{Dictionary, DictionaryTrait};

#[derive(Eq, PartialEq, Debug, Default)]
pub struct UserId(pub i64);

impl std::hash::Hash for UserId {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        hasher.write_i64(self.0)
    }
}

impl IsEnabled for UserId {}

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
        value.as_i64().map(Self)
    }
}

impl ToSql for User {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        todo!()
    }
}

impl DictionaryTrait<UserId, User> for Dictionary<UserId, User> {
    fn fetch_all_query() -> &'static str {
        "SELECT * FROM users"
    }

    fn upsert_table_query() -> &'static str {
        "CREATE TABLE IF NOT EXISTS users(id INTEGER PRIMARY KEY, username TEXT NOT NULL, avatar_last_updated INTEGER) STRICT;"
    }

    fn drop_table_query() -> &'static str {
        "DROP TABLE IF EXISTS users"
    }

    fn insert_query() -> &'static str {
        "INSERT INTO permission_levels(id, username, avarar_last_updated) VALUES (?, ?)"
    }

    fn insert_statement(statement: &mut rusqlite::Statement, (key, user): &(UserId, User)) {
        statement
            .execute((
                key.0 as i32,
                user.username.clone(),
                user.avatar_last_updated,
            ))
            .unwrap();
    }
}
