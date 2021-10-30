use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Data, Sample, SampleFormat, SampleRate};
use std::fs::read;

use crate::audio::{self, audio_playback};

fn find_audio_output_device(start_of_name: Option<&str>) -> Option<cpal::Device> {
    cpal::default_host()
        .output_devices()
        .unwrap()
        .find(|device| {
            (start_of_name.is_none() || device.name().unwrap().starts_with(start_of_name.unwrap()))
                && device.supported_output_configs().unwrap().next().is_some()
        })
}

fn find_any_audio_output_device() -> Option<cpal::Device> {
    cpal::default_host()
        .output_devices()
        .unwrap()
        .find(|device| {
            let mut supported_configs_range = device
                .supported_output_configs()
                .expect("error while querying configs");
            supported_configs_range.next().is_some()
        })
}

pub fn play(wav_filename: &str) -> cpal::Stream {
    let mut wav_file = audio::wav_reader::WavFile::new(wav_filename.to_string());
    // TODO deal with invalid wav file

    let device = find_audio_output_device(Some("sysdefault"))
        .or_else(|| find_audio_output_device(None))
        .unwrap();

    println!("{}", device.name().unwrap());

    let mut supported_configs = device
        .supported_output_configs()
        .expect("Error while querying audio configs");

    let supported_stream_config = supported_configs
        .find(|config_range| {
            config_range.channels() == wav_file.channel_count as cpal::ChannelCount
                && config_range.min_sample_rate() <= cpal::SampleRate(wav_file.sample_rate)
                && config_range.max_sample_rate() >= cpal::SampleRate(wav_file.sample_rate)
                && config_range.sample_format() == cpal::SampleFormat::F32
        })
        .unwrap()
        .with_sample_rate(cpal::SampleRate(wav_file.sample_rate));

    println!("supported_stream_config {:?}", supported_stream_config);
    let stream_config = supported_stream_config.config().clone();
    println!("stream_config {:?}", stream_config);

    device
        .build_output_stream(
            &stream_config,
            move |data: &mut [f32], info: &cpal::OutputCallbackInfo| {
                wav_file.get_samples(data, 0.05)
            },
            |error| {
                println!("Audio output error: {}", error);
            },
        )
        .unwrap()
}
