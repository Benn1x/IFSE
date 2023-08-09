use std::process::exit;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use crate::api::commands::execute;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use crate::api::lexer::{Command, Input};
use crate::search_engine::search::SearchRes;
use crate::search_engine::search_engine::Engine;

pub struct API {
    engine: Engine,
}

impl API {
    pub fn new(engine: Engine) -> Self {
        Self { engine }
    }

    pub fn search(&mut self) {
        let (tx, rx): (Sender<Input>, Receiver<Input>) = mpsc::channel();
        let (tx_appr, rx_appr): (Sender<bool>, Receiver<bool>) = mpsc::channel();

        let input = thread::spawn(move || {
            let mut rl = match DefaultEditor::new() {
                Ok(rl) => rl,
                Err(err) => panic!("Unable to init reader, with err: {:?}", err),
            };
            if rl.load_history("history.txt").is_err() {
                println!("No previous history.");
            }

            loop {
                let readline = rl.readline(">> ");
                match readline {
                    Ok(line) => {
                        tx.send(Input::new(&line))
                            .expect("error while sending, maybe is the receiver thread down");
                        rl.add_history_entry(line)
                            .expect("Unable to write to history");
                    }
                    Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => tx
                        .send(Input::new(&String::from(":q")))
                        .expect("error while sending, maybe is the receiver thread down"),
                    Err(err) => {
                        panic!("Error: {:?}", err);
                    }
                }
                match rl.save_history("history.txt") {
                    Ok(_) => (),
                    Err(err) => panic!("{:?}", err),
                }
                let appr = match rx_appr.recv() {
                    Ok(t) => t,
                    Err(e) => panic!("Failed to get verification {:?}", e),
                };
                if !appr {
                    exit(1);
                }
            }
        });

        loop {
            let input = rx.recv();
            match input {
                Ok(inp) => {
                    match inp {
                        Input::Exit => {
                            self.engine.shutdown();
                            exit(0)
                        }
                        Input::Cache => {
                            println!("Cache Content: ");
                            self.engine.iterate().for_each(|element| {
                                println!(
                                    "Phrase search: {:?} : Found in location {:?}",
                                    element.0, element.1
                                );
                            });
                            tx_appr
                                .send(true)
                                .expect("error while sending, maybe is the receiver thread down");
                        }
                        Input::CacheSize => {
                            println!("Total cache size is {:?}, currently in cache are {:?} elements. Leaving {:?} free spaces", self.engine.get_size() ,self.engine.cache_size(), self.engine.get_size() - self.engine.cache_size());
                            tx_appr
                                .send(true)
                                .expect("error while sending, maybe is the receiver thread down");
                        }

                        Input::Empty => (),

                        Input::Input(input) => {
                            if input.starts_with(":") {
                                let inp: Vec<&str> = input.split_whitespace().collect();
                                let command = Command::new(inp);
                                execute(&command);
                            } else {
                                let now = std::time::Instant::now();
                                let res = self.engine.get(input);
                                let time = now.elapsed();
                                let mut size = 0;
                                let mut element_count = 0;
                                let mut threads = 0;
                                match res.0 {
                                    SearchRes::Success(_) => {}
                                    SearchRes::GlobalSuccess(res) => {
                                        for s_res in res.0.unwrap().iter() {
                                            println!(
                                                "Found in File {} in Line {}",
                                                s_res.0.display(),
                                                s_res.1
                                            )
                                        }
                                        element_count = res.0.size();
                                        size = res.1;
                                        threads = res.2;
                                    }

                                    SearchRes::Failure => {
                                        println!("An unexpected behavior accorded. please check the logs")
                                    }
                                    SearchRes::NotFound(size) => println!(
                                        "Searched in {}mb of data  and no result was found! :(",
                                        size
                                    ),
                                }
                                println!(
                                    "Total operation took {:?} and has used {} threads and searched in {}mb of data with a total count of {} elements, {:?} time needed to load from/into cache",
                                    time,
                                    threads,
                                    size/1_000_000,
                                    element_count,
                                    res.1,
                                );
                            }
                            tx_appr
                                .send(true)
                                .expect("error while sending, maybe is the receiver thread down");
                        }
                    }
                }
                Err(recver) => {
                    println!("{:?}", recver);
                    tx_appr
                        .send(false)
                        .expect("error while sending maybe is the receiver thread down");
                    break;
                }
            }
        }
        input.join().expect("Input thread failed");
    }
}
