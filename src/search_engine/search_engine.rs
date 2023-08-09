use crate::search_engine::backend::backend::Backend;
use log::{info, warn};
use std::{path::Path, time::Duration};

use crate::search_engine::search::{Search, SearchRes};
use moka::sync::{Cache, Iter};

pub struct Engine {
    cache: Cache<String, SearchRes>,
    size: u64,
    backend: Backend,
}

impl Engine {
    pub fn new() -> Self {
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
        }
    }

    pub fn get(&mut self, phrase: String) -> (SearchRes, Duration) {
        let now = std::time::Instant::now();
        match self.cache.get(&*phrase) {
            Some(entry) => {
                info!("Found inside cache");
                (entry, now.elapsed())
            }
            None => {
                info!("Not found in cache. Start searching!");
                let search = Search::new(
                    String::from(&*phrase),
                    Box::new(Path::new("/home/benn1x/Dokumente/").to_owned()),
                );
                info!("Start Search!");
                let result = self.backend.global_search(search);
                info!("Found! Gave cache to decide if its put in cache!");
                let now = std::time::Instant::now();
                self.cache.insert(phrase, result.clone());
                (result, now.elapsed())
            }
        }
    }
    pub fn cache_size(&self) -> u64 {
        self.cache.weighted_size()
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn iterate(&self) -> Iter<'_, String, SearchRes> {
        self.cache.iter()
    }

    pub fn shutdown(&self) {
        // TODO: I need a method that safes the cache to a file and later at boot up it should be reloaded
    }
}
