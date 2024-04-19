use gamercade_interface::game::GameInfoBasic;
use rusqlite::Connection;

use super::LocalDirectory;

pub struct Game {
    pub id: i64,
    pub title: String,
    pub short_description: String,
    pub long_description: Option<String>,
    pub tags: Vec<i32>,
    pub rating: f32,
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
    UNIQUE(title)
) STRICT;
";

pub(super) fn upsert_games_table(db: &Connection) {
    db.execute_batch(UPSERT_GAMES_QUERIES).unwrap();
}

impl LocalDirectory {
    pub fn update_game(&self, game: GameInfoBasic) {
        let tag_bytes = game
            .tags
            .into_iter()
            .map(|tag| u8::try_from(tag).unwrap())
            .collect::<Vec<_>>();
        
        self.db.execute("INSERT OR REPLACE INTO games (id, title, short_description, file_checksum, rating, tags)
        VALUES (?, ?, ?, ?, ?, ?, ?)",
        (game.game_id, game.title, game.short_description, game.checksum, game.average_rating, tag_bytes)).unwrap();
    }

    pub fn iter_games(&self) -> GameIter<'_> {
        GameIter::new(&self.cached_games)
    }
}

pub struct GameIter<'a> {
    vec: &'a Vec<Game>,
    index: usize,
}

impl<'a> Iterator for GameIter<'a> {
    type Item = &'a Game;

    fn next(&mut self) -> Option<Self::Item> {
        self.vec.get(self.index)
    }
}

impl<'a> GameIter<'a> {
    fn new(vec: &'a Vec<Game>) -> Self {
        Self { vec, index: 0 }
    }
}
