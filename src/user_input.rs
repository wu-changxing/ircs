use std::sync::mpsc::{self, Sender};
use std::thread;

pub fn spawn_user_input_thread(tx: Sender<String>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_string();
            if input.is_empty() {
                continue;
            }

            // Send the user input to the main thread
            tx.send(input).unwrap();
        }
    })
}
