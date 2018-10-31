use super::*;

#[test]
fn hide_payload() {
    let payload_path = "./src/tests/texts/payload_tiny.txt";
    let carrier_path = "./src/tests/images/stripes.png";
    let output_path = "./src/test.png";

    let img = hide(payload_path, carrier_path);

    img.save(output_path).unwrap();
    // Need to test that we have actually put the text in!
    assert_eq!(img.dimensions(), (10, 10));
    //assert_eq!(img.dimensions(), (340, 148));
}

#[test]
fn reveal_hidden_text() {
    let carrier_path = "./src/tests/images/test.png";
    let _text = reveal(carrier_path);
    assert!(false);
}
