use crate::file_system::folder::Folder;
#[derive(Clone, Debug)]
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
    result: Vec<(Folder, u64)>,
}

impl Results {
    pub fn new(result: Vec<(Folder, u64)>) -> Self {
        Self { result }
    }
    pub fn unwrap(&self) -> &Vec<(Folder, u64)> {
        &self.result
    }
    pub fn size(&self) -> usize {
        self.result.len()
    }
}

#[derive(Clone, Debug)]
pub enum SearchRes {
    Success((Vec<(Folder, u64)>, u64)),
    GlobalSuccess((Results, u64, u64)),
    Failure,
    NotFound(u64),
}
