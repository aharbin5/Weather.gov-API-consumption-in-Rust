use bmp::{Image, Pixel};
use bmp::px as px;

pub fn line(mut img: Image, start_x_pos: u32, start_y_pos: u32, end_x_pos: u32, end_y_pos: u32) -> Image
{
    let slope: f32;

    if start_x_pos > end_x_pos || start_y_pos > end_y_pos
    {
        slope = ((end_y_pos as f32) - (start_y_pos as f32)) / ((end_x_pos as f32) - (start_x_pos as f32));
    }
    else
    {
        slope = ((start_y_pos as f32) - (end_y_pos as f32)) / ((start_x_pos as f32) - (end_x_pos as f32));
    }

    println!("{}", slope);

    for i in 0 .. (start_x_pos as f32 - end_x_pos as f32).abs() as u32
    {
        // y = mx
        img.set_pixel(i + start_x_pos, ((slope * i as f32) + start_y_pos as f32) as u32, px!(255,255,255));
    }

    
    // Something's wrong with this.  It draws the line backwards after the x formula has already drawn it forwards
    for i in 0 .. (start_y_pos as f32 - end_y_pos as f32).abs() as u32
    {
        // (y) / m = x
        img.set_pixel(((i as f32 / slope) + start_x_pos as f32) as u32, i + start_y_pos, px!(255,255,255));
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