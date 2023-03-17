use base64::Engine;

const RAW: &str = "SGVsbG8sIHdvcmxkIQ==";
const BYTES: &[u8] = &base64_literal::base64_literal!("SGVsbG8sIHdvcmxkIQ==");

#[test]
fn basic() {
    let mut output = String::new();

    base64::engine::general_purpose::STANDARD.encode_string(BYTES, &mut output);

    assert_eq!(RAW, &output);
}
