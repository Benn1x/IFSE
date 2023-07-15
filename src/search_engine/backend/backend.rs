use super::super::search::{Results, Search};
use crate::file_system::folder::Folder;

pub struct Backend {
    // search_engine:
}

impl Backend {
    pub fn new() -> Self {
        Self {}
    }
    pub fn search(&self, search: Search) -> Results {
        Results::new(Folder::new(
            String::from("Dummy Search for: ") + &*search.phrase,
        ))
    }
}
