use crate::parser::{is_digit, parse_cue_payload_text, parse_header_value, parse_timing_hms};

#[test]
pub fn is_digit_returns_false_for_non_digit() {
    // arrange
    let character = 'a';

    // act
    let result = is_digit(character);

    // assert
    assert!(!result);
}

#[test]
pub fn is_digit_returns_true_for_digit() {
    // arrange
    let character = '8';

    // act
    let result = is_digit(character);

    // assert
    assert!(result);
}

#[test]
pub fn parse_timing_hms_parses_valid_timestamp() {
    // arrange
    let timestamp = "01:34:21.0000";

    // act
    let result = parse_timing_hms(timestamp);

    // assert
    assert_eq!(result, Ok((".0000", vec!["01", "34", "21"])));
}

#[test]
pub fn test_parse_cue_payload_text_sticks_to_max_width_where_possible() {
    // arrange
    let line = "So, these are the people who actually work at the organization. You've got a name for each of them.";

    // act
    let result = parse_cue_payload_text(line);

    // assert
    assert_eq!(
        result,
        vec![
            "So, these are the people who actually work",
            "at the organization. You've got a name for",
            "each of them."
        ]
    );
    assert_eq!(result.len(), 3);
    assert!(result.iter().all(|val| val.len() <= 42));
}

#[test]
pub fn test_parse_cue_payload_text_stretches_max_width_where_needed() {
    // arrange
    let line = "So, these are the people who actually work at the organization. For each of them, we have a name, as well as a job role and email.";

    // act
    let result = parse_cue_payload_text(line);

    // assert
    assert_eq!(
        result,
        vec![
            "So, these are the people who actually work at",
            "the organization. For each of them, we have a",
            "name, as well as a job role and email."
        ]
    );
    assert_eq!(result.len(), 3);
    assert!(result.iter().all(|val| val.len() <= 45));
}

#[test]
pub fn test_parse_header_value() {
    let line = "Kind: captions";
    assert_eq!(parse_header_value(line), Ok(("", "Kind")));

    let line = "Language: en-GB";
    assert_eq!(parse_header_value(line), Ok(("", "Language")));
}
