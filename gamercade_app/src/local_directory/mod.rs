use std::hash::Hash;

use hashbrown::HashMap;
use rusqlite::{types::FromSql, Connection, Row};

mod game;
mod permission_level;
mod tag;
mod user;

use game::Game;

use self::{
    permission_level::{PermissionLevelId, PermissionLevelName},
    tag::{Tag, TagId},
    user::{User, UserId},
};

const LOCAL_DB_PATH: &str = "./local.db";

type TagDictionary = Dictionary<TagId, Tag>;
type UserDictionary = Dictionary<UserId, User>;
type PermissionLevelDictionary = Dictionary<PermissionLevelId, PermissionLevelName>;

pub struct LocalDirectory {
    db: Connection,
    cached_games: Vec<Game>,
    tags: TagDictionary,
    users: UserDictionary,
    permission_levels: PermissionLevelDictionary,
    // TODO: Add Images
}

#[derive(Default)]
struct Dictionary<Key, Value> {
    map: HashMap<Key, Value>,
}

trait DictionaryTrait<K, V> {
    fn new(db: &Connection) -> Self
    where
        Self: Sized + Default + IsDictionary<K, V>,
        K: Hash + Eq + FromSql,
        V: for<'a> From<&'a Row<'a>>,
    {
        let mut output = Self::default();

        db.execute(Self::upsert_table_query(), []).unwrap();

        let mut query = db.prepare(Self::fetch_all_query()).unwrap();
        let mut results = query.query([]).unwrap();

        while let Ok(Some(row)) = results.next() {
            let key = row.get(0).unwrap();
            let value = V::from(row);
            output.map_mut().insert(key, value);
        }

        output
    }

    fn fetch_all_query() -> &'static str;
    fn upsert_table_query() -> &'static str;
    fn drop_table_query() -> &'static str;
}

trait IsDictionary<K, V> {
    fn map(&self) -> &HashMap<K, V>;
    fn map_mut(&mut self) -> &mut HashMap<K, V>;
}

impl<K, V> IsDictionary<K, V> for Dictionary<K, V> {
    fn map_mut(&mut self) -> &mut HashMap<K, V> {
        &mut self.map
    }

    fn map(&self) -> &HashMap<K, V> {
        &self.map
    }
}

impl Default for LocalDirectory {
    fn default() -> Self {
        let db = Connection::open(LOCAL_DB_PATH).unwrap();

        let output = Self {
            tags: TagDictionary::new(&db),
            users: UserDictionary::new(&db),
            permission_levels: PermissionLevelDictionary::new(&db),
            db,
            cached_games: Vec::new(),
        };

        output.refresh_cached_games();

        output
    }
}
