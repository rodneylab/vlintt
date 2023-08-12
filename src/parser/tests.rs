use crate::parser::{div_ceiling, parse_cue_payload_text, parse_header_value};

#[test]
pub fn test_div_ceiling() {
    assert_eq!(div_ceiling(8, 3), 3);
    assert_eq!(div_ceiling(6, 2), 3);
}

#[test]
pub fn test_parse_cue_payload_text() {
    // arrange
    let line = "Sample cue text.";

    // assert
    assert_eq!(
        parse_cue_payload_text(line),
        Ok(("", vec!["Sample cue text."]))
    );
}

#[test]
pub fn test_parse_header_value() {
    let line = "Kind: captions";
    assert_eq!(parse_header_value(line), Ok(("", "Kind")));

    let line = "Language: en-GB";
    assert_eq!(parse_header_value(line), Ok(("", "Language")));
}
