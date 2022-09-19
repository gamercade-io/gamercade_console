use std::path::PathBuf;

pub(crate) trait Watchable {
    fn get_watch_list(&self) -> Vec<PathBuf>;
    fn watchable(&self) -> bool;
}
