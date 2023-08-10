use std::mem;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Search {
    pub phrase: String,
    pub folder: Box<PathBuf>,
    pub results: Option<Results>,
}

impl Search {
    pub fn new(phrase: String, folder: Box<PathBuf>) -> Self {
        Self {
            phrase,
            folder,
            results: None,
        }
    }
    pub fn get_phrase(&self) -> &str {
        &*self.phrase
    }

    pub fn get_folder(&self) -> &Box<PathBuf> {
        &self.folder
    }
}

#[derive(Clone, Debug)]
pub struct Results {
    result: Vec<(Box<PathBuf>, u64)>,
}

impl Results {
    pub fn new(result: Vec<(Box<PathBuf>, u64)>) -> Self {
        Self { result }
    }
    pub fn unwrap(&self) -> &Vec<(Box<PathBuf>, u64)> {
        &self.result
    }
    pub fn size(&self) -> usize {
        self.result.len()
    }
}

#[derive(Clone, Debug)]
pub enum SearchRes {
    Success((Vec<(Box<PathBuf>, u64)>, u64)),
    GlobalSuccess((Results, u64, u64)),
    Failure,
    NotFound(u64),
    Done,
}

impl PartialEq<SearchRes> for SearchRes {
    fn eq(&self, other: &Self) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }
}
