pub fn is_payload_too_large(payload_length: u32, image_x_max: u32, image_y_max: u32) -> bool {
    // TODO: This need to take into account the header
    let pixels = image_x_max * image_y_max;
    let pixels_to_hold_a_byte = 3_u32;

    (payload_length * pixels_to_hold_a_byte) > pixels
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tiny_payload_case() {
        assert!(!is_payload_too_large(1, 2, 2))
    }

    #[test]
    fn tiny_payload_case_fail() {
        assert!(is_payload_too_large(2, 2, 2))
    }

    #[test]
    fn perfect_payload() {
        assert!(!is_payload_too_large(3, 3, 3))
    }
}
