use imageproc::{
    drawing::draw_filled_rect_mut,
    image::{Rgb, RgbImage},
    rect::Rect,
};

pub fn fill_background(img: &mut RgbImage) {
    draw_filled_rect_mut(
        img,
        Rect::at(0, 0).of_size(img.width(), img.height()),
        Rgb([43, 42, 51]),
    );
}
