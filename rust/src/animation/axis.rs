use imageproc::{
    drawing::{draw_line_segment, draw_line_segment_mut, draw_polygon_mut},
    image::{Rgb, RgbImage},
    point::Point,
};

use crate::api::{
    screen::{Screen2D, ScreenLike},
    util::{interpolate, Quality},
};

fn draw_lines(img: &mut RgbImage, color: Rgb<u8>, screen: &Screen2D, quality: &Quality) {
    let (usable_sw, usable_sh) = quality.usable();
    let center = screen.get_center_pixels(quality.resolution());
    draw_line_segment_mut(
        img,
        (center.0, quality.resolution().1 - usable_sh),
        (center.0, usable_sh as f32),
        color,
    );
    draw_line_segment_mut(
        img,
        (quality.resolution().0 - usable_sw as f32, center.1),
        (usable_sw as f32, center.1),
        color,
    );
}

fn draw_arrow_tips(img: &mut RgbImage, color: Rgb<u8>) {
    let height_c: f32 = (img.height() / 2) as f32 + 0.5;
    let width_c: f32 = (img.width() / 2) as f32 + 0.5;
    draw_polygon_mut(
        img,
        &[
            Point::new(1890, height_c as i32),
            Point::new(1870, height_c as i32 + 10),
            Point::new(1870, height_c as i32 - 10),
        ],
        color,
    );
    draw_polygon_mut(
        img,
        &[
            Point::new(width_c as i32, 30),
            Point::new(width_c as i32 - 10, 50),
            Point::new(width_c as i32 + 10, 50),
        ],
        color,
    );
}

fn draw_markers(img: &mut RgbImage, color: Rgb<u8>, screen: &Screen2D, quality: &Quality) {
    let (xstart, xend) = (
        screen.x_axis().0.ceil() as i32,
        screen.x_axis().1.floor() as i32,
    );
    let (ystart, yend) = (
        screen.y_axis().0.ceil() as i32,
        screen.y_axis().1.floor() as i32,
    );

    let pairs: Vec<(f32, f32)> = (ystart..=yend)
        .flat_map(move |y| (xstart..=xend).map(move |x| (x as f32, y as f32)))
        .filter(|(x, y)| (*x == 0.0 || *y == 0.0) && *x != *y)
        .collect();
    for pair in pairs {
        let (x, y) = interpolate(quality, screen, pair);
        if pair.0 == 0.0 {
            draw_line_segment_mut(img, (x, y - 5.0), (x, y + 5.0), color);
        } else {
            draw_line_segment_mut(img, (x - 5.0, y), (x + 5.0, y), color);
        }
    }
}

pub fn draw_axis(img: &mut RgbImage, color: Rgb<u8>, screen: &Screen2D, quality: &Quality) {
    draw_lines(img, color, screen, quality);
    // draw_arrow_tips(img, color);
    draw_markers(img, color, screen, quality);
}
