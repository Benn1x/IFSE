use super::super::search::{Results, Search};
use crate::file_system::folder::Folder;
use grep::searcher::sinks::UTF8;
use grep::searcher::Searcher;
use grep_regex::RegexMatcher;
use pdf_extract::extract_text;
use std::path::Path;
use walkdir::WalkDir;
pub struct Backend {
    // search_engine:
}

impl Backend {
    pub fn new() -> Self {
        Self {}
    }
    pub fn search(&self, phrase: &str, path: &Path) -> Option<Results> {
        // todo use pdf_extract to also scan pdfs!!!
        let matcher = RegexMatcher::new(&*phrase).expect("Expected Matcher to return true");
        let mut res: Vec<(u64, String)> = Vec::new();
        if path.extension().unwrap().eq("pdf") {
            let text = match extract_text(path) {
                Ok(text) => text,
                Err(e) => {
                    println!("{:?}", e);
                    return None;
                }
            };
            match Searcher::new().search_slice(
                matcher,
                &*text.as_bytes(),
                UTF8(|line, s| {
                    res.push((line, String::from(s)));
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
                UTF8(|line, s| {
                    res.push((line, String::from(s)));
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
        }
        return if !res.is_empty() {
            Some(Results::new(Folder::new(String::from(format!(
                "{} at line: {}",
                path.display(),
                res[0].0
            )))))
        } else {
            None
        };
    }

    pub fn global_search(&self, search: Search) -> Results {
        let folders = WalkDir::new(search.get_folder().as_path());
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
                    return Results::new(Folder::new(String::from("Failure")));
                }
            };
            if res.is_some() {
                return res.unwrap();
            }
        }

        Results::new(Folder::new(String::from("Not Found")))
    }
}
