use crate::display_data::{AsciiFont, AsciiText, Color, Pixmap, Point};
use crate::hardware::{SpiInterface, PinInterface};
use std::{thread, time};

pub struct Display<T, U> {
    buffer: [u8; 512],
    pub spi: T,
    reset_pin: U,
    dc_pin: U
}

impl<T: SpiInterface, U: PinInterface> Display<T, U> {
    pub fn new(spi: T, reset_pin: U, dc_pin: U) -> Display<T, U> {
        Display { buffer: [0u8; 512], spi, reset_pin, dc_pin }
    }

    pub fn init(&mut self) {
        // Based on https://www.waveshare.com/w/upload/b/b5/SSD1305-Revision_1.8.pdf
        self.reset();
        self.send_display_commands(&[0xAE]); // Display OFF (sleep mode)
        self.send_display_commands(&[0x20, 0x01]); // Set Vertical Addressing Mode
        self.send_display_commands(&[0x21, 0x00, 0x7F]); // Set Column Address range to 0-127
        self.send_display_commands(&[0x22, 0x00, 0x03]); // Set Page Address range to 0-3
        self.send_display_commands(&[0xAC]); // Display ON in dim mode
        self.swap();
    }

    pub fn deinit(&mut self) {
        self.fill_with_black();
        self.swap();
        thread::sleep(time::Duration::from_millis(10));
        self.reset_pin.set_pin(false);
    }

    pub fn swap(&mut self) {
        self.dc_pin.set_pin(true);
        self.spi.send_bytes(&self.buffer);
    }

    pub fn fill_with_black(&mut self) {
        self.buffer = [0u8; 512];
    }

    pub fn fill_with_white(&mut self) {
        self.buffer = [255u8; 512];
    }

    pub fn fill_with_pixmap(&mut self, pixmap: &Pixmap) {

    }

    pub fn write_ascii_text(&mut self, text: AsciiText) {

    }

    pub fn get_pixel_color(&self, point: &Point) -> Result<Color, &'static str> {
        let byte = point.y * 4 + (point.x / 8);
        if byte >= 0 && byte < self.buffer.len() as isize {
            let byte_checked: usize = byte as usize;
            let bit_mask = 1u8 << (point.x % 8);
            match self.buffer[byte_checked] & bit_mask > 0 {
                true => return Ok(Color::White),
                false => return Ok(Color::Black)
            }
        }
        Err("Pixel is outside of defined screen, cannot get pixel color.")
    }

    pub fn set_pixel_color(&mut self, point: &Point, color: &Color) {
        let byte = point.y * 4 + (point.x / 8);
        if byte >= 0 && byte < self.buffer.len() as isize {
            let byte_checked: usize = byte as usize;
            let bit_mask = 1u8 << (point.x % 8);
            match color {
                Color::White => self.buffer[byte_checked] |= bit_mask,
                Color::Black => self.buffer[byte_checked] &= !bit_mask
            }
        } else {
            // fail silently
        }
    }

    pub fn toggle_pixel(&mut self, point: &Point) {
        match self.get_pixel_color(&point) {
            Ok(Color::White) => self.set_pixel_color(&point, &Color::Black),
            Ok(Color::Black) => self.set_pixel_color(&point, &Color::White),
            Err(s) => println!("toggle_pixel failed for reason: {}", s)
        }
    }

    pub fn draw_line_with_coords(&mut self, x0: isize, y0: isize, x1: isize, y1: isize, color: &Color) {
        self.draw_line_with_points(&Point{ x: x0, y: y0 }, &Point{ x: x1, y: y1 }, color);
    }

    pub fn draw_line_with_points(&mut self, start: &Point, end: &Point, color: &Color) {
        let dx =  isize::abs(end.x - start.x);
        let sx = if start.x < end.x { 1 } else { -1 };
        let dy = -isize::abs(end.y - start.y);
        let sy = if start.y < end.y { 1 } else { -1 };
        let mut err = dx + dy;
        let mut current: Point = start.clone();
        let mut e2: isize;
        loop {
            self.set_pixel_color(&current, color);
            if current == *end { break; }
            e2 = 2 * err;
            if e2 >= dy { err += dy; current.x += sx; }
            if e2 <= dx { err += dx; current.y += sy; }
        }
    }

    pub fn draw_box_with_coords(&mut self, x0: isize, y0: isize, x1: isize, y1: isize, fill_color: &Color, border_color: &Color) {
        self.draw_box_with_points(
            &Point{ x: std::cmp::min(x0, x1), y: std::cmp::min(y0, y1) },
            &Point{ x: std::cmp::max(x0, x1), y: std::cmp::max(y0, y1) },
            &Color::Black,
            &Color::White
        );
    }

    pub fn draw_box_with_points(&mut self, bottom_left: &Point, top_right: &Point, fill_color: &Color, border_color: &Color) {
        for x in bottom_left.x..(top_right.x + 1) {
            self.set_pixel_color(&Point{x: x, y: bottom_left.y}, border_color);
            self.set_pixel_color(&Point{x: x, y: top_right.y}, border_color);
        }
        for y in (bottom_left.y + 1)..top_right.y {
            self.set_pixel_color(&Point{x: bottom_left.x, y: y}, border_color);
            self.set_pixel_color(&Point{x: top_right.x, y: y}, border_color);
            for x in (bottom_left.x + 1)..top_right.x {
                self.set_pixel_color(&Point{x, y}, fill_color);
            }
        }
    }

    fn send_display_commands(&mut self, commands : &[u8]) {
        self.dc_pin.set_pin(false);
        self.spi.send_bytes(commands);
    }

    fn reset(&mut self) {
        let interval = time::Duration::from_millis(10);
        self.reset_pin.set_pin(true);
        thread::sleep(interval);
        self.reset_pin.set_pin(false);
        thread::sleep(interval);
        self.reset_pin.set_pin(true);
        thread::sleep(interval);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::*;
    use mockall::predicate::*;
    use crate::hardware::{MockSpiInterface, MockPinInterface};

    fn get_new_mocked_display() -> Display<MockSpiInterface, MockPinInterface>  {
        Display::new(
            MockSpiInterface::new(),
            MockPinInterface::new(),
            MockPinInterface::new()
        )
    }

    fn set_pin_expectation(value: bool, pin: &mut MockPinInterface, sequence: &mut Sequence) {
        pin.expect_set_pin()
           .with(eq(value))
           .return_const(())
           .times(1)
           .in_sequence(sequence);
    }

    fn set_send_bytes_expectation(bytes: Vec<u8>, spi: &mut MockSpiInterface, sequence: &mut Sequence) {
        spi.expect_send_bytes()
           .withf(move |send_bytes: &[u8]| send_bytes == bytes)
           .return_const(())
           .times(1)
           .in_sequence(sequence);
    }

    #[test]
    fn test_reset_sequence() {
        let mut display = get_new_mocked_display();

        let mut sequence = Sequence::new();
        set_pin_expectation(true, &mut display.reset_pin, &mut sequence);
        set_pin_expectation(false, &mut display.reset_pin, &mut sequence);
        set_pin_expectation(true, &mut display.reset_pin, &mut sequence);

        display.reset();
    }

    #[test]
    fn test_swap() {
        let mut display = get_new_mocked_display();

        let mut sequence = Sequence::new();
        set_pin_expectation(true, &mut display.dc_pin, &mut sequence);
        set_send_bytes_expectation(vec![0xAAu8; 512], &mut display.spi, &mut sequence);
        set_pin_expectation(true, &mut display.dc_pin, &mut sequence);
        set_send_bytes_expectation(vec![0xCCu8; 512], &mut display.spi, &mut sequence);

        display.buffer = [0xAAu8; 512];
        display.swap();
        display.buffer = [0xCCu8; 512];
        display.swap();
    }

    #[test]
    fn test_display_command_sending() {
        let mut display = get_new_mocked_display();

        let mut sequence = Sequence::new();
        set_pin_expectation(false, &mut display.dc_pin, &mut sequence);
        set_send_bytes_expectation(vec![0xABu8,0xCDu8], &mut display.spi, &mut sequence);

        display.send_display_commands(&[0xABu8,0xCDu8]);
    }

    #[test]
    fn test_init_sequence() {
        let mut display = get_new_mocked_display();

        let mut sequence = Sequence::new();

        // Toggle reset pin
        set_pin_expectation(true, &mut display.reset_pin, &mut sequence);
        set_pin_expectation(false, &mut display.reset_pin, &mut sequence);
        set_pin_expectation(true, &mut display.reset_pin, &mut sequence);

        // Display OFF (sleep mode)
        set_pin_expectation(false, &mut display.dc_pin, &mut sequence);
        set_send_bytes_expectation(vec![0xAEu8], &mut display.spi, &mut sequence);

        // Set Vertical Addressing Mode
        set_pin_expectation(false, &mut display.dc_pin, &mut sequence);
        set_send_bytes_expectation(vec![0x20, 0x01], &mut display.spi, &mut sequence);

        // Set Column Address range to 0-127
        set_pin_expectation(false, &mut display.dc_pin, &mut sequence);
        set_send_bytes_expectation(vec![0x21, 0x00, 0x7F], &mut display.spi, &mut sequence);

        // Set Page Address range to 0-3
        set_pin_expectation(false, &mut display.dc_pin, &mut sequence);
        set_send_bytes_expectation(vec![0x22, 0x00, 0x03], &mut display.spi, &mut sequence);

        // Display ON in dim mode
        set_pin_expectation(false, &mut display.dc_pin, &mut sequence);
        set_send_bytes_expectation(vec![0xAC], &mut display.spi, &mut sequence);

        // Send the buffer with all 0u8 to clear the screen to black
        set_pin_expectation(true, &mut display.dc_pin, &mut sequence);
        set_send_bytes_expectation(vec![0u8; 512], &mut display.spi, &mut sequence);

        display.init();
    }

    #[test]
    fn test_after_display_creation_buffer_is_black() {
        let display = get_new_mocked_display();
        let mut point: Point = Point {x: 0, y: 0};
        for x in 0..32 {
            point.x = x;
            for y in 0..128 {
                point.y = y;
                assert!(display.get_pixel_color(&point) == Ok(Color::Black));
            }
        }
    }

    #[test]
    fn test_after_fill_with_white_buffer_is_white() {
        let mut display = get_new_mocked_display();
        display.fill_with_white();
        let mut point: Point = Point {x: 0, y: 0};
        for x in 0..32 {
            point.x = x;
            for y in 0..128 {
                point.y = y;
                assert!(display.get_pixel_color(&point) == Ok(Color::White));
            }
        }
    }

    #[test]
    fn test_after_fill_with_black_buffer_is_all_black() {
        let mut display = get_new_mocked_display();
        display.fill_with_white();
        display.fill_with_black();
        let mut point: Point = Point {x: 0, y: 0};
        for x in 0..32 {
            point.x = x;
            for y in 0..128 {
                point.y = y;
                assert!(display.get_pixel_color(&point) == Ok(Color::Black));
            }
        }
    }

    #[test]
    fn test_set_and_get_of_pixels() {
        let mut display = get_new_mocked_display();

        let mut points = Vec::<Point>::new();
        points.push(Point{x: 0,y: 0});
        points.push(Point{x: 0,y: 127});
        points.push(Point{x: 31,y: 0});
        points.push(Point{x: 31,y: 127});

        for point in points {
            assert!(display.get_pixel_color(&point) == Ok(Color::Black));
            display.set_pixel_color(&point, &Color::White);
            assert!(display.get_pixel_color(&point) == Ok(Color::White));
            display.set_pixel_color(&point, &Color::Black);
            assert!(display.get_pixel_color(&point) == Ok(Color::Black));
        }
    }

    #[test]
    fn test_pixel_toggle() {
        let mut display = get_new_mocked_display();

        let mut points = Vec::<Point>::new();
        points.push(Point{x: 0,y: 0});
        points.push(Point{x: 0,y: 127});
        points.push(Point{x: 31,y: 0});
        points.push(Point{x: 31,y: 127});

        for point in points {
            assert!(display.get_pixel_color(&point) == Ok(Color::Black));
            display.toggle_pixel(&point);
            assert!(display.get_pixel_color(&point) == Ok(Color::White));
            display.toggle_pixel(&point);
            assert!(display.get_pixel_color(&point) == Ok(Color::Black));
        }
    }

    #[test]
    fn test_draw_boxes() {
        let check_rect = |display: &Display<MockSpiInterface, MockPinInterface>, x0: isize, y0: isize, x1: isize, y1: isize, fill_color: &Color, border_color: &Color|-> bool  {
            let bottom_left = Point{ x: std::cmp::min(x0, x1), y: std::cmp::min(y0, y1) };
            let top_right =  Point{ x: std::cmp::max(x0, x1), y: std::cmp::max(y0, y1) };
            for x in bottom_left.x..(top_right.x + 1) {
                if display.get_pixel_color(&Point{x, y: bottom_left.y}).unwrap().ne(border_color) { return false };
                if display.get_pixel_color(&Point{x, y: top_right.y}).unwrap().ne(border_color) { return false };
            }
            for y in (bottom_left.y + 1)..top_right.y {
                if display.get_pixel_color(&Point{x: bottom_left.x, y}).unwrap().ne(border_color) { return false };
                if display.get_pixel_color(&Point{x: top_right.x, y}).unwrap().ne(border_color) { return false };
                for x in (bottom_left.x + 1)..top_right.x {
                    if display.get_pixel_color(&Point{x, y}).unwrap().ne(fill_color) { return false };
                }
            }
            true
        };

        let mut display = get_new_mocked_display();

        display.draw_box_with_points(&Point{ x: 0, y: 0 },&Point{ x: 31, y: 127 }, &Color::Black,&Color::White);
        assert!(check_rect(&display, 0, 0, 31, 127, &Color::Black, &Color::White));
        display.fill_with_black();

        display.draw_box_with_coords(0, 0, 31, 127, &Color::Black, &Color::White);
        assert!(check_rect(&display, 0, 0, 31, 127, &Color::Black, &Color::White));
        display.fill_with_black();
    }
}
