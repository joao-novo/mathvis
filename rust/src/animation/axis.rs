use imageproc::{
    drawing::{draw_line_segment, draw_line_segment_mut, draw_polygon_mut},
    image::{Rgb, RgbImage},
    point::Point,
};

use crate::api::{
    point::PointLike,
    screen::{Screen2D, ScreenLike},
    util::{interpolate, Quality},
};

fn draw_lines(img: &mut RgbImage, color: Rgb<u8>, screen: &Screen2D, quality: &Quality) {
    let usable_res = quality.usable();
    let center = screen.get_center_pixels(quality.resolution());
    draw_line_segment_mut(
        img,
        (
            center.0,
            quality.resolution().values()[1] - usable_res.values()[1],
        ),
        (center.0, usable_res.values()[1]),
        color,
    );
    draw_line_segment_mut(
        img,
        (
            quality.resolution().values()[0] - usable_res.values()[0],
            center.1,
        ),
        (usable_res.values()[0], center.1),
        color,
    );
}

fn draw_arrow_tips(img: &mut RgbImage, color: Rgb<u8>, screen: &Screen2D, quality: &Quality) {
    let center = screen.get_center_pixels(quality.resolution());
    let usable = quality.usable();

    draw_polygon_mut(
        img,
        &[
            Point::new(usable.values()[0] as i32, center.1 as i32),
            Point::new(usable.values()[0] as i32 - 20, center.1 as i32 + 10),
            Point::new(usable.values()[0] as i32 - 20, center.1 as i32 - 10),
        ],
        color,
    );
    draw_polygon_mut(
        img,
        &[
            Point::new(
                center.0 as i32,
                (quality.resolution().values()[1] - usable.values()[1]) as i32,
            ),
            Point::new(
                center.0 as i32 - 10,
                (quality.resolution().values()[1] - usable.values()[1]) as i32 + 20,
            ),
            Point::new(
                center.0 as i32 + 10,
                (quality.resolution().values()[1] - usable.values()[1]) as i32 + 20,
            ),
        ],
        color,
    );
}

fn draw_markers(img: &mut RgbImage, color: Rgb<u8>, screen: &Screen2D, quality: &Quality) {
    let (xstart, xend) = (
        screen.x_axis().0.ceil() as i32 + 1,
        screen.x_axis().1.floor() as i32 - 1,
    );
    let (ystart, yend) = (
        screen.y_axis().0.ceil() as i32 + 1,
        screen.y_axis().1.floor() as i32 - 1,
    );

    let pairs: Vec<(f32, f32)> = (ystart..=yend)
        .flat_map(move |y| (xstart..=xend).map(move |x| (x as f32, y as f32)))
        .filter(|(x, y)| (*x == 0.0 || *y == 0.0) && *x != *y)
        .collect();
    for pair in pairs {
        let (x, y) = interpolate(quality, screen, pair);
        if pair.1 == 0.0 {
            draw_line_segment_mut(img, (x, y - 10.0), (x, y + 10.0), color);
        } else {
            draw_line_segment_mut(img, (x - 10.0, y), (x + 10.0, y), color);
        }
    }
}

pub fn draw_axis(img: &mut RgbImage, color: Rgb<u8>, screen: &Screen2D, quality: &Quality) {
    draw_lines(img, color, screen, quality);
    draw_arrow_tips(img, color, screen, quality);
    draw_markers(img, color, screen, quality);
}
