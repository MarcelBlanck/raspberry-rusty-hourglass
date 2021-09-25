use crate::display::{DisplayBuffer, Color, WIDTH, HEIGHT};

pub fn draw_block_clock(remaining_seconds: u128, frame_buffer: &mut DisplayBuffer) {
    let minutes = 1 + remaining_seconds as isize / 60;
    let seconds = remaining_seconds as isize % 60;
    println!("drawing {}:{}", minutes, seconds);

    let outer_border = 5;
    let inner_border = 2;
    let rects_in_row = 4;
    let rect_area_width = WIDTH - 2 * outer_border;
    println!("rect_area_width {}", rect_area_width);
    let rect_size = (rect_area_width - (rects_in_row  - 1) * inner_border) / rects_in_row;
    let check = (WIDTH - rects_in_row - 1) % rects_in_row;

    for minute in 0..minutes {
        let row = minute / rects_in_row;
        let minute_in_row = minute % rects_in_row;
        let x0 = WIDTH - outer_border - rect_size  *  minute_in_row - (inner_border * minute_in_row);
        let y0 = outer_border + row * rect_size + row * inner_border;
        let x1 = x0 - rect_size;

        let y1 = if minute == minutes - 1 {
            let segment_second_size = 60 / (rect_size + 1);
            let segments_filled = seconds / segment_second_size;
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