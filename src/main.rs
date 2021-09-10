#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::display::Display;
use crate::display_data::{Point, Color};

use std::io;
use std::io::Read;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::{thread, time};

use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use rppal::gpio::{Gpio, OutputPin};

mod display;
mod display_data;
mod hardware;

fn get_initialized_display() -> Display<Spi, OutputPin> {
    let mut display = Display::new(
        Spi::new(Bus::Spi0, SlaveSelect::Ss0, 2_000_000, Mode::Mode0).unwrap(),
        rppal::gpio::Gpio::new().unwrap().get(25).unwrap().into_output(),
        rppal::gpio::Gpio::new().unwrap().get(24).unwrap().into_output()
    );
    display.init();
    display
}

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
    let mut display = get_initialized_display();

    println!("Hourglass running. Press Enter to end...");
    loop {
        display.fill_with_black();
        display.draw_box_with_coords(0, 0, 31, 127,&Color::Black, &Color::White);
        display.draw_line_with_coords( 0, 0, 31, 127,&Color::White);
        display.draw_line_with_coords( 31, 0, 0, 127,&Color::White);

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
