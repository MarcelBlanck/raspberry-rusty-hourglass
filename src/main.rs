#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::display::{DisplayControl, Point, Color};
#[cfg(target_arch="arm")]
use crate::display::raspberry_display::RaspberryDisplay;
#[cfg(not(target_arch="arm"))]
use crate::display::minifb_display::MiniFbDisplay;

use std::io;
use std::io::Read;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::{thread, time};

mod display;

fn spawn_input_thread() -> (thread::JoinHandle<()>, mpsc::Receiver<()>) {
    let (tx, rx): (Sender<()>, Receiver<()>) = mpsc::channel();
    let input_thread = thread::spawn(move || {
        let mut buffer = [0u8; 1];
        io::stdin().read_exact(&mut buffer).unwrap();
        tx.send(()).unwrap();
    });
    (input_thread, rx)
}

fn main() {
    let (input_thread, input_rx) = spawn_input_thread();

    #[cfg(target_arch="arm")]
    let mut display = RaspberryDisplay::new();
    #[cfg(not(target_arch="arm"))]
    let mut display = MiniFbDisplay::new();
    display.init();

    println!("Hourglass running. Press Enter to end...");
    loop {
        display.fb().fill_with_black();
        display.fb().draw_box_with_coords(0, 0, 31, 127,&Color::Black, &Color::White);
        display.fb().draw_line_with_coords( 0, 0, 31, 127,&Color::White);
        display.fb().draw_line_with_coords( 31, 0, 0, 127,&Color::White);
        display.fb().draw_line_with_coords( 16, 0, 16, 64,&Color::White);
        display.fb().draw_line_with_coords( 15, 0, 15, 64,&Color::White);
        display.swap();

        thread::sleep(time::Duration::from_millis(1000));

        if input_rx.try_recv().is_ok() {
            println!("Thanks for using hourglass. Good bye!");
            break;
        }
    }

    display.deinit();
    input_thread.join().unwrap();
}
