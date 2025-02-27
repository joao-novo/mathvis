use std::f32::consts::PI;

use imageproc::{
    drawing::{draw_filled_circle_mut, draw_line_segment_mut, draw_polygon_mut},
    image::{Rgb, RgbImage},
    point::Point,
};
use rand::distr::{Distribution, StandardUniform};

use crate::api::{
    point::{self, PointLike},
    screen::Screen2D,
    util::{interpolate, Number, Quality},
    vector::Vector,
};

pub fn draw_vector<T>(
    vector: &Vector<T>,
    img: &mut RgbImage,
    color: Rgb<u8>,
    screen: &Screen2D,
    quality: &Quality,
) where
    StandardUniform: Distribution<T>,
    T: Number,
{
    let center = screen.get_center_pixels(quality.resolution());
    let (x, y) = interpolate(
        quality,
        screen,
        (
            vector.values()[0].to_f32().unwrap(),
            vector.values()[1].to_f32().unwrap(),
        ),
    );
    draw_line_segment_mut(img, center, (x, y), color);
    draw_vector_tip(vector, img, color, screen, quality);
}

pub fn rotate(point: &Point<f32>, angle: f32, rotation_center: &Point<f32>) -> Point<f32> {
    let new_x = (point.x - rotation_center.x) * angle.cos()
        - (point.y - rotation_center.y) * angle.sin()
        + rotation_center.x;
    let new_y = (point.x - rotation_center.x) * angle.sin()
        + (point.y - rotation_center.y) * angle.cos()
        + rotation_center.y;
    Point::new(new_x, new_y)
}

pub fn draw_vector_tip<T>(
    vector: &Vector<T>,
    img: &mut RgbImage,
    color: Rgb<u8>,
    screen: &Screen2D,
    quality: &Quality,
) where
    T: Number,
    StandardUniform: Distribution<T>,
{
    let (a, b) = (
        vector.values()[0].to_f32().unwrap(),
        vector.values()[1].to_f32().unwrap(),
    );
    let (a1, a2) = (
        rotate(
            &Point::new(a, b),
            2.0 * PI / 3.0,
            &Point::new(0.95 * a, 0.95 * b),
        ),
        rotate(
            &Point::new(a, b),
            4.0 * PI / 3.0,
            &Point::new(0.95 * a, 0.95 * b),
        ),
    );
    let p1 = point::Point::<f32>::new(vec![a1.x, a1.y]).unwrap();
    let p2 = point::Point::<f32>::new(vec![a2.x, a2.y]).unwrap();
    let (x, y) = interpolate(quality, screen, (a, b));
    let (x1, y1) = interpolate(quality, screen, (p1.values()[0], p1.values()[1]));
    let (x2, y2) = interpolate(quality, screen, (p2.values()[0], p2.values()[1]));

    draw_polygon_mut(
        img,
        &[
            Point::new(x as i32, y as i32),
            Point::new(x1 as i32, y1 as i32),
            Point::new(x2 as i32, y2 as i32),
        ],
        color,
    );
}
