use crate::gui::display_control::{Color, DisplayBuffer, HEIGHT, WIDTH};

pub fn draw_block_clock(remaining_seconds: u128, frame_buffer: &mut DisplayBuffer) {
    let minutes = 1 + remaining_seconds as isize / 60;
    let seconds = remaining_seconds as isize % 60;

    // TODO maybe not magic numbers
    let rect_size = 6;
    let rect_x_positions = vec![30, 22, 14, 6];
    let x_offset = 1;
    let rect_per_row: usize = 4;

    for minute in 0..minutes {
        let row = minute / rect_per_row as isize;
        let minute_in_row = minute % rect_per_row as isize;
        let x0 = rect_x_positions[minute as usize % rect_per_row] + x_offset;
        let y0 = 1 + row * rect_size + row * 2;
        let x1 = x0 - rect_size;

        let y1 = if minute == minutes - 1 {
            let segments_filled =
                f32::floor((1 + rect_size) as f32 * seconds as f32 / 60f32) as isize;
            let blinking_subtractor = if seconds % 2 == 1 { 0 } else { 1 };
            y0 + segments_filled - blinking_subtractor
        } else {
            y0 + rect_size
        };

        if y1 >= y0 {
            frame_buffer.draw_box_with_coords(x0, y0, x1, y1, &Color::White, &Color::White);
        }
    }
}
