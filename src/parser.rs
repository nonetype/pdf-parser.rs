use nom::branch::alt;
use nom::character::complete::hex_digit1;
use nom::sequence::{delimited, tuple};
use nom::{bytes::complete::tag, character::complete::char};

use crate::object::{DictionaryObject, NameObject, Object};
use crate::utils::{
    digit1_u32, take_bracketed, take_till_whitespace, take_while_separator, take_while_whitespace,
};
use crate::{error::ParseResult, object::Header};

impl Header {
    pub fn parse(input: &[u8]) -> ParseResult<Header> {
        let (input, _) = tag(b"%PDF-")(input)?;
        // Take a str digit and convert it to u32.

        let (input, major) = digit1_u32(input)?;
        let (input, _) = char('.')(input)?;
        let (input, minor) = digit1_u32(input)?;
        let (input, _) = take_while_separator(input)?;

        Ok((input, Header { major, minor }))
    }
}

impl<'a> Object<'a> {
    pub fn parse_null(input: &'a [u8]) -> ParseResult<'a, Object<'a>> {
        let (input, _) = tag(b"null")(input)?;
        let (input, _) = take_while_separator(input)?;
        Ok((input, Object::Null))
    }

    pub fn parse_bool(input: &'a [u8]) -> ParseResult<'a, Object<'a>> {
        let (input, result) = crate::utils::bool(input)?;
        let (input, _) = take_while_separator(input)?;
        Ok((input, Object::Boolean(result)))
    }

    pub fn parse_integer(input: &'a [u8]) -> ParseResult<'a, Object<'a>> {
        let (input, result) = crate::utils::digit1_i32(input)?;
        let (input, _) = take_while_separator(input)?;
        Ok((input, Object::Integer(result)))
    }

    pub fn parse_real(input: &'a [u8]) -> ParseResult<'a, Object<'a>> {
        let (input, result) = crate::utils::float_f32(input)?;
        let (input, _) = take_while_separator(input)?;
        Ok((input, Object::Real(result)))
    }

    pub fn parse_numeric(input: &'a [u8]) -> ParseResult<Object<'a>> {
        let (input, result) = alt((Object::parse_integer, Object::parse_real))(input)?;

        Ok((input, result))
    }

    pub fn parse_literal_string(input: &'a [u8]) -> ParseResult<Object<'a>> {
        let (input, value) = delimited(char('('), take_bracketed(b'(', b')'), char(')'))(input)?;
        let (input, _) = take_while_separator(input)?;

        let result = std::str::from_utf8(value).map_err(crate::error::ParseError::UTF8Error)?;
        Ok((input, Object::LiteralString(result)))
    }

    pub fn parse_hexadecimal_string(input: &'a [u8]) -> ParseResult<Object<'a>> {
        let (input, value) = delimited(char('<'), hex_digit1, char('>'))(input)?;
        let (input, _) = take_while_separator(input)?;

        let result = std::str::from_utf8(value).map_err(crate::error::ParseError::UTF8Error)?;
        Ok((input, Object::HexadecimalString(result)))
    }

    pub fn parse_name(input: &'a [u8]) -> ParseResult<Object<'a>> {
        let (input, _) = char('/')(input)?;
        let (input, value) = take_till_whitespace(input)?;
        let (input, _) = take_while_separator(input)?;

        let result = std::str::from_utf8(value).map_err(crate::error::ParseError::UTF8Error)?;
        Ok((input, Object::Name(NameObject(result))))
    }

    pub fn parse_array(input: &'a [u8]) -> ParseResult<Object<'a>> {
        let (outer_input, value) =
            delimited(char('['), take_bracketed(b'[', b']'), char(']'))(input)?;

        let mut elements = Vec::new();
        let (mut inner_input, _) = take_while_whitespace(value)?;

        loop {
            let (input, element) = Object::parse_one(inner_input)?;

            elements.push(element);
            let (input, _) = take_while_whitespace(input)?;
            if input.is_empty() {
                break;
            }

            inner_input = input;
        }

        Ok((outer_input, Object::Array(elements)))
    }

    pub fn parse_dictionary(input: &'a [u8]) -> ParseResult<Object<'a>> {
        let (outer_input, inner_input) =
            delimited(char('<'), take_bracketed(b'<', b'>'), char('>'))(input)?;
        let (_, inner_input) =
            delimited(char('<'), take_bracketed(b'<', b'>'), char('>'))(inner_input)?;

        let mut elements = DictionaryObject::new();
        let (mut inner_input, _) = take_while_whitespace(inner_input)?;

        loop {
            let (input, key_object) = Object::parse_name(inner_input)?;
            let key_name_object = {
                if let Object::Name(name_object) = key_object {
                    name_object
                } else {
                    unreachable!()
                }
            };

            let (input, value_object) = Object::parse_one(input)?;

            elements.insert(key_name_object, value_object);

            let (input, _) = take_while_whitespace(input)?;
            if input.is_empty() {
                break;
            }

            inner_input = input;
        }
        Ok((outer_input, Object::Dictionary(elements)))
    }

    pub fn parse_indirect_reference(input: &'a [u8]) -> ParseResult<Object<'a>> {
        let (input, (id, _, generation, _)) =
            tuple((digit1_u32, char(' '), digit1_u32, tag(" R")))(input)?;
        let (input, _) = take_while_separator(input)?;

        Ok((input, Object::IndirectReference { id, generation }))
    }

    pub fn parse_indirect_object(input: &'a [u8]) -> ParseResult<Object<'a>> {
        let (input, (id, _, generation, _)) =
            tuple((digit1_u32, char(' '), digit1_u32, tag(" obj")))(input)?;
        let (input, _) = take_while_separator(input)?;
        let (input, dictionary) = Object::parse_dictionary(input)?;
        let (input, _) = take_while_separator(input)?;
        let (input, _) = tag("endobj")(input)?;
        let (input, _) = take_while_separator(input)?;
        
        Ok((
            input,
            Object::IndirectObject {
                id,
                generation,
                dictionary: Box::new(dictionary)
            }
        ))
    }

    pub fn parse_comment(input: &'a [u8]) -> ParseResult<&str> {
        let (input, (_, comment, _)) =
            tuple((char('%'), take_till_newline, take_while_separator))(input)?;
        let comment = std::str::from_utf8(comment).map_err(crate::error::ParseError::UTF8Error)?;

        Ok((input, comment))
    }

    pub fn parse_one(input: &'a [u8]) -> ParseResult<Object<'a>> {
        let (input, value_object) = alt((
            Object::parse_indirect_reference,
            Object::parse_dictionary,
            Object::parse_array,
            Object::parse_name,
            Object::parse_literal_string,
            Object::parse_hexadecimal_string,
            Object::parse_numeric,
            Object::parse_bool,
            Object::parse_null,
        ))(input)?;

        Ok((input, value_object))
    }

    pub fn parse(input: &'a [u8]) -> ParseResult<Vec<Object<'a>>> {
        let mut result = Vec::new();
        let mut remaining = input;

        loop {
            match Object::parse_one(remaining) {
                Ok((input, value_object)) => {
                    remaining = input;
                    result.push(value_object);
                }
                Err(_) => break,
            }
        }

        if result.is_empty() {
            Err(nom::Err::Error(crate::error::ParseError::InvalidPDFObject))
        } else {
            Ok((remaining, result))
        }
    }
}

// TODO: implement CrossReferenceTable::parse

// TODO: implement Trailer::parse
