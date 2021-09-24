use std::io;
use std::io::Read;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::{thread, time};

pub fn spawn_await_input_enter_thread() -> (thread::JoinHandle<()>, mpsc::Receiver<()>) {
    let (tx, rx): (Sender<()>, Receiver<()>) = mpsc::channel();
    let input_thread = thread::spawn(move || {
        let mut buffer = [0u8; 1];
        io::stdin().read_exact(&mut buffer).unwrap();
        tx.send(()).unwrap();
    });
    (input_thread, rx)
}