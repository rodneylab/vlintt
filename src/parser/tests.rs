use crate::parser::parse_header_value;

#[test]
pub fn test_parse_header_value() {
    let line = "Kind: captions";
    assert_eq!(parse_header_value(line), Ok(("", "Kind")));

    let line = "Language: en-GB";
    assert_eq!(parse_header_value(line), Ok(("", "Language")));
}
