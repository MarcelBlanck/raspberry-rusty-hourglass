use crate::display::{DisplayControl, DisplayBuffer, Point, Color, WIDTH, HEIGHT};
use minifb::{Key, Window, WindowOptions, Scale};
use std::convert::TryFrom;

const WHITE: u32 = 0xFFFFFFFFu32;
const BLACK: u32 = 0x00000000u32;

pub struct MiniFbDisplay {
    fb: DisplayBuffer,
    buffer: [u32; 32*128],
    window: Window
}

impl MiniFbDisplay {
    pub fn new() -> Self {
        MiniFbDisplay {
            fb: DisplayBuffer::new(),
            buffer: [0u32; 32*128],
            window: Window::new(
                "Hourglass",
                32,
                128,
                WindowOptions {
                    resize: false,
                    scale: Scale::X4,
                    borderless: false,
                    title: false,
                    ..WindowOptions::default()
                },
            )
            .expect("Unable to create window")
        }
    }
}

impl DisplayControl for MiniFbDisplay {
    fn init(&mut self) {
        // nothing to do
    }

    fn deinit(&mut self) {
        // nothing to do
    }

    fn swap(&mut self) {
        println!("Buffersize {}", self.buffer.len() );
        for i in 0..self.buffer.len() {
            let i_isize = isize::try_from(i).unwrap();
            let point = Point{
                x: i_isize % 32,
                y: 127 - i_isize / 32
            };
            self.buffer[i] = match self.fb.get_pixel_color(&point) {
                Ok(Color::White) => WHITE,
                Ok(Color::Black) => BLACK,
                Err(s) => panic!("Unable to get pixel color {}", s)
            }
        }
        self.window.update_with_buffer(&self.buffer).unwrap();
    }

    fn fb<'a>(&'a mut self) -> &'a mut DisplayBuffer {
        &mut self.fb
    }
}