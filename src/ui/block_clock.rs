use crate::display::{DisplayBuffer, Color, WIDTH, HEIGHT};

pub fn draw_block_clock(remaining_seconds: isize, frame_buffer: &mut DisplayBuffer) {
    let minutes = remaining_seconds / 60;
    let seconds = remaining_seconds % 60;

    let outer_border = 2;
    let inner_border = 2;
    let rects_in_row = 4;
    let rect_area_width = WIDTH - 2 * outer_border;
    let rect_size = (rect_area_width - (rects_in_row  - 1) * inner_border) / rects_in_row;
    let check = (WIDTH - rects_in_row - 1) % rects_in_row;

    for minute in 0..minutes {
        let row = minute / rects_in_row;
        let minute_in_row = minute % rects_in_row;
        let x0 = WIDTH - outer_border - rect_size  *  minute_in_row - (inner_border * minute_in_row);
        let y0 = outer_border + row * rect_size + row * inner_border;
        frame_buffer.draw_box_with_coords(
            x0,
            y0,
            x0 - rect_size,
            y0 + rect_size,
            &Color::White,
            &Color::Black
        );
    }


}