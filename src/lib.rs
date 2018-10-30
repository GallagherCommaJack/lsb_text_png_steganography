extern crate image;

use image::{GenericImageView, ImageBuffer};

#[cfg(test)]
mod tests;

mod bit_helpers;
mod file_helpers;
mod hider;
mod revealer;

use bit_helpers:: { change_last_bit, get_bit_at, transform_u32_to_array_of_u8 };

pub fn hide<'a>(payload_path: &str, carrier_path: &'a str) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let payload = file_helpers::get_file_string(payload_path);
    let payload_bytes = payload.as_bytes();
    let carrier = image::open(carrier_path).unwrap();

    let (carrier_x_limit, carrier_y_limit) = carrier.dimensions();

    let number_of_bytes_in_payload = payload_bytes.len() as u32;
    if hider::is_payload_too_large(number_of_bytes_in_payload, carrier_x_limit, carrier_y_limit) {
        panic!("Payload is too large for the carrier image");
    };

    let mut vec: Vec<u8> = Vec::with_capacity((number_of_bytes_in_payload + 4) as usize);
    vec.extend_from_slice(&transform_u32_to_array_of_u8(number_of_bytes_in_payload));
    vec.extend_from_slice(payload_bytes);

    let mut byte_cursor = 8;
    let mut bytes_to_hide = vec.iter();

    let mut img: image::RgbImage = ImageBuffer::new(carrier_x_limit, carrier_y_limit);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let carrier_pixel = carrier.get_pixel(x, y);

        if let Some(&current_byte) = bytes_to_hide.next() {
            if byte_cursor > 7 {
                byte_cursor = 0;
            };

            // each of these bits need to be modified to have some of the payload in it.
            *pixel = image::Rgb([
                change_last_bit(carrier_pixel.data[0], get_bit_at(current_byte, byte_cursor)),
                change_last_bit(carrier_pixel.data[1], get_bit_at(current_byte, byte_cursor + 1)),
                change_last_bit(carrier_pixel.data[2], get_bit_at(current_byte, byte_cursor + 2)),
            ]);
            byte_cursor = byte_cursor + 3;
        } else {
            *pixel = image::Rgb([
                carrier_pixel.data[0],
                carrier_pixel.data[1],
                carrier_pixel.data[2],
            ]);
        }
    }

    img
}

pub fn reveal(carrier_path: &str) -> String {
    let carrier = file_helpers::get_file_string(carrier_path);

    revealer::extract(carrier)
}
