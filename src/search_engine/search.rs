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
    pub fn get_phrase(&self) -> &str {
        &*self.phrase
    }

    pub fn get_folder(&self) -> &Folder {
        &self.folder
    }
}

#[derive(Clone, Debug)]
pub struct Results {
    _folder: Folder,
}

impl Results {
    pub fn new(folder: Folder) -> Self {
        Self { _folder: folder }
    }
    pub fn unwarp(&self) -> &str {
        self._folder.get_folder_location()
    }
}
