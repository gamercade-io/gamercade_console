use super::{tag::TagId, LocalDirectory};

pub struct Game {
    pub id: u64,
    pub title: String,
    pub short_description: String,
    pub long_description: Option<String>,
    pub releases: Vec<GameRelease>,
    pub tags: Vec<TagId>,
    pub rating: Option<f32>,
}

pub struct GameRelease {
    pub id: u64,
    pub checksum: u128,
    pub name: String,
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
