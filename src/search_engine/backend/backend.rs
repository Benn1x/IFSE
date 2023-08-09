use super::super::search::Search;
use crate::search_engine::search::SearchRes::GlobalSuccess;
use crate::search_engine::search::{Results, SearchRes};
use grep::searcher::sinks::UTF8;
use grep::searcher::Searcher;
use log::info;
use lopdf::Document;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use walkdir::WalkDir;

pub struct Backend {}

impl Backend {
    pub fn new() -> Self {
        Self {}
    }
    pub fn search(&self, phrase: &str, path: Box<PathBuf>) -> SearchRes {
        let matcher =
            grep::regex::RegexMatcher::new(&*phrase).expect("Expected Matcher to return true");
        let mut res: Vec<(Box<PathBuf>, u64)> = Vec::new();
        if path.is_dir() || !path.is_file() || path.is_symlink() {
            return SearchRes::NotFound(0);
        }
        match path.extension() {
            Some(ex) => {
                let text = match std::fs::read(&path.as_path()) {
                    Ok(text) => {
                        if ex.eq("pdf") {
                            //content::Content::from_ops();
                            info!("{}", path.display());
                            match Document::load(&*path) {
                                Ok(text) => {
                                    let pages = text.get_pages();
                                    let mut texts = Vec::new();

                                    for (i, _) in pages.iter().enumerate() {
                                        let page_number = (i + 1) as u32;
                                        let text = text.extract_text(&[page_number]);
                                        texts.push(text.unwrap_or_default());
                                    }
                                    let final_text = texts.join("");
                                    final_text.as_bytes().to_vec()
                                }
                                Err(_) => {
                                    return SearchRes::Failure;
                                }
                            }
                        } else {
                            text
                        }
                    }
                    Err(_) => {
                        return SearchRes::Failure;
                    }
                };

                match Searcher::new().search_slice(
                    matcher,
                    &*text,
                    UTF8(|line, _| {
                        res.push((path.clone(), line));
                        return Ok(true);
                    }),
                ) {
                    Ok(_) => (),
                    Err(_) => (),
                }
            }
            _ => (),
        }

        return if !res.is_empty() {
            SearchRes::Success((res, path.metadata().unwrap().size()))
        } else {
            SearchRes::NotFound(path.metadata().unwrap().size())
        };
    }

    pub fn global_search(&self, search: Search) -> SearchRes {
        let folders = WalkDir::new(search.get_folder().as_path()).max_depth(1);
        let mut search_results = Vec::<SearchRes>::new();
        let mut all_res = Vec::<(Box<PathBuf>, u64)>::new();
        let (tx, rx): (Sender<Vec<SearchRes>>, Receiver<Vec<SearchRes>>) = mpsc::channel();
        let mut threads = 0;
        let mut vec_threads = Vec::<thread::JoinHandle<()>>::new();
        let tx_mutex = Arc::new(Mutex::new(tx));
        for path in folders {
            match path {
                Ok(path_t) => {
                    threads += 1;
                    {
                        let tx_t = tx_mutex.clone();
                        let search_t = search.clone();
                        let path_t = path_t.clone();
                        vec_threads.push(thread::spawn(move || {
                            let folder = WalkDir::new(path_t.into_path());
                            let mut res = Vec::<SearchRes>::new();
                            for path_t in folder {
                                match path_t {
                                    Ok(path) => {
                                        if path.clone().into_path().is_dir() {
                                            continue;
                                        }

                                        //println!("{}", total_size);
                                        res.push(Backend::new().search(
                                            &*search_t.phrase,
                                            Box::new(path.path().to_owned()),
                                        ));
                                    }
                                    Err(err) => {
                                        println!("{}", err);
                                    }
                                }
                            }
                            tx_t.lock()
                                .unwrap()
                                .send(res)
                                .expect("Unable to send the result");
                        }));
                    }
                }
                Err(err) => {
                    println!("{}", err);
                    return SearchRes::Failure;
                }
            }
        }
        let total_thread = threads;
        loop {
            match rx.recv() {
                Ok(res) => {
                    search_results.extend(res.clone());
                    threads -= 1;
                    if threads == 0 {
                        break;
                    }
                }
                Err(_) => {
                    if threads == 0 {
                        break;
                    }
                }
            }
        }
        for thread in vec_threads {
            thread.join().expect("Unable to join the thread");
        }
        let mut total_size = 0;
        for s_res in search_results.iter() {
            match s_res {
                SearchRes::Success(res) => {
                    all_res.extend_from_slice(&res.0);
                    total_size += res.1;
                }
                SearchRes::NotFound(size) => total_size += size,
                _ => {}
            }
        }
        if !all_res.is_empty() {
            return GlobalSuccess((Results::new(all_res), total_size, total_thread));
        }

        SearchRes::NotFound(total_size)
    }
}
