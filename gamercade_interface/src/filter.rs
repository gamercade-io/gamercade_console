const BANNED_WORDS: [&str; 0] = [];

/// Checks if the passed in text should be filtered
/// from the banned words list.
pub fn check_single_word_banned(text: &str) -> bool {
    BANNED_WORDS.binary_search(&text).is_ok()
}

/// Checks if the passed in string contains any banned
/// words and should be filtered.
pub fn check_string_contains_banned_text(text: &str) -> bool {
    for word in BANNED_WORDS.iter() {
        if text.contains(word) {
            return true;
        }
    }

    false
}

// TODO: Benchmark binary search, hashmap, or Vec lookup for banned words
// TODO: Add the banned words list
