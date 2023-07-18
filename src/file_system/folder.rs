use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Folder {
    location: String,
    hash: u64,
}

impl Folder {
    pub fn new(location: String) -> Self {
        let mut hasher = DefaultHasher::new();
        location.hash(&mut hasher);
        Self {
            location,
            hash: hasher.finish(),
        }
    }

    pub fn get_hash(&self) -> u64 {
        self.hash
    }

    pub fn get_folder_location(&self) -> &str {
        &self.location
    }

    pub fn as_path(&self) -> &Path {
        Path::new(&self.location)
    }
}
