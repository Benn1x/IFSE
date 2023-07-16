use crate::search_engine::search_engine::Engine;
use std::process::exit;

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
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
                Ok(inp) => match &*inp {
                    ":q" => exit(0),
                    ":cs" => {
                        println!("Total cache size is {:?}, currently in cache are {:?} elements. Leaving {:?} free spaces", self.engine.get_size() ,self.engine.cache_size(), self.engine.get_size() - self.engine.cache_size());
                        tx_appr
                            .send(true)
                            .expect("error while sending, maybe is the receiver thread down");
                    }
                    ":c" => {
                        println!("Cache Content: ");
                        for element in self.engine.iterate() {
                            println!("{:?}", element);
                        }
                        tx_appr
                            .send(true)
                            .expect("error while sending, maybe is the receiver thread down");
                    }
                    ":h" => {
                        println!("Help, this will be here in a decade or so shrug");
                        tx_appr
                            .send(true)
                            .expect("error while sending, maybe is the receiver thread down");
                    }
                    _ => {
                        let res = self.engine.get(inp);
                        println!("{:?}", res);
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
