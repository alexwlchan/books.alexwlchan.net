use palette::Srgb;
use std::str::FromStr;

// Parses a hex string as an RGB tuple, e.g. #d01c11 ~> (208, 28, 17)
//
// This function assumes the hex string is correctly formatted.
pub fn parse_hex_string(s: &str) -> Srgb<u8> {
    Srgb::from_str(s).unwrap()
}
