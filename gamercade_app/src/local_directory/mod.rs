use rusqlite::Connection;

mod game;
mod tag;

use game::Game;
use tag::TagDictionary;

const GAME_DIRECTORY: &str = &"./game_directory";

pub struct LocalDirectory {
    db: Connection,
    cached_games: Vec<Game>,
    tag_dictionary: TagDictionary,
    // TODO: Add users
    // TODO: Permission Levels
    // TODO: Add Images
}

impl Default for LocalDirectory {
    fn default() -> Self {
        let db = Connection::open(GAME_DIRECTORY).unwrap();

        let output = Self {
            tag_dictionary: TagDictionary::new(&db),
            db,
            cached_games: Vec::new(),
        };

        output.refresh_cached_games();

        output
    }
}
