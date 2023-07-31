use std::{time::Duration, sync::{Arc, RwLock}};
use std::thread;

use cpal::traits::StreamTrait;

use crate::audio::Player;
use crate::audio::wav_file::WavFile;
use crate::audio::playback_device::create_stream;

pub struct WavPlayer {
    wav_file: Arc<RwLock<Option<WavFile>>>,
    stream: Option<cpal::Stream>
}

impl WavPlayer {
    pub fn new(wav_file_path: String) -> Self {
        let player = WavPlayer {
            wav_file: Arc::new(RwLock::new(None)),
            stream: None
        };

        let wav_file_ref = Arc::clone(&player.wav_file);
        thread::spawn(move || {
            let file = WavFile::new(wav_file_path);
            let mut locked_wav_file = wav_file_ref.write().unwrap();
            *locked_wav_file = Some(file);
        });

        player
    }
}

impl Player for WavPlayer {
    fn play(&mut self, duration: Duration) {

        let wav_file_ref = Arc::clone(&self.wav_file);
        self.stream = Some(create_stream(wav_file_ref));
        if let Some(stream) = &self.stream {
            stream.play().expect("Failed to play the audio stream.");
        } else {
            eprintln!("No audio stream to play.");
        }
    }

    fn stop(&mut self) {
        if let Some(stream) = &self.stream {
            stream.pause().expect("Failed to pause the audio stream.");
        } else {
            eprintln!("No audio stream to pause.");
        }
        if let Ok(wav_file_lock) = &mut self.wav_file.write() {
            if let Some(wav_file) = wav_file_lock.as_mut() {
                wav_file.rewind();
            } else {
                eprintln!("WavFile is None");
            }
        } else {
            eprintln!("WavFile lock could not be aquired");
        }
    }

    fn is_ready(&self) -> bool {
        self.wav_file.read().unwrap().is_some()
    }
}