use super::super::search::{Results, Search};
use crate::file_system::folder::Folder;
use grep::searcher::sinks::UTF8;
use grep::searcher::Searcher;
use grep_regex::RegexMatcher;
use walkdir::WalkDir;

pub struct Backend {
    // search_engine:
}

impl Backend {
    pub fn new() -> Self {
        Self {}
    }
    pub fn search(&self, search: Search) -> Option<Results> {
        let matcher =
            RegexMatcher::new(search.get_phrase()).expect("Expected Matcher to return true");
        let mut res: Vec<(u64, String)> = Vec::new();
        match Searcher::new().search_path(
            matcher,
            search.get_folder().as_path(),
            UTF8(|line, s| {
                res.push((line, String::from(s)));
                return Ok(true);
            }),
        ) {
            Ok(_) => (),
            Err(e) => println!(
                "Error opening file {}, following error occurred: {}",
                search.folder.get_folder_location(),
                e
            ),
        }
        return if !res.is_empty() {
            Some(Results::new(Folder::new(String::from(format!(
                "{} at line: {}",
                search.get_folder().get_folder_location(),
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
                    if path.file_type().is_dir() {
                        continue;
                    }
                    self.search(Search::new(
                        search.phrase.clone(),
                        Folder::new(path.path().display().to_string()),
                    ))
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
