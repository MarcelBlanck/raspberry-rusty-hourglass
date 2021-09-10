#[derive(Eq, PartialEq, Clone)]
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
