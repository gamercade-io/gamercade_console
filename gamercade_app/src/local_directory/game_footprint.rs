use gamercade_interface::platform::{EditableGamesResponse, VotedGamesResponse};
use rusqlite::types::FromSql;

use super::{game::GameId, Dictionary, DictionaryTrait, LocalDirectory};

#[derive(Default)]
pub struct GameFootprint {
    pub permission_level: Option<i32>,
    pub vote: Option<Vote>,
}

pub enum Vote {
    Up,
    Down,
}

impl FromSql for Vote {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        value.as_i64().map(|num| match num {
            0 => Vote::Down,
            1 => Vote::Up,
            _ => unreachable!(),
        })
    }
}

impl From<&rusqlite::Row<'_>> for GameFootprint {
    fn from(value: &rusqlite::Row<'_>) -> Self {
        let permission_level = value.get(0).unwrap();
        let vote: Option<Vote> = value.get(1).unwrap();

        Self {
            permission_level,
            vote,
        }
    }
}

impl DictionaryTrait<GameId, GameFootprint> for Dictionary<GameId, GameFootprint> {
    fn fetch_all_query() -> &'static str {
        "SELECT * FROM game_footprint"
    }

    fn upsert_table_query() -> &'static str {
        "CREATE TABLE IF NOT EXISTS game_footprint(game_id INTEGER PRIMARY KEY, permission_level INTEGER, vote INTEGER) STRICT;"
    }

    fn drop_table_query() -> &'static str {
        "DROP TABLE IF EXISTS game_footprint"
    }

    fn insert_query() -> &'static str {
        "INSERT INTO game_footprint(game_id, permission_level, vote) VALUES (?, ?, ?)"
    }

    fn insert_statement(
        statement: &mut rusqlite::Statement,
        (key, value): &(GameId, GameFootprint),
    ) {
        let vote = value.vote.as_ref().map(|vote| match vote {
            Vote::Up => 1,
            Vote::Down => 0,
        });
        statement
            .execute((key.0 as i32, value.permission_level, vote))
            .unwrap();
    }
}

impl LocalDirectory {
    pub fn handle_voted_games_response(&mut self, response: VotedGamesResponse) {
        let query = "INSERT OR REPLACE INTO game_footprint(vote) VALUES ? WHERE game_id = ?;";

        for game in response.voted_games {
            self.db
                .execute(query, (game.vote_value as i32, game.game_id))
                .unwrap();
        }

        self.game_footprint.sync(&self.db);
    }

    pub fn handle_editable_games_response(&mut self, response: EditableGamesResponse) {
        let query =
            "INSERT OR REPLACE INTO game_footprint(permission_level) VALUES ? WHERE game_id = ?;";

        for game in response.editable_games {
            self.db
                .execute(query, (game.permission_level as i32, game.game_id))
                .unwrap();
        }

        self.game_footprint.sync(&self.db);
    }
}
