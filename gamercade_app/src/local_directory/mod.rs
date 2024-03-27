use std::hash::Hash;

use hashbrown::HashMap;
use rusqlite::{types::FromSql, Connection, Row, Statement};

mod game;
mod permission_level;
mod tag;
mod user;

use game::Game;

pub use permission_level::{PermissionLevelId, PermissionLevelName};
pub use tag::{Tag, TagId};
pub use user::{User, UserId};

use self::game::upsert_games_table;

const LOCAL_DB_PATH: &str = "./local.db";

type TagDictionary = Dictionary<TagId, Tag>;
type UserDictionary = Dictionary<UserId, User>;
type PermissionLevelDictionary = Dictionary<PermissionLevelId, PermissionLevelName>;

pub struct LocalDirectory {
    db: Connection,
    pub cached_games: Vec<Game>,
    pub tags: TagDictionary,
    pub users: UserDictionary,
    pub permission_levels: PermissionLevelDictionary,
    // TODO: Add Images
}

#[derive(Default)]
pub struct Dictionary<Key, Value> {
    map: HashMap<Key, Value>,
}

impl LocalDirectory {
    pub fn upsert_tags(&mut self, tags: &[(TagId, Tag)], clear_db: bool) {
        if clear_db {
            self.db
                .execute(TagDictionary::drop_table_query(), ())
                .unwrap();
            self.db
                .execute(TagDictionary::upsert_table_query(), ())
                .unwrap();
        }
        self.tags.bulk_insert(&mut self.db, tags);
        self.tags.sync(&self.db);
    }

    pub fn upsert_permission_levesl(
        &mut self,
        permission_levels: &[(PermissionLevelId, PermissionLevelName)],
        clear_db: bool,
    ) {
        if clear_db {
            self.db
                .execute(PermissionLevelDictionary::drop_table_query(), ())
                .unwrap();
            self.db
                .execute(PermissionLevelDictionary::upsert_table_query(), ())
                .unwrap();
        }
        self.permission_levels
            .bulk_insert(&mut self.db, permission_levels);
        self.permission_levels.sync(&self.db);
    }

    pub fn upsert_users(&mut self, users: &[(UserId, User)], clear_db: bool) {
        if clear_db {
            self.db
                .execute(UserDictionary::drop_table_query(), ())
                .unwrap();
            self.db
                .execute(UserDictionary::upsert_table_query(), ())
                .unwrap();
        }

        self.users.bulk_insert(&mut self.db, users);
        self.users.sync(&self.db);
    }
}

trait DictionaryTrait<K, V> {
    fn new(db: &Connection) -> Self
    where
        Self: Sized + Default + IsDictionary<K, V>,
        K: Hash + Eq + FromSql,
        V: for<'a> From<&'a Row<'a>>,
    {
        let mut output = Self::default();

        output.sync(db);

        output
    }

    fn sync(&mut self, db: &Connection)
    where
        Self: IsDictionary<K, V>,
        K: Hash + Eq + FromSql,
        V: for<'a> From<&'a Row<'a>>,
    {
        self.get_map_mut().clear();

        db.execute(Self::upsert_table_query(), ()).unwrap();

        let mut query = db.prepare(Self::fetch_all_query()).unwrap();
        let mut results = query.query([]).unwrap();

        while let Ok(Some(row)) = results.next() {
            let key = row.get(0).unwrap();
            let value = V::from(row);
            self.get_map_mut().insert(key, value);
        }
    }

    fn bulk_insert(&self, db: &mut rusqlite::Connection, values: &[(K, V)]) {
        let mut tx = db.transaction().unwrap();
        tx.set_drop_behavior(rusqlite::DropBehavior::Commit);

        let mut statement = tx.prepare(Self::insert_query()).unwrap();

        for kv in values.iter() {
            Self::insert_statement(&mut statement, kv);
        }
    }

    fn fetch_all_query() -> &'static str;
    fn upsert_table_query() -> &'static str;
    fn drop_table_query() -> &'static str;
    fn insert_query() -> &'static str;
    fn insert_statement(statement: &mut Statement, kv: &(K, V));
}

pub trait IsDictionary<K, V> {
    fn get_map(&self) -> &HashMap<K, V>;
    fn get_map_mut(&mut self) -> &mut HashMap<K, V>;
}

impl<K, V> IsDictionary<K, V> for Dictionary<K, V> {
    fn get_map_mut(&mut self) -> &mut HashMap<K, V> {
        &mut self.map
    }

    fn get_map(&self) -> &HashMap<K, V> {
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

        upsert_games_table(&output.db);

        output.refresh_cached_games();

        output
    }
}
