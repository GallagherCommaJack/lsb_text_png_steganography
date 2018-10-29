extern crate image;

use image::{GenericImageView, ImageBuffer};

#[cfg(test)]
mod tests;

mod file_helpers;
mod hider;
mod revealer;

pub fn hide<'a>(payload_path: &str, carrier_path: &'a str) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let payload = file_helpers::get_file_string(payload_path);
    let carrier = image::open(carrier_path).unwrap();

    let (carrier_x_limit, carrier_y_limit) = carrier.dimensions();

    if hider::is_payload_too_large(payload.len(), carrier_x_limit, carrier_y_limit) {
        panic!("Payload is too large for the carrier image");
    }

    let mut img: image::RgbImage = ImageBuffer::new(carrier_x_limit, carrier_y_limit);

    // 3 pixels hold a byte
    // 3 * 3 = 9 (8 for a byte and the last 1 skip)

    // payload -> chars -> bytes
    // for each byte 3 iterations of loop

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let carrier_pixel = carrier.get_pixel(x, y);

        // each of these bits need to be modified to have some of the paylod in it.
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
