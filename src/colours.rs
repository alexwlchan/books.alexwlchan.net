use palette::Srgb;

// Parses a hex string as an RGB tuple, e.g. #d01c11 ~> (208, 28, 17)
//
// This function assumes the hex string is correctly formatted.
pub fn parse_hex_string(s: &str) -> Srgb<u8> {
    // Can I just use Srgb::from_str here???
    assert_eq!(s.len(), 7);
    let r = u8::from_str_radix(&s[1..3], 16).unwrap();
    let g = u8::from_str_radix(&s[3..5], 16).unwrap();
    let b = u8::from_str_radix(&s[5..7], 16).unwrap();
    Srgb::new(r, g, b)
}
