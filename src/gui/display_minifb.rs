use crate::gui::display_control::{Color, DisplayBuffer, DisplayControl, Point, HEIGHT, WIDTH};
use minifb::{Key, Scale, Window, WindowOptions};
use std::convert::TryFrom;

const WHITE: u32 = 0xFFFFFFFFu32;
const BLACK: u32 = 0x00000000u32;

pub struct MiniFbDisplay {
    fb: DisplayBuffer,
    buffer: [u32; 32 * 128],
    window: Window,
}

impl MiniFbDisplay {
    pub fn new() -> Self {
        MiniFbDisplay {
            fb: DisplayBuffer::new(),
            buffer: [0u32; WIDTH as usize * HEIGHT as usize],
            window: Window::new(
                "Hourglass",
                WIDTH as usize,
                HEIGHT as usize,
                WindowOptions {
                    resize: false,
                    scale: Scale::X4,
                    borderless: false,
                    title: false,
                    ..WindowOptions::default()
                },
            )
            .expect("Unable to create window"),
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
        for i in 0..self.buffer.len() {
            let i_isize = isize::try_from(i).unwrap();
            let point = Point {
                x: i_isize % 32,
                y: 127 - i_isize / 32,
            };
            self.buffer[i] = match self.fb.get_pixel_color(&point) {
                Ok(Color::White) => WHITE,
                Ok(Color::Black) => BLACK,
                Err(s) => panic!("Unable to get pixel color {}", s),
            }
        }
        self.window
            .update_with_buffer(&self.buffer, 32, 127)
            .unwrap();
    }

    fn safe_swap(&mut self) {
        self.swap();
        self.swap(); // Additional swap to ensure display when double buffering
    }

    fn fb<'a>(&'a mut self) -> &'a mut DisplayBuffer {
        &mut self.fb
    }
}
