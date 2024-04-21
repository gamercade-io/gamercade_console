use gamercade_interface::game::GameInfoBasic;
use nohash_hasher::IsEnabled;
use rusqlite::{types::FromSql, Connection};

use super::LocalDirectory;

#[derive(Eq, PartialEq, Debug, Default)]
pub struct GameId(pub i64);

impl std::hash::Hash for GameId {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        hasher.write_i64(self.0)
    }
}

impl IsEnabled for GameId {}

impl FromSql for GameId {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        value.as_i64().map(|num| Self(num as i64))
    }
}

pub struct Game {
    pub id: i64,
    pub title: String,
    pub short_description: String,
    pub long_description: Option<String>,
    pub tags: Vec<i32>,
    pub rating: f32,
    pub file_checksum: Option<i64>,
    pub rom_size: Option<i64>,
}

const UPSERT_GAMES_QUERIES: &str = "
CREATE TABLE IF NOT EXISTS games (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    short_description TEXT NOT NULL,
    long_description TEXT,
    tags BLOB,
    rating REAL,
    file_checksum INTEGER,
    rom_size INTEGER,
    UNIQUE(title)
) STRICT;
";

pub(super) fn upsert_games_table(db: &Connection) {
    db.execute_batch(UPSERT_GAMES_QUERIES).unwrap();
}

impl LocalDirectory {
    pub fn update_game(&mut self, game: GameInfoBasic) {
        let tag_bytes = game
            .tags
            .into_iter()
            .map(|tag| u8::try_from(tag).unwrap())
            .collect::<Vec<_>>();

        self.db.execute("INSERT OR REPLACE INTO games (id, title, short_description, file_checksum, rom_size, rating, tags)
        VALUES (?, ?, ?, ?, ?, ?, ?);",
        (game.game_id, game.title, game.short_description, game.checksum, game.rom_size, game.average_rating, tag_bytes)).unwrap();

        self.cache_dirty = true;
    }

    pub fn update_game_rom(&mut self, game_id: i64, checksum: i64, rom_size: i32) {
        self.db
            .execute(
                "UPDATE games SET file_checksum = ?, rom_size = ? WHERE id = ?;",
                (checksum, rom_size, game_id),
            )
            .unwrap();

        self.cache_dirty = true;
    }

    pub fn iter_games(&self) -> GameIter<'_> {
        GameIter::new(&self.cached_games)
    }

    pub fn sync_games_cache(&mut self) {
        if !self.cache_dirty {
            return;
        }

        let mut query = self.db.prepare("SELECT * FROM games;").unwrap();

        self.cached_games = query
            .query_map((), |row| {
                let tag_bytes: Vec<u8> = row.get(4)?;
                let tags = tag_bytes
                    .chunks_exact(4)
                    .map(|num| *bytemuck::from_bytes(num))
                    .collect::<Vec<i32>>();
                Ok(Game {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    short_description: row.get(2)?,
                    long_description: row.get(3)?,
                    tags,
                    rating: row.get(5)?,
                    file_checksum: row.get(6)?,
                    rom_size: row.get(7)?,
                })
            })
            .unwrap()
            .flatten()
            .collect();

        self.cache_dirty = false;
    }
}

pub struct GameIter<'a> {
    vec: &'a Vec<Game>,
    index: usize,
}

impl<'a> Iterator for GameIter<'a> {
    type Item = &'a Game;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.vec.get(self.index);
        self.index += 1;
        out
    }
}

impl<'a> GameIter<'a> {
    fn new(vec: &'a Vec<Game>) -> Self {
        Self { vec, index: 0 }
    }
}
