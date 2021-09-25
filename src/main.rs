#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::{display::{DisplayControl, Point, Color}, hourglass::HourglassState};
#[cfg(target_arch="arm")]
use crate::display::raspberry_display::RaspberryDisplay;
#[cfg(not(target_arch="arm"))]
use crate::display::minifb_display::MiniFbDisplay;

use std::{sync::Arc, sync::RwLock, thread, time};

mod display;
mod control;
mod ui;
mod hourglass;

#[actix_web::main]
async fn main() {
    let hourglass_state = Arc::new(RwLock::new(HourglassState::new(false, 1200, false)));

    let webservice = control::webservice::start_webservice(hourglass_state.clone());

    let (await_input_enter_thread, await_input_enter_rx) = control::input::spawn_await_input_enter_thread();

    #[cfg(target_arch="arm")]
    let mut display = RaspberryDisplay::new();
    #[cfg(not(target_arch="arm"))]
    let mut display = MiniFbDisplay::new();

    display.init();

    println!("Hourglass running. Press Enter to end...");
    loop {
        let finished_by_key_input = await_input_enter_rx.try_recv().is_ok();
        let finished_by_webservice = hourglass_state.read().unwrap().finalize;
        if  finished_by_key_input || finished_by_webservice {
            println!("Thanks for using hourglass. Good bye!");
            break;
        }

        let remaining_seconds = hourglass_state.read().unwrap().remaining_seconds;
        display.fb().fill_with_black();
        ui::block_clock::draw_block_clock(remaining_seconds, display.fb());
        display.swap();

        thread::sleep(time::Duration::from_millis(33));
    }

    display.deinit();
    webservice.stop(false).await;
    await_input_enter_thread.join().unwrap();
}
