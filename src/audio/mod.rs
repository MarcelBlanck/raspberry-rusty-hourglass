pub mod wav_player;
pub mod playback_device;
mod wav_file;

use std::time::Duration;

pub trait Player {
    fn play(&mut self, duration: Duration);
    fn stop(& mut self);
    fn is_ready(&self) -> bool;
}