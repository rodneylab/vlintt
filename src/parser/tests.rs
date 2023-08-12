use crate::parser::{parse_cue_payload_text, parse_header_value};

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
