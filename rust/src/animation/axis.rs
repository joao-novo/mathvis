use imageproc::{
    drawing::{draw_line_segment, draw_line_segment_mut, draw_polygon_mut},
    image::{Rgb, RgbImage},
    point::Point,
};

fn draw_lines(img: &mut RgbImage, color: Rgb<u8>) {
    let height_c: f32 = (img.height() / 2) as f32 + 0.5;
    let width_c: f32 = (img.width() / 2) as f32 + 0.5;
    draw_line_segment_mut(img, (width_c, 0.0 + 30.0), (width_c, 1080.0 - 30.0), color);
    draw_line_segment_mut(
        img,
        (0.0 + 30.0, height_c),
        (1920.0 - 30.0, height_c),
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

fn draw_markers(img: &mut RgbImage, color: Rgb<u8>) {
    let height_c: f32 = (img.height() / 2) as f32 + 0.5;
    let width_c: f32 = (img.width() / 2) as f32 + 0.5;
    for x in (70..img.width() as i32 - 70).step_by(30) {
        draw_line_segment_mut(
            img,
            (x as f32, height_c - 10.0),
            (x as f32, height_c + 10.0),
            color,
        )
    }
    for y in (70 as i32..img.height() as i32 - 70).step_by(30) {
        draw_line_segment_mut(
            img,
            (width_c - 10.0, y as f32),
            (width_c + 10.0, y as f32),
            color,
        )
    }
}

pub fn draw_axis(img: &mut RgbImage, color: Rgb<u8>) {
    draw_lines(img, color);
    draw_arrow_tips(img, color);
    draw_markers(img, color);
}
