extern crate image;

use image::{GenericImageView, ImageBuffer};

#[cfg(test)]
mod tests;

mod file_helpers;
mod hider;
mod revealer;

pub fn hide<'a>(payload_path: &str, carrier_path: &'a str) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let _payload = file_helpers::get_file_string(payload_path);
    let carrier = image::open(carrier_path).unwrap();

    let (carrier_x_limit, carrier_y_limit) = carrier.dimensions();

    let mut img: image::RgbImage = ImageBuffer::new(carrier_x_limit, carrier_y_limit);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let carrier_pixel = carrier.get_pixel(x, y);

        *pixel = image::Rgb([
            carrier_pixel.data[0],
            carrier_pixel.data[1],
            carrier_pixel.data[2],
        ]);
    }

    img
}

pub fn reveal(carrier_path: &str) -> String {
    let carrier = file_helpers::get_file_string(carrier_path);

    revealer::extract(carrier)
}
