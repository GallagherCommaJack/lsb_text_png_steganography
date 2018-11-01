extern crate image;

use image::{GenericImageView, ImageBuffer};
use std::str;

#[cfg(test)]
mod tests;

mod file_helpers;
mod hider;
mod revealer;

mod bit_helpers;
use bit_helpers::{change_last_bit, get_bit_at, transform_u32_to_array_of_u8};

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

    let mut pixel_seen_count = 0;
    let mut current_byte = *bytes_to_hide.next().unwrap();

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let carrier_pixel = carrier.get_pixel(x, y);
        pixel_seen_count = pixel_seen_count + 1;

        if pixel_seen_count <= (vec.len() * 3) {
            if byte_cursor > 7 {
                byte_cursor = 0;
            };

            *pixel = image::Rgb([
                change_last_bit(carrier_pixel.data[0], get_bit_at(current_byte, byte_cursor)),
                change_last_bit(
                    carrier_pixel.data[1],
                    get_bit_at(current_byte, byte_cursor + 1),
                ),
                change_last_bit(
                    carrier_pixel.data[2],
                    get_bit_at(current_byte, byte_cursor + 2),
                ),
            ]);
            byte_cursor = byte_cursor + 3;

            if pixel_seen_count % 3 == 0 && pixel_seen_count != (vec.len() * 3) {
                current_byte = *bytes_to_hide.next().unwrap();
            }
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

fn get_number_of_bytes_in_message(four_bytes: &[u8]) -> u32 {
    (((*four_bytes)[0] as u32) << 24)
        + (((*four_bytes)[1] as u32) << 16)
        + (((*four_bytes)[2] as u32) << 8)
        + (((*four_bytes)[3] as u32) << 0)
}

pub fn reveal(carrier_path: &str) -> String {
    // Just wrote but this needs a refactor!

    let carrier = image::open(carrier_path).unwrap();

    let (carrier_x_limit, carrier_y_limit) = carrier.dimensions();

    let message_header_length = 4;

    let mut byte_cursor = 0;
    let mut byte = 0b0000_0000;
    let mut vec: Vec<u8> = Vec::new();

    let mut byte_counter = 0;
    let mut bytes_in_message: u32 = 5;

    'outer: for y in 0..carrier_y_limit {
        for x in 0..carrier_x_limit {
            let carrier_pixel = carrier.get_pixel(x, y);
            for i in 0..3 {
                if byte_counter == bytes_in_message + message_header_length {
                    break 'outer;
                }
                if byte_cursor < 8 {
                    byte |= (get_bit_at(carrier_pixel.data[i], 0) as u8) << byte_cursor;
                    byte_cursor = byte_cursor + 1;
                } else {
                    vec.push(byte);
                    byte_counter = byte_counter + 1;
                    if byte_counter == message_header_length {
                        bytes_in_message = get_number_of_bytes_in_message(vec.as_slice());
                    };
                    byte = 0b0000_0000;
                    byte_cursor = 0;
                }
            }
        }
    }

    String::from(str::from_utf8(&vec[4..]).unwrap())
}
