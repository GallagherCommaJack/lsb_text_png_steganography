use super::*;

#[test]
fn hide_payload() {
    // hide
    let payload_path = "./src/tests/texts/payload_tiny.txt";
    let carrier_path = "./src/tests/images/stripes.png";
    let output_path = "./src/tests/images/output/test-intermediate.png";

    let img = hide(payload_path, carrier_path);
    img.save(output_path).unwrap();

    // reveal
    let text = reveal(output_path);
    assert_eq!(text, "12345678");
    assert_eq!(img.dimensions(), (10, 10));
}

#[test]
fn reveal_hidden_text() {
    let carrier_path = "./src/tests/images/test.png";
    let text = reveal(carrier_path);
    assert_eq!(text, "12345678");
}
