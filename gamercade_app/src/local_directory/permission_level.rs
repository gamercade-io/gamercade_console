use rusqlite::types::FromSql;

use super::{Dictionary, DictionaryTrait};

#[derive(Hash, Eq, PartialEq, Debug, Default)]
pub struct PermissionLevelId(pub usize);

#[derive(Default, Debug)]
pub struct PermissionLevelName(pub String);

impl From<&rusqlite::Row<'_>> for PermissionLevelName {
    fn from(value: &rusqlite::Row<'_>) -> Self {
        Self(value.get_unwrap(1))
    }
}

impl FromSql for PermissionLevelId {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        value.as_i64().map(|num| Self(num as usize))
    }
}

impl DictionaryTrait<PermissionLevelId, PermissionLevelName>
    for Dictionary<PermissionLevelId, PermissionLevelName>
{
    fn fetch_all_query() -> &'static str {
        "SELECT * FROM permission_levels;"
    }

    fn upsert_table_query() -> &'static str {
        "CREATE TABLE IF NOT EXISTS permission_levels(id INTEGER PRIMARY KEY, level_name STRING NOT NULL);"
    }

    fn drop_table_query() -> &'static str {
        "DROP TABLE permission_levels;"
    }
}
