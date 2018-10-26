use super::*;

#[test]
fn hide_payload() {
    let payload_path = "./src/tests/texts/small.txt";
    let carrier_path = "./src/tests/texts/carrier_sonnet.png";

    let _image = hide(payload_path, carrier_path);
    assert!(false);
}

#[test]
fn reveal_hidden_text() {
    let carrier_path = "./src/tests/texts/hidden.png";
    let _text = reveal(carrier_path);
    assert!(false);
}
