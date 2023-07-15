use crate::search_engine::search_engine::Engine;
use std::process::exit;

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, RecvError, Sender};
use std::thread;

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
            use std::io::{stdin, stdout, Write};
            loop {
                let mut s = String::new();
                print!("Please enter some text: ");
                let _ = stdout().flush();
                stdin()
                    .read_line(&mut s)
                    .expect("Did not enter a correct string");
                if let Some('\n') = s.chars().next_back() {
                    s.pop();
                }
                if let Some('\r') = s.chars().next_back() {
                    s.pop();
                }
                tx.send(s)
                    .expect("error while sending, maybe is the receiver thread down");
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
                    let res = self.engine.get(inp);
                    println!("{:?}", res);
                    tx_appr
                        .send(true)
                        .expect("error while sending, maybe is the receiver thread down");
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
