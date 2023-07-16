use crate::file_system::folder::Folder;
use crate::search_engine::{backend::backend::Backend, search::Results};
use log::{info, warn};

use crate::search_engine::search::Search;
use moka::sync::{Cache, Iter};

pub struct Engine {
    cache: Cache<String, Results>,
    size: u64,
    backend: Backend,
    _folder: Folder,
}

impl Engine {
    pub fn new(location: String) -> Self {
        let max_num = match std::env::var("MAX_NUMBER_IN_CACHE") {
            Ok(val) => {
                match val.parse::<u64>() {
                    Ok(val) => val,
                    Err(_) => {
                        warn!("Unable to parse te content of MAX_NUMBER_IN_CACHE, defaulting to 10.000");
                        10_000
                    }
                }
            }
            Err(_) => {
                warn!("Unable to find MAX_NUMBER_IN_CACHE as env variable, defaulting to 10.000");
                10_000
            }
        };
        Self {
            cache: Cache::new(max_num),
            size: max_num,
            backend: Backend::new(),
            _folder: Folder::new(location),
        }
    }

    pub fn get(&mut self, phrase: String) -> Results {
        match self.cache.get(&*phrase) {
            Some(entry) => {
                info!("Found inside cache");
                entry
            }
            None => {
                info!("Not found in cache start searching!");
                let search =
                    Search::new(String::from(&*phrase), Folder::new(String::from(&*phrase)));
                info!("Start Search!");
                let result = self.backend.search(search);
                info!("Found and give cache to decide if its put in cache!");
                self.cache.insert(phrase, result.clone());

                result
            }
        }
    }
    pub fn cache_size(&self) -> u64 {
        self.cache.weighted_size()
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn iterate(&self) -> Iter<'_, String, Results> {
        self.cache.iter()
    }
}
