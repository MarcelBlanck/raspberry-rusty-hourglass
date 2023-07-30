use std::convert::TryInto;
use std::fmt;
use std::fs::{metadata, File};
use std::io::Read;

pub struct WavFile {
    is_valid: bool,
    file_name: String,
    file_byte_size: u32,
    fmt_byte_size: u32,
    wav_format: WavFormat,
    channel_count: ChannelCount,
    sample_rate: u32,
    bytes_per_sec: u32,
    bytes_per_sample: u16,
    bits_per_sample: u16,
    current_sample: usize,
    bytes: Vec<u8>,
    data_chunk_offset: usize,
}

pub type ChannelCount = u16;

#[derive(Debug, PartialEq, Eq)]
pub enum WavFormat {
    Pcm,
    Unsupported,
}

impl WavFormat {
    fn new(value: u16) -> Self {
        match value {
            1 => WavFormat::Pcm,
            _ => WavFormat::Unsupported,
        }
    }
}

fn read_string(buffer: &[u8], pos: usize, length: usize) -> String {
    let mut string = String::with_capacity(length);
    for data in buffer.iter().skip(pos).take(length) {
        string.push(*data as char);
    }
    string
}

fn vec8_slice_to_array<const N: usize>(v: &[u8], pos: usize) -> [u8; N] {
    TryInto::try_into(&v[pos..pos + N]).unwrap()
}

fn read_u32_value(v: &[u8], pos: usize) -> u32 {
    u32::from_le_bytes(vec8_slice_to_array::<4>(v, pos))
}

fn read_u16_value(v: &[u8], pos: usize) -> u16 {
    u16::from_le_bytes(vec8_slice_to_array::<2>(v, pos))
}

fn read_i16_value(v: &[u8], pos: usize) -> i16 {
    i16::from_le_bytes(vec8_slice_to_array::<2>(v, pos))
}

impl WavFile {
    pub fn new(file_name: String) -> Self {
        // TODO return invalid in exeptional cases
        let mut file = File::open(&file_name).expect("File not found.");
        let metadata = metadata(&file_name).expect("Metadata could not be read.");
        let mut bytes = vec![0; metadata.len() as usize];
        let n = file.read(&mut bytes).expect("buffer overflow");
        if n != bytes.len() {
            println!("Warning, not all metadata bytes could be read.")
        }

        let riff_string = read_string(&bytes, 0, 4);
        let file_byte_size = read_u32_value(&bytes, 4);

        let wave_string = read_string(&bytes, 8, 4);
        let fmt_string = read_string(&bytes, 12, 4);
        let fmt_byte_size = read_u32_value(&bytes, 16);
        let wav_format = WavFormat::new(read_u16_value(&bytes, 20));
        let channel_count = read_u16_value(&bytes, 22);
        let sample_rate = read_u32_value(&bytes, 24);
        let bytes_per_sec = read_u32_value(&bytes, 28);
        let bytes_per_sample = read_u16_value(&bytes, 32);
        let bits_per_sample = read_u16_value(&bytes, 34);

        // Find first "data" chunk, assuming exacly one exists
        let data_chunk_offset: usize;
        let mut next_chunk_start: usize = 36;
        loop {
            if read_string(&bytes, next_chunk_start, 4) == "data" {
                data_chunk_offset = next_chunk_start + 8;
                break;
            } else {
                let chunk_size = read_u32_value(&bytes, next_chunk_start + 4);
                next_chunk_start = next_chunk_start + 8 + chunk_size as usize;
            }
        }

        WavFile {
            is_valid: riff_string == "RIFF"
                && file_byte_size as u64 == metadata.len()
                && wave_string == "WAVE"
                && fmt_string == "fmt "
                && fmt_byte_size == 16
                && wav_format == WavFormat::Pcm
                && channel_count == 2,
            file_name,
            file_byte_size,
            fmt_byte_size,
            wav_format,
            channel_count,
            sample_rate,
            bytes_per_sec,
            bytes_per_sample,
            bits_per_sample,
            current_sample: 0,
            bytes,
            data_chunk_offset,
        }
    }

    pub fn invalid() -> Self {
        WavFile {
            is_valid: false,
            file_name: String::new(),
            file_byte_size: 0,
            fmt_byte_size: 0,
            wav_format: WavFormat::Unsupported,
            channel_count: 0,
            sample_rate: 0,
            bytes_per_sec: 0,
            bytes_per_sample: 0,
            bits_per_sample: 0,
            current_sample: 0,
            bytes: vec![],
            data_chunk_offset: 0,
        }
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn channel_count(&self) -> u16 {
        self.channel_count
    }

    pub fn get_samples(&mut self, data: &mut [f32], volume_factor: f32) {
        for data_out in data.iter_mut() {
            let sample_data_index = self.data_chunk_offset + 2 * self.current_sample;
            if sample_data_index + 1 < self.bytes.len() {
                let sample = read_i16_value(&self.bytes, sample_data_index);
                *data_out = (sample as f32 / 32768f32) * volume_factor;
                self.current_sample += 1;
            } else {
                *data_out = 0f32;
            }
        }
    }
}

impl fmt::Debug for WavFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WavFile")
            .field("is_valid", &self.is_valid)
            .field("file_byte_size", &self.file_byte_size)
            .field("fmt_byte_size", &self.fmt_byte_size)
            .field("wav_format", &self.wav_format)
            .field("channel_count", &self.channel_count)
            .field("sample_rate", &self.sample_rate)
            .field("bytes_per_sec", &self.bytes_per_sec)
            .field("bytes_per_sample", &self.bytes_per_sample)
            .field("bits_per_sample", &self.bits_per_sample)
            .field("current_sample", &self.current_sample)
            .field("bytes (count)", &self.bytes.len())
            .finish()
    }
}
