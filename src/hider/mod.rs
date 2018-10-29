pub fn is_payload_too_large(_payload_length: usize, _image_x_max: u32, _image_y_max: u32) -> bool {
  // TODO: make this a real calculations
  false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tiny_payload_case() {
      assert!(!is_payload_too_large(1, 2, 2))
    }
}
