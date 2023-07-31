#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use audio::wav_player;
use audio::Player;

#[cfg(not(target_arch = "arm"))]
use crate::gui::display_minifb::MiniFbDisplay;
#[cfg(target_arch = "arm")]
use crate::gui::display_raspberry::RaspberryDisplay;

use crate::gui::display_control::{Color, DisplayControl, Point};
use crate::hourglass::HourglassState;

use std::time::{SystemTime, Duration};
use std::{sync::Arc, sync::RwLock, thread, time};

mod audio;
mod control;
mod data;
mod gui;
mod hourglass;

const MAX_BLINK_TIME_MS: u128 = 120000;

#[actix_web::main]
async fn main() {
    let wav_file_path = "./audio/424244__aceinet__number-90-flange-the-hammer-on-e.wav".to_string();
    let mut wav_player = wav_player::WavPlayer::new(wav_file_path);

    let hourglass_state = Arc::new(RwLock::new(HourglassState::new()));
    control::webservice::start_webservice(hourglass_state.clone());
    let (await_input_enter_thread, await_input_enter_rx) =
        control::input::spawn_await_input_enter_thread();

    #[cfg(target_arch = "arm")]
    let mut display = RaspberryDisplay::new();
    #[cfg(not(target_arch = "arm"))]
    let mut display = MiniFbDisplay::new();

    thread::sleep(time::Duration::from_millis(1250));
    display.init();

    println!("Hourglass running. Press Enter to end...");

    // These variables help minimize the display update.
    // They make the ui drawing look a bit more complex,
    // but save a lot of processing and energy.
    let mut last_remaining_seconds = 0;
    let mut welcome_screen_shown = false;
    let mut is_filled_white = false;
    let mut end_audio_played = false;

    loop {
        {
            let hourglass_state_unlocked_r = hourglass_state.read().unwrap();
            let finished = await_input_enter_rx.try_recv().is_ok();
            if finished {
                println!("Thanks for using hourglass. Good bye!");
                break;
            }
        }

        let current_time_ms = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        if hourglass_state.read().unwrap().ticking {
            welcome_screen_shown = false;
            let target_time_ms = hourglass_state.read().unwrap().target_time_ms;
            if current_time_ms < target_time_ms {
                is_filled_white = false;
                end_audio_played = false;
                // Draw and animate boxes to show remaining time
                let remaining_seconds = (target_time_ms - current_time_ms) / 1000;
                if remaining_seconds != last_remaining_seconds {
                    last_remaining_seconds = remaining_seconds;
                    display.fb().fill_with_black();
                    gui::block_clock::draw_block_clock(remaining_seconds, display.fb());
                    display.safe_swap();
                }
            } else if current_time_ms < target_time_ms + MAX_BLINK_TIME_MS {
                last_remaining_seconds = 0;
                // Blink the display to signal "time's up"
                let fill_white = (current_time_ms / 500) % 2 == 0;
                if fill_white && !is_filled_white {
                    is_filled_white = true;
                    display.fb().fill_with_white();
                    display.safe_swap();
                } else if !fill_white && is_filled_white {
                    is_filled_white = false;
                    display.fb().fill_with_black();
                    display.safe_swap();
                }

                if !end_audio_played {
                    wav_player.play(Duration::from_secs(120));
                    end_audio_played = true;
                }
            } else {
                let mut hourglass_state_unlocked_rw = hourglass_state.write().unwrap();
                hourglass_state_unlocked_rw.target_time_ms = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis();
                hourglass_state_unlocked_rw.duration_ms = 0;
                hourglass_state_unlocked_rw.ticking = false;
                end_audio_played = false;
            }
        } else {
            // Show welcome screen
            if !welcome_screen_shown {
                welcome_screen_shown = true;
                last_remaining_seconds = 0;
                is_filled_white = false;
                display.fb().fill_with_pixmap(&data::WELCOME_SCREEN_PIXMAP);
                display.safe_swap();
            }
        }
        thread::sleep(time::Duration::from_millis(250));
    }

    display.deinit();
    await_input_enter_thread.join().unwrap();
}
