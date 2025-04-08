#[cfg(test)]
mod tests;

use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::complete::alphanumeric1,
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use std::{
    borrow::Cow,
    fs::File,
    io::{BufRead, BufReader, Write},
    path::Path,
    time::Instant,
};

const TARGET_CUE_PAYLOAD_TEXT_LENGTH: usize = 42;
const MAX_CUE_PAYLOAD_TEXT_OVERFLOW: usize = 3;
const TARGET_CUE_PAYLOAD_TEXT_MAX_LINES: usize = 3;

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

/**
 * Wrap text trying to balance lines, so they are approximately equal in length, where possible.
 * Initially, limit line length to `TARGET_CUE_PAYLOAD_TEXT_LENGTH`, but if this results in the
 * payload requiring more than `TARGET_CUE_PAYLOAD_TEXT_MAX_LINES`, incrementally relax the width
 * limit, one character at a time, up to `MAX_CUE_PAYLOAD_TEXT_OVERFLOW` characters over the target.
 */
fn wrap_line(line: &str) -> Vec<Cow<'_, str>> {
    let mut result = textwrap::wrap(line, TARGET_CUE_PAYLOAD_TEXT_LENGTH);
    let mut overflow = 1;
    while result.len() > TARGET_CUE_PAYLOAD_TEXT_MAX_LINES
        && overflow <= MAX_CUE_PAYLOAD_TEXT_OVERFLOW
    {
        result = textwrap::wrap(line, TARGET_CUE_PAYLOAD_TEXT_LENGTH + overflow);
        overflow += 1;
    }
    result
}

pub fn parse_cue_payload_text(line: &str) -> Vec<Cow<'_, str>> {
    let trimmed_line = line.trim();
    wrap_line(trimmed_line)
}

pub fn parse_timing_hms(line: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(":"), take_while_m_n(2, 2, is_digit)).parse(line)
}

pub fn parse_timing_milliseconds(line: &str) -> IResult<&str, &str> {
    take_while_m_n(3, 3, is_digit)(line)
}

// 00:00:00.320 or mm:ss.ttt
// hh can be up to 4 digits - todo
pub fn parse_timing(line: &str) -> IResult<&str, &str> {
    let (remaining_line, (timing_hms, _, _)) =
        (parse_timing_hms, tag("."), parse_timing_milliseconds).parse(line)?;
    if timing_hms.len() == 2 {
        return Ok((remaining_line, &line[..9]));
    }
    Ok((remaining_line, &line[..12]))
}

// 00:00:00.320 --> 00:00:05.920
pub fn parse_cue_timings(line: &str) -> IResult<&str, &str> {
    let (_, (start, _, end)) = (parse_timing, tag(" --> "), parse_timing).parse(line)?;
    let timing_length = start.len() + 5 + end.len();
    Ok((line, &line[..timing_length]))
}

pub fn parse_header_value(line: &str) -> IResult<&str, &str> {
    let (_, (key, _value)) = separated_pair(alphanumeric1, tag(": "), alphanumeric1).parse(line)?;
    Ok(("", key))
}

pub fn parse_vtt_file(input_path: &Path, output_path: &Path, _verbose: bool) {
    println!("[ INFO ] Parsing {}...", input_path.display());
    let start = Instant::now();

    let mut tokens: Vec<String> = Vec::new();
    let file = File::open(input_path).expect("[ ERROR ] Couldn't open that file!");
    let reader = BufReader::new(&file);
    let mut lines_iterator = reader.lines();
    let mut has_kind_header_value_set = false;
    let mut has_language_header_value_set = false;

    // parse body
    while let Some(line) = lines_iterator.next() {
        let line_content = line.unwrap();

        if let Ok((_, cue_timings)) = parse_cue_timings(&line_content) {
            if !has_kind_header_value_set || !has_language_header_value_set {
                if !has_kind_header_value_set {
                    tokens.push("Kind: captions".to_string());
                    has_kind_header_value_set = true;
                }
                if !has_language_header_value_set {
                    tokens.push("Language: en-GB".to_string());
                    has_language_header_value_set = true;
                }
                tokens.push(String::new());
            }
            tokens.push(cue_timings.to_string());
            for line in lines_iterator.by_ref() {
                let line_content = line.unwrap();
                let payload_text_lines = parse_cue_payload_text(&line_content);
                if payload_text_lines.is_empty() {
                    break;
                }
                payload_text_lines
                    .into_iter()
                    .for_each(|payload_text_line| tokens.push(payload_text_line.to_string()));
            }
            // assume this is the end of the cue and output a new line
            tokens.push(String::new());
        } else {
            if !has_kind_header_value_set || !has_language_header_value_set {
                match parse_header_value(&line_content) {
                    Ok((_, "Kind")) => {
                        has_kind_header_value_set = true;
                    }
                    Ok((_, "Language")) => {
                        has_language_header_value_set = true;
                    }
                    Ok((_, &_)) | Err(_) => {}
                }
            }
            if !has_kind_header_value_set
                && !has_language_header_value_set
                && !line_content.is_empty()
            {
                tokens.push(line_content.to_string());
            }
        }
    }

    let Ok(mut outfile) = File::create(output_path) else {
        panic!(
            "[ ERROR ] Was not able to create the output file: {}!",
            output_path.display()
        );
    };

    for line in &tokens {
        outfile
            .write_all(line.as_bytes())
            .expect("[ ERROR ] Was not able to create the output file!");
        outfile
            .write_all(b"\n")
            .expect("[ ERROR ] Was not able to create the output file!");
    }

    let duration = start.elapsed();
    let duration_milliseconds = duration.as_millis();
    let duration_microseconds = duration.as_micros() - (duration_milliseconds * 1000);
    let file_size = file.metadata().unwrap().len() / 1000;
    println!("[ INFO ] Parsing complete ({file_size} KB) in {duration_milliseconds}.{duration_microseconds:0>3} ms.");
}
