const BANNED_WORDS: [&str; 1] = [""];

pub fn check_word_allowed(text: &str) -> bool {
    BANNED_WORDS.binary_search(&text).is_ok()
}

// TODO: Benchmark binary search, hashmap, or Vec lookup for banned words
// TODO: Add the banned words list
