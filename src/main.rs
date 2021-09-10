#[cfg(target_arch = "arm-unknown-linux-gnueabihf")]
use crate::display::{Display};
#[cfg(target_arch = "arm-unknown-linux-gnueabihf")]
use crate::display_data::{Point, Color};
#[cfg(target_arch = "arm-unknown-linux-gnueabihf")]
use crate::hardware::{SpiInterface, PinInterface};

#[cfg(target_arch = "arm-unknown-linux-gnueabihf")]
use std::{thread, time};
#[cfg(target_arch = "arm-unknown-linux-gnueabihf")]
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
#[cfg(target_arch = "arm-unknown-linux-gnueabihf")]
use wiringpi::pin::OutputPin;

mod display;
mod display_data;
mod hardware;

#[cfg(all(not(test), not(target_arch = "arm-unknown-linux-gnueabihf")))]
fn main() {
	println!("Nothing to do without a Pi.")
}

#[cfg(all(not(test), target_arch = "arm-unknown-linux-gnueabihf"))]
fn main() {
	let pi = wiringpi::setup();
	let mut display = Display::new(
		Spi::new(Bus::Spi0, SlaveSelect::Ss0, 2_000_000, Mode::Mode0).unwrap(), 
		pi.output_pin(6), 
		pi.output_pin(5)
	);
	display.init();

	let square_size = 8;
	let row_count = 100 / square_size;
	let column_count = 32 / square_size;
	let left_margin = 32 % column_count / 2;

	loop {
		display.fill_with_black();
		for row in 0..row_count {
			let y_offset = row * square_size;
			for column in 0..column_count {
				let x_offset = column * square_size;
				display.draw_box(
					&Point{x: left_margin + x_offset, y: square_size - y_offset - 1}, 
					&Point{x: left_margin + square_size + x_offset - 1, y: y_offset}, 
					&Color::White, 
					&Color::Black
				);
			}
		}
		display.swap();
		thread::sleep(time::Duration::from_millis(1000));
	}
}
