use bmp::{Image, Pixel};
use bmp::px as px;

// This is probably the least efficient code I've ever written
// These could be done with line functions in a maximum of 7 lines
// I'll get around to that later, this works for now

pub fn draw_number(number: u32, mut img: Image, x_pos: u32, y_pos: u32) -> Image
{
    match number
    {
        0 => {
            img.set_pixel(x_pos+1, y_pos, px!(255,255,255));
            img.set_pixel(x_pos+2, y_pos, px!(255,255,255));

            img.set_pixel(x_pos, y_pos+1, px!(255,255,255));
            img.set_pixel(x_pos+3, y_pos+1, px!(255,255,255));
            img.set_pixel(x_pos, y_pos+2, px!(255,255,255));
            img.set_pixel(x_pos+3, y_pos+2, px!(255,255,255));
            img.set_pixel(x_pos, y_pos+3, px!(255,255,255));
            img.set_pixel(x_pos+3, y_pos+3, px!(255,255,255));

            img.set_pixel(x_pos+1, y_pos+4, px!(255,255,255));
            img.set_pixel(x_pos+2, y_pos+4, px!(255,255,255));

            img
        },
        1 => {
            img.set_pixel(x_pos+1, y_pos, px!(255,255,255));
            img.set_pixel(x_pos, y_pos+1, px!(255,255,255));
            img.set_pixel(x_pos+1, y_pos+1, px!(255,255,255));
            img.set_pixel(x_pos+1, y_pos+2, px!(255,255,255));
            img.set_pixel(x_pos+1, y_pos+3, px!(255,255,255));
            img.set_pixel(x_pos, y_pos+4, px!(255,255,255));
            img.set_pixel(x_pos+1, y_pos+4, px!(255,255,255));
            img.set_pixel(x_pos+2, y_pos+4, px!(255,255,255));

            img
        },
        2 => {
            img.set_pixel(x_pos, y_pos, px!(255,255,255));
            img.set_pixel(x_pos+1, y_pos, px!(255,255,255));
            img.set_pixel(x_pos+2, y_pos, px!(255,255,255));
            img.set_pixel(x_pos+3, y_pos+1, px!(255,255,255));
            img.set_pixel(x_pos+2, y_pos+2, px!(255,255,255));
            img.set_pixel(x_pos+1, y_pos+2, px!(255,255,255));
            img.set_pixel(x_pos, y_pos+3, px!(255,255,255));
            img.set_pixel(x_pos, y_pos+4, px!(255,255,255));
            img.set_pixel(x_pos+1, y_pos+4, px!(255,255,255));
            img.set_pixel(x_pos+2, y_pos+4, px!(255,255,255));
            img.set_pixel(x_pos+3, y_pos+4, px!(255,255,255));

            img
        },
        3 => {
            img.set_pixel(x_pos, y_pos, px!(255,255,255));
            img.set_pixel(x_pos+1, y_pos, px!(255,255,255));
            img.set_pixel(x_pos+2, y_pos, px!(255,255,255));
            img.set_pixel(x_pos+3, y_pos+1, px!(255,255,255));
            img.set_pixel(x_pos+1, y_pos+2, px!(255,255,255));
            img.set_pixel(x_pos+2, y_pos+2, px!(255,255,255));
            img.set_pixel(x_pos+3, y_pos+3, px!(255,255,255));
            img.set_pixel(x_pos, y_pos+4, px!(255,255,255));
            img.set_pixel(x_pos+1, y_pos+4, px!(255,255,255));
            img.set_pixel(x_pos+2, y_pos+4, px!(255,255,255));

            img
        },
        4 => {
            img.set_pixel(x_pos, y_pos, px!(255,255,255));
            img.set_pixel(x_pos+3, y_pos, px!(255,255,255));
            img.set_pixel(x_pos, y_pos+1, px!(255,255,255));
            img.set_pixel(x_pos+3, y_pos+1, px!(255,255,255));
            img.set_pixel(x_pos, y_pos+2, px!(255,255,255));
            img.set_pixel(x_pos+1, y_pos+2, px!(255,255,255));
            img.set_pixel(x_pos+2, y_pos+2, px!(255,255,255));
            img.set_pixel(x_pos+3, y_pos+2, px!(255,255,255));
            img.set_pixel(x_pos+3, y_pos+3, px!(255,255,255));
            img.set_pixel(x_pos+3, y_pos+4, px!(255,255,255));

            img
        },
        5 => {
            img.set_pixel(x_pos+3, y_pos, px!(255,255,255));
            img.set_pixel(x_pos+2, y_pos, px!(255,255,255));
            img.set_pixel(x_pos+1, y_pos, px!(255,255,255));
            img.set_pixel(x_pos, y_pos, px!(255,255,255));
            img.set_pixel(x_pos, y_pos+1, px!(255,255,255));
            img.set_pixel(x_pos, y_pos+2, px!(255,255,255));
            img.set_pixel(x_pos+1, y_pos+2, px!(255,255,255));
            img.set_pixel(x_pos+2, y_pos+2, px!(255,255,255));
            img.set_pixel(x_pos+3, y_pos+3, px!(255,255,255));
            img.set_pixel(x_pos+2, y_pos+4, px!(255,255,255));
            img.set_pixel(x_pos+1, y_pos+4, px!(255,255,255));
            img.set_pixel(x_pos, y_pos+4, px!(255,255,255));

            img
        },
        6 => {
            img.set_pixel(x_pos+2, y_pos, px!(255,255,255));
            img.set_pixel(x_pos+1, y_pos, px!(255,255,255));
            img.set_pixel(x_pos, y_pos+1, px!(255,255,255));
            img.set_pixel(x_pos, y_pos+2, px!(255,255,255));
            img.set_pixel(x_pos+1, y_pos+2, px!(255,255,255));
            img.set_pixel(x_pos+2, y_pos+2, px!(255,255,255));
            img.set_pixel(x_pos+3, y_pos+3, px!(255,255,255));
            img.set_pixel(x_pos, y_pos+3, px!(255,255,255));
            img.set_pixel(x_pos+1, y_pos+4, px!(255,255,255));
            img.set_pixel(x_pos+2, y_pos+4, px!(255,255,255));

            img
        },
        7 => {
            img.set_pixel(x_pos, y_pos, px!(255,255,255));
            img.set_pixel(x_pos+1, y_pos, px!(255,255,255));
            img.set_pixel(x_pos+2, y_pos, px!(255,255,255));
            img.set_pixel(x_pos+3, y_pos, px!(255,255,255));
            img.set_pixel(x_pos+3, y_pos+1, px!(255,255,255));
            img.set_pixel(x_pos+2, y_pos+2, px!(255,255,255));
            img.set_pixel(x_pos+1, y_pos+3, px!(255,255,255));
            img.set_pixel(x_pos+1, y_pos+4, px!(255,255,255));

            img
        },
        8 => {
            img.set_pixel(x_pos+1, y_pos, px!(255,255,255));
            img.set_pixel(x_pos+2, y_pos, px!(255,255,255));

            img.set_pixel(x_pos, y_pos+1, px!(255,255,255));
            img.set_pixel(x_pos+3, y_pos+1, px!(255,255,255));
            img.set_pixel(x_pos+1, y_pos+2, px!(255,255,255));
            img.set_pixel(x_pos+2, y_pos+2, px!(255,255,255));
            img.set_pixel(x_pos, y_pos+3, px!(255,255,255));
            img.set_pixel(x_pos+3, y_pos+3, px!(255,255,255));

            img.set_pixel(x_pos+1, y_pos+4, px!(255,255,255));
            img.set_pixel(x_pos+2, y_pos+4, px!(255,255,255));

            img
        },
        9 => {
            img.set_pixel(x_pos+1, y_pos, px!(255,255,255));
            img.set_pixel(x_pos+2, y_pos, px!(255,255,255));

            img.set_pixel(x_pos, y_pos+1, px!(255,255,255));
            img.set_pixel(x_pos+3, y_pos+1, px!(255,255,255));
            img.set_pixel(x_pos+1, y_pos+2, px!(255,255,255));
            img.set_pixel(x_pos+2, y_pos+2, px!(255,255,255));
            img.set_pixel(x_pos+3, y_pos+2, px!(255,255,255));
            img.set_pixel(x_pos+3, y_pos+3, px!(255,255,255));

            img.set_pixel(x_pos+1, y_pos+4, px!(255,255,255));
            img.set_pixel(x_pos+2, y_pos+4, px!(255,255,255));

            img
        },
        1_u32..=u32::MAX => todo!(),
    }
}