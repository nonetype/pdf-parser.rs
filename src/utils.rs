use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_while, take_while1},
    character::{complete::char, is_newline},
    character::complete::digit1,
    combinator::{eof, recognize},
    error::ErrorKind,
    number::complete::float,
    sequence::pair,
};

use crate::error::{ParseError, ParseResult};

/// What PDF considers white space characters.
pub const WHITE_SPACE_CHARS: [u8; 6] = [0x00, 0x09, 0x0A, 0x0C, 0x0D, 0x20];

pub fn bool(input: &[u8]) -> ParseResult<bool> {
    let (input, res) = alt((tag("true"), tag("false")))(input)?;
    let res = match res {
        b"true" => true,
        b"false" => false,
        _ => unreachable!(),
    };
    Ok((input, res))
}

pub fn digit1_u32(input: &[u8]) -> ParseResult<u32> {
    let (input, digits) = digit1_u32_validate_length(input, 0)?;
    Ok((input, digits))
}

pub fn digit1_u32_validate_length(input: &[u8], length: usize) -> ParseResult<u32> {
    let (input, digits) = digit1(input)?;
    // Convert the &[u8] to a str.
    let digits = std::str::from_utf8(digits).map_err(ParseError::UTF8Error)?;
    if 0 < length && digits.len() != length {
        return Err(nom::Err::Error(ParseError::NomError(ErrorKind::Verify)));
    }
    // Convert the str to a u32.
    let digits = digits.parse::<u32>().map_err(ParseError::ParseIntError)?;
    Ok((input, digits))
}

pub fn digit1_i32(input: &[u8]) -> ParseResult<i32> {
    let (input, digits) = alt((
        recognize(pair(char('+'), digit1)),
        recognize(pair(char('-'), digit1)),
        recognize(digit1),
    ))(input)?;
    // Convert the &[u8] to a str.
    let digits = std::str::from_utf8(digits).map_err(ParseError::UTF8Error)?;
    // Convert the str to a u32.
    let digits = digits.parse::<i32>().map_err(ParseError::ParseIntError)?;
    Ok((input, digits))
}

pub fn float_f32(input: &[u8]) -> ParseResult<f32> {
    let (input, digits) = alt((
        recognize(pair(char('+'), float)),
        recognize(pair(char('-'), float)),
        recognize(float),
    ))(input)?;
    // Convert the &[u8] to a str.
    let digits = std::str::from_utf8(digits).map_err(ParseError::UTF8Error)?;
    // Convert the str to a u32.
    let digits = digits.parse::<f32>().map_err(ParseError::ParseFloatError)?;
    Ok((input, digits))
}

pub fn take_till_whitespace(input: &[u8]) -> ParseResult<&[u8]> {
    take_till(|c| WHITE_SPACE_CHARS.contains(&c))(input)
}

pub fn take_while_whitespace(input: &[u8]) -> ParseResult<&[u8]> {
    take_while(|c| WHITE_SPACE_CHARS.contains(&c))(input)
}

pub fn take_while1_whitespace(input: &[u8]) -> ParseResult<&[u8]> {
    take_while1(|c| WHITE_SPACE_CHARS.contains(&c))(input)
}

// Some objects must be separated by white space and eof.
// e.g., 'true' should be true, 'truee' should return an error.
pub fn take_while_separator(input: &[u8]) -> ParseResult<&[u8]> {
    alt((take_while1_whitespace, eof))(input)
}

pub fn take_till_newline(input: &[u8]) -> ParseResult<&[u8]> {
    take_till(is_newline)(input)
}

// fn take_bracketed<'a>(input: &'a [u8], opening: &'a [u8], closing: &'a [u8]) -> ParseResult<'a, &'a [u8]> {
//     delimited(
//         tag(opening),
//         delimited(multispace0, take_until(closing), multispace0),
//         tag(closing),
//     )(input)
// }

// Code from https://github.com/edg-l/nompdf
pub fn take_bracketed(opening: u8, closing: u8) -> impl Fn(&[u8]) -> ParseResult<&[u8]> {
    move |i: &[u8]| {
        let mut bracket_counter = 0;

        for (index, x) in i.iter().enumerate() {
            match *x {
                x if x == opening => {
                    bracket_counter += 1;
                }
                x if x == closing => {
                    bracket_counter -= 1;
                }
                _ => {}
            }
            if bracket_counter == -1 {
                // We do not consume it.
                return Ok((&i[index..], &i[0..index]));
            };
        }

        if bracket_counter == 0 {
            Ok((b"", i))
        } else {
            Err(ParseError::NomError(ErrorKind::TakeUntil).into())
        }
    }
}
