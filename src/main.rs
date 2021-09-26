#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::{display::{DisplayControl, Point, Color}, hourglass::HourglassState};
#[cfg(target_arch="arm")]
use crate::display::raspberry_display::RaspberryDisplay;
#[cfg(not(target_arch="arm"))]
use crate::display::minifb_display::MiniFbDisplay;

use std::{sync::Arc, sync::RwLock, thread, time};
use std::time::SystemTime;

mod display;
mod control;
mod ui;
mod hourglass;
mod data;

const MAX_BLINK_TIME_MS: u128 = 120000;

#[actix_web::main]
async fn main() {
    let hourglass_state = Arc::new(RwLock::new(HourglassState::new()));
    let webservice = control::webservice::start_webservice(hourglass_state.clone());
    let (await_input_enter_thread, await_input_enter_rx) = control::input::spawn_await_input_enter_thread();

    #[cfg(target_arch="arm")]
    let mut display = RaspberryDisplay::new();
    #[cfg(not(target_arch="arm"))]
    let mut display = MiniFbDisplay::new();

    display.init();

    println!("Hourglass running. Press Enter to end...");
    loop {
        {
            let hourglass_state_unlocked_r = hourglass_state.read().unwrap();
            let finished_by_webservice = hourglass_state_unlocked_r.finalize;
            let finished_by_key_input = await_input_enter_rx.try_recv().is_ok();
            if  finished_by_key_input || finished_by_webservice {
                println!("Thanks for using hourglass. Good bye!");
                break;
            }
        }

        let current_time_ms = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
        if hourglass_state.read().unwrap().ticking {
            let target_time_ms = hourglass_state.read().unwrap().target_time_ms;
            if current_time_ms < target_time_ms {
                // Draw and animate boxes to show remaining time
                display.fb().fill_with_black();
                ui::block_clock::draw_block_clock((target_time_ms - current_time_ms)/1000, display.fb());
            } else if current_time_ms < target_time_ms + MAX_BLINK_TIME_MS {
                // Blink the display to signal "time's up"
                if (current_time_ms / 500) % 2 == 0 {
                    display.fb().fill_with_white();
                } else {
                    display.fb().fill_with_black();
                }
            } else {
                hourglass_state.write().unwrap().ticking = false;
            }
        } else {
            // Show welcome screen
            display.fb().fill_with_pixmap(&data::WELCOME_SCREEN_PIXMAP);
        }
        display.swap();

        thread::sleep(time::Duration::from_millis(250));
    }

    display.deinit();
    webservice.stop(false).await;
    await_input_enter_thread.join().unwrap();
}
