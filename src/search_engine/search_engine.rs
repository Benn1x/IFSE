use crate::search_engine::backend::backend::Backend;
use log::{info, warn};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::{path::Path, time::Duration};

use crate::search_engine::search::{Search, SearchRes};
use moka::sync::{Cache, Iter};

#[derive(Clone)]
pub struct Engine {
    cache: Cache<String, SearchRes>,
    size: u64,
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
        }
    }

    pub fn get(&mut self, phrase: String, tx_: Sender<(SearchRes, Duration)>) {
        let now = std::time::Instant::now();
        match self.cache.get(&*phrase) {
            Some(entry) => {
                info!("Found inside cache");
                tx_.send((entry, now.elapsed())).expect("Receiver down");
            }
            None => {
                info!("Not found in cache. Start searching!");
                let search = Search::new(
                    phrase.clone(),
                    Box::new(Path::new("/home/benn1x/Dokumente/").to_owned()),
                );
                info!("Start Search!");
                let (tx, rx): (Sender<SearchRes>, Receiver<SearchRes>) = mpsc::channel();
                {
                    let search = search;
                    let tx = tx;
                    std::thread::spawn(|| {
                        Backend::new().global_search(search, tx);
                    });
                }
                loop {
                    let res = match rx.recv() {
                        Ok(res) => res,
                        Err(e) => {
                            println!("{e}");
                            tx_.send((SearchRes::Failure, now.elapsed()))
                                .expect("Receiver down");
                            return;
                        }
                    };
                    if res != SearchRes::Done {
                        self.cache.insert(phrase.clone(), res.clone());
                        tx_.send((res, now.elapsed())).expect("Receiver down");
                    } else {
                        tx_.send((res, now.elapsed())).expect("Receiver down");
                        break;
                    }
                }
                info!("Found! Gave cache to decide if its put in cache!");
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
