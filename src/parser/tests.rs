use crate::parser::{parse_cue_payload_text, parse_header_value};

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
