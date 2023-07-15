use crate::file_system::folder::Folder;

pub struct Search {
    pub phrase: String,
    pub folder: Folder,
    pub results: Option<Results>,
}

impl Search {
    pub fn new(phrase: String, folder: Folder) -> Self {
        Self {
            phrase,
            folder,
            results: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Results {
    folder: Folder,
}

impl Results {
    pub fn new(folder: Folder) -> Self {
        Self { folder }
    }
}
