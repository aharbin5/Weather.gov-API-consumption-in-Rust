use bmp::{Image, Pixel};
use bmp::px as px;

// This is a terrible way to render a line
// I need to learn more about rendering, but I'm actively working to fix this and make this into a usable crate

pub fn line(mut img: Image, start_x_pos: u32, start_y_pos: u32, end_x_pos: u32, end_y_pos: u32) -> Image
{
    let mut slope: f32 = (start_y_pos as f32 - end_y_pos as f32) / (start_x_pos as f32 - end_x_pos as f32);

    if slope < 0 as f32
    {
        slope = (end_y_pos as f32 - start_y_pos as f32) / (end_x_pos as f32 - start_x_pos as f32);
    }

    println!("{}", slope);

    if slope <= 1 as f32 && slope >= 0 as f32
    {
        for x in 0 .. start_x_pos.abs_diff(end_x_pos)
        {
            img.set_pixel(start_x_pos + x,((start_x_pos + x) as f32 * slope) as u32, px!(255,255,255));
        }
    }
    else if slope > 1 as f32
    {
        for y in 0 .. start_y_pos.abs_diff(end_y_pos)
        {
            img.set_pixel((start_x_pos as f32 + (start_y_pos + y) as f32 / slope) as u32, start_y_pos + y, px!(255,255,255));
        }
    }

    img
}

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