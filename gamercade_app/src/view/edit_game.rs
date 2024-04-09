pub struct EditGameView {
    game_id: Option<u64>,
    title: String,
    short_description: String,
    long_description: String,
}

impl EditGameView {
    pub fn new(
        existing_game_id: Option<u64>,
        title: Option<String>,
        short_description: Option<String>,
        long_description: Option<String>,
    ) -> Self {
        Self {
            game_id: existing_game_id,
            title: title.unwrap_or_default(),
            short_description: short_description.unwrap_or_default(),
            long_description: long_description.unwrap_or_default(),
        }
    }
}
