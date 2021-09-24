#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::display::{DisplayControl, Point, Color};
#[cfg(target_arch="arm")]
use crate::display::raspberry_display::RaspberryDisplay;
#[cfg(not(target_arch="arm"))]
use crate::display::minifb_display::MiniFbDisplay;

use std::{thread, time};

mod display;
mod control;
mod ui;

#[actix_web::main]
async fn main() {
    let webservice = control::webservice::start_webservice();

    let (await_input_enter_thread, await_input_enter_rx) = control::input::spawn_await_input_enter_thread();

    #[cfg(target_arch="arm")]
    let mut display = RaspberryDisplay::new();
    #[cfg(not(target_arch="arm"))]
    let mut display = MiniFbDisplay::new();

    display.init();

    println!("Hourglass running. Press Enter to end...");
    let mut remaining_seconds = 0;
    loop {
        match webservice.hourglass_state_rx.try_recv() {
            Ok(msg) => {
                println!("State changed {:?}", msg);
                if msg.finalize {
                    println!("Ended by Webservice: Thanks for using hourglass. Good bye!");
                    break;
                } else {
                    remaining_seconds = msg.remaining_seconds;
                }
            },
            Err(_) => ()
        }

        if await_input_enter_rx.try_recv().is_ok() {
            println!("Ended by key: Thanks for using hourglass. Good bye!");
            break;
        }

        display.fb().fill_with_black();
        ui::block_clock::draw_block_clock(remaining_seconds, display.fb());
        display.swap();

        thread::sleep(time::Duration::from_millis(33));
    }

    display.deinit();
    webservice.server_control.stop(false).await;
    await_input_enter_thread.join().unwrap();
}
