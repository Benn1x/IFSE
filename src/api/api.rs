use std::process::exit;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use crate::api::commands::execute;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use crate::api::lexer::Input;
use crate::search_engine::search_engine::Engine;

pub struct API {
    engine: Engine,
}

impl API {
    pub fn new(engine: Engine) -> Self {
        Self { engine }
    }

    pub fn search(&mut self) {
        let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
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
                        rl.add_history_entry(line.clone())
                            .expect("Unable to write to history");
                        tx.send(line)
                            .expect("error while sending, maybe is the receiver thread down");
                    }
                    Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => tx
                        .send(String::from(":q"))
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
                Ok(inp) => match Input::new(&inp) {
                    Input::Exit => {
                        self.engine.shutdown();
                        exit(0)
                    }
                    Input::Cache => {
                        println!("Cache Content: ");
                        for element in self.engine.iterate() {
                            println!(
                                "Phrase search: {:?} : Found in location {:?}",
                                element.0,
                                element.1.unwarp()
                            );
                        }
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

                    Input::Command(command) => {
                        execute(command.as_ref());
                        tx_appr
                            .send(true)
                            .expect("error while sending, maybe is the receiver thread down");
                    }

                    Input::Input(input) => {
                        let res = self.engine.get(input);
                        println!("Found in: {:?}", res.unwarp());
                        tx_appr
                            .send(true)
                            .expect("error while sending, maybe is the receiver thread down");
                    }
                },
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
