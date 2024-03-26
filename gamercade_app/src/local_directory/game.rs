use rusqlite::Connection;

use super::{tag::TagId, LocalDirectory};

pub struct Game {
    pub id: u64,
    pub title: String,
    pub short_description: String,
    pub long_description: Option<String>,
    pub releases: Vec<GameRelease>,
    pub tags: Vec<TagId>,
    pub rating: Option<f32>,
    pub images: Vec<GameImage>,
}

pub struct GameRelease {
    pub id: u64,
    pub checksum: u128,
    pub name: String,
}

pub struct GameImage {
    pub path: String,
}

const UPSERT_GAMES_QUERIES: &str = "
BEGIN;
CREATE TABLE IF NOT EXISTS games (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    short_description TEXT NOT NULL,
    long_description TEXT NOT NULL,
    rating REAL,
    UNIQUE(title)
) STRICT;

CREATE TABLE IF NOT EXISTS releases(
    id INTEGER PRIMARY KEY,
    file_checksum BLOB,
    game_id INTEGER NOT NULL,
    release_name TEXT NOT NULL,
    FOREIGN KEY (game_id) REFERENCES games (id),
    UNIQUE(game_id, file_checksum),
    UNIQUE(game_id, release_name)
) STRICT;

CREATE TABLE IF NOT EXISTS game_tags(
    id INTEGER PRIMARY KEY,
    game_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    FOREIGN KEY (game_id) REFERENCES games (id),
    FOREIGN KEY (tag_id) REFERENCES tags (pid),
    UNIQUE(game_id, tag_id)
) STRICT;
COMMIT;
";

pub(super) fn upsert_games_table(db: &Connection) {
    db.execute_batch(UPSERT_GAMES_QUERIES).unwrap();
}

impl LocalDirectory {
    pub fn refresh_cached_games(&self) {
        // TODO: Hit the DB to fetch the list of games and populate all of the fields
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
