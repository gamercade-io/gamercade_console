use hashbrown::HashMap;
use rusqlite::Connection;

pub struct TagDictionary {
    map: HashMap<TagId, String>,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct TagId(pub usize);

impl TagDictionary {
    pub fn new(db: &Connection) -> Self {
        let mut map = HashMap::new();

        //TOOD: Hit the DB, get the list of tags
        //Populate the map.

        Self { map }
    }

    pub fn get(&self, key: TagId) -> Option<&str> {
        self.map.get(&key).map(|x| x.as_str())
    }
}
