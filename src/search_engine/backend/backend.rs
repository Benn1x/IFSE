use super::super::search::Search;
use crate::file_system::folder::Folder;
use crate::search_engine::search::SearchRes::GlobalSuccess;
use crate::search_engine::search::{Results, SearchRes};
use grep::searcher::sinks::UTF8;
use grep::searcher::Searcher;
use log::info;
use lopdf::Document;
use std::path::Path;
use walkdir::WalkDir;

pub struct Backend {
    // search_engine:
}

impl Backend {
    pub fn new() -> Self {
        Self {}
    }
    pub fn search(&self, phrase: &str, path: &Path) -> SearchRes {
        let matcher =
            grep::regex::RegexMatcher::new(&*phrase).expect("Expected Matcher to return true");
        let mut res: Vec<(Folder, u64)> = Vec::new();
        if path.is_dir() || !path.is_file() || path.is_symlink() {
            return SearchRes::NotFound;
        }
        match path.extension() {
            Some(ex) => {
                if ex.eq("pdf") {
                    //content::Content::from_ops();
                    info!("{}", path.display());
                    let text = match Document::load(path) {
                        Ok(text) => {
                            let pages = text.get_pages();
                            let mut texts = Vec::new();

                            for (i, _) in pages.iter().enumerate() {
                                let page_number = (i + 1) as u32;
                                let text = text.extract_text(&[page_number]);
                                texts.push(text.unwrap_or_default());
                            }
                            texts.join("")
                        }
                        Err(e) => {
                            println!("{:?}", e);
                            return SearchRes::Failure;
                        }
                    };
                    match Searcher::new().search_slice(
                        matcher,
                        &*text.as_bytes(),
                        UTF8(|line, _| {
                            res.push((Folder::new(path.display().to_string()), line));
                            return Ok(true);
                        }),
                    ) {
                        Ok(_) => (),
                        Err(e) => println!(
                            "Error opening file {}, following error occurred: {}",
                            path.display(),
                            e
                        ),
                    }
                } else {
                    match Searcher::new().search_path(
                        matcher,
                        &path,
                        UTF8(|line, _| {
                            res.push((Folder::new(path.display().to_string()), line));
                            return Ok(true);
                        }),
                    ) {
                        Ok(_) => (),
                        Err(_) => (),
                    }
                }
            }
            _ => (),
        }

        return if !res.is_empty() {
            SearchRes::Success(res)
        } else {
            SearchRes::NotFound
        };
    }

    pub fn global_search(&self, search: Search) -> SearchRes {
        let folders = WalkDir::new(search.get_folder().get_folder_location());
        let mut search_results = Vec::<SearchRes>::new();
        let mut all_res = Vec::<(Folder, u64)>::new();
        for path in folders {
            let res = match path {
                Ok(path) => {
                    if path.clone().into_path().is_dir() {
                        continue;
                    }
                    self.search(&*search.phrase, path.into_path().as_path())
                }
                Err(err) => {
                    println!("{}", err);
                    return SearchRes::Failure;
                }
            };
            search_results.push(res);
        }

        for s_res in search_results.iter() {
            match s_res {
                SearchRes::Success(res) => all_res.extend(res.iter().cloned()),
                _ => {}
            }
        }
        if !all_res.is_empty() {
            return GlobalSuccess(Results::new(all_res));
        }

        SearchRes::NotFound
    }
}
