
pub const WIDTH: isize = 32;
pub const HEIGHT: isize = 128;
const DISPLAY_BUFFER_SIZE: usize = 512;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Point {
    pub x: isize,
    pub y: isize
}

#[derive(Eq, PartialEq)]
pub enum Color {
    Black,
    White
}

pub struct Pixmap {
    data: Vec<u8>,
    width: usize,
    height: usize
}

pub struct AsciiFont {
    characters: Vec<Pixmap>
}

pub struct AsciiText {
    text: String,
    font: AsciiFont,
    bottom_left: Point,
    spacing: isize,
    invert: bool
}

pub trait DisplayControl {
    fn init(&mut self);
    fn deinit(&mut self);
    fn swap(&mut self);
    fn fb<'a>(&'a mut self) -> &'a mut DisplayBuffer;
}

pub struct DisplayBuffer {
    pub buffer: [u8; DISPLAY_BUFFER_SIZE]
}

impl DisplayBuffer {
    pub fn new() -> Self {
        DisplayBuffer { buffer: [0u8; DISPLAY_BUFFER_SIZE] }
    }

    pub fn fill_with_black(&mut self) {
        self.buffer = [0u8; DISPLAY_BUFFER_SIZE];
    }

    pub fn fill_with_white(&mut self) {
        self.buffer = [255u8; DISPLAY_BUFFER_SIZE];
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
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_after_display_creation_buffer_is_black() {
        let display = DisplayBuffer::new();
        let mut point: Point = Point {x: 0, y: 0};
        for x in 0..WIDTH {
            point.x = x;
            for y in 0..HEIGHT {
                point.y = y;
                assert!(display.get_pixel_color(&point) == Ok(Color::Black));
            }
        }
    }

    #[test]
    fn test_after_fill_with_white_buffer_is_white() {
        let mut display = DisplayBuffer::new();
        display.fill_with_white();
        let mut point: Point = Point {x: 0, y: 0};
        for x in 0..WIDTH {
            point.x = x;
            for y in 0..HEIGHT {
                point.y = y;
                assert!(display.get_pixel_color(&point) == Ok(Color::White));
            }
        }
    }

    #[test]
    fn test_after_fill_with_black_buffer_is_all_black() {
        let mut display = DisplayBuffer::new();
        display.fill_with_white();
        display.fill_with_black();
        let mut point: Point = Point {x: 0, y: 0};
        for x in 0..WIDTH {
            point.x = x;
            for y in 0..HEIGHT {
                point.y = y;
                assert!(display.get_pixel_color(&point) == Ok(Color::Black));
            }
        }
    }

    #[test]
    fn test_set_and_get_of_pixels() {
        let mut display = DisplayBuffer::new();

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
        let mut display = DisplayBuffer::new();

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
        let check_rect = |display: &DisplayBuffer, x0: isize, y0: isize, x1: isize, y1: isize, fill_color: &Color, border_color: &Color|-> bool  {
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

        let mut display = DisplayBuffer::new();

        display.draw_box_with_points(&Point{ x: 0, y: 0 },&Point{ x: 31, y: 127 }, &Color::Black,&Color::White);
        assert!(check_rect(&display, 0, 0, 31, 127, &Color::Black, &Color::White));
        display.fill_with_black();

        display.draw_box_with_coords(0, 0, 31, 127, &Color::Black, &Color::White);
        assert!(check_rect(&display, 0, 0, 31, 127, &Color::Black, &Color::White));
        display.fill_with_black();
    }
}
