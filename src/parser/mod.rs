use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::complete::anychar,
    multi::{fold_many0,separated_list1},
    sequence::tuple,
    IResult,
};
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::Path,
    time::Instant,
};

const TARGET_CUE_PAYLOAD_TEXT_LENGTH: usize = 42;

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

pub fn parse_cue_payload_text(line: &str) -> IResult<&str, Vec<&str>> {
    fold_many0(
        parse_cue_payload_text_line,
        Vec::new,
        |mut accumulator: Vec<_>, item| {
            accumulator.push(item);
            accumulator
        },
    )(line)
}

pub fn parse_cue_payload_text_line(line: &str) -> IResult<&str, &str> {
    let trimmed_line = line.trim();
    let _ = anychar(trimmed_line)?;
    let line_length = trimmed_line.len();
    if line_length < TARGET_CUE_PAYLOAD_TEXT_LENGTH {
        return Ok(("", trimmed_line));
    }
    let last_space = match line[..TARGET_CUE_PAYLOAD_TEXT_LENGTH].rfind(' ') {
        Some(value) => value,
        None => TARGET_CUE_PAYLOAD_TEXT_LENGTH,
    };
    Ok((line[last_space..].trim(), line[..last_space].trim()))
}

pub fn parse_timing_hms(line: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(":"), take_while_m_n(2, 2, is_digit))(line)
}

pub fn parse_timing_milliseconds(line: &str) -> IResult<&str, &str> {
    take_while_m_n(3, 3, is_digit)(line)
}

// 00:00:00.320 or mm:ss.ttt
// hh can be up to 4 digits - todo
pub fn parse_timing(line: &str) -> IResult<&str, &str> {
    let (remaining_line, (parse_timing_hms, _, _)) =
        tuple((parse_timing_hms, tag("."), parse_timing_milliseconds))(line)?;
    if parse_timing_hms.len() == 2 {
        return Ok((remaining_line, &line[..9]));
    }
    Ok((remaining_line, &line[..12]))
}

// 00:00:00.320 --> 00:00:05.920
pub fn parse_cue_timings(line: &str) -> IResult<&str, &str> {
    let (_, (start, _, end)) = tuple((parse_timing, tag(" --> "), parse_timing))(line)?;
    let timing_length = start.len() + 5 + end.len();
    Ok((line, &line[..timing_length]))
}

pub fn parse_vtt_file(input_path: &Path, output_path: &Path, _verbose: bool) {
    println!("[ INFO ] Parsing {:?}...", input_path);
    let start = Instant::now();

    let mut tokens: Vec<String> = Vec::new();
    let file = File::open(input_path).expect("[ ERROR ] Couldn't open that file!");
    let reader = BufReader::new(&file);

    let mut lines_iterator = reader.lines();

    // parse body
    while let Some(line) = lines_iterator.next() {
        let line_content = line.unwrap();
        match parse_cue_timings(&line_content) {
            Ok((_, cue_timings)) => {
                tokens.push(cue_timings.to_string());
                for line in lines_iterator.by_ref() {
                    let line_content = line.unwrap();
                    if let Ok((_, payload_text_lines)) = parse_cue_payload_text(&line_content) {
                        if payload_text_lines.is_empty() {
                            break;
                        }
                        payload_text_lines
                            .into_iter()
                            .for_each(|payload_text_line| {
                                tokens.push(payload_text_line.to_string())
                            });
                    }
                }
                // assume this is the end of the cue and output a new line
                tokens.push("".to_string());
            }
            // assume this is the header block
            Err(_) => tokens.push(line_content.to_string()),
        }
    }

    let mut outfile = match File::create(output_path) {
        Ok(value) => value,
        Err(_) => panic!(
            "[ ERROR ] Was not able to create the output file: {:?}!",
            output_path
        ),
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
