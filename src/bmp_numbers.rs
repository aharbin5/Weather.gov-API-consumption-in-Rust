use bmp::{Image, Pixel};
use bmp::px as px;

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
        /*1 => {},
        2 => {},
        3 => {},
        4 => {},
        5 => {},
        6 => {},
        7 => {},
        8 => {},
        9 => {},*/
        1_u32..=u32::MAX => todo!(),
    }
}