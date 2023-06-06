use nom::branch::alt;
use nom::bytes::complete::take_until;
use nom::character::complete::{digit1, hex_digit1, newline};
use nom::combinator::verify;
use nom::multi::{many0, many1, many_m_n};
use nom::sequence::{delimited, tuple};
use nom::{bytes::complete::tag, character::complete::char};

use crate::error::ParseError;
use crate::object::{
    CrossReferenceEntry, CrossReferenceTable, DictionaryObject, NameObject, Object, Trailer, PDF,
};
use crate::utils::{
    digit1_u32, digit1_u32_validate_length, take_bracketed, take_till_newline,
    take_till_whitespace, take_while_separator, take_while_whitespace,
};
use crate::{error::ParseResult, object::Header};

impl<'a> PDF<'a> {
    pub fn parse(input: &'a [u8]) -> ParseResult<'a, PDF<'a>> {
        let (input, header) = Header::parse(input)?;
        let (input, objects) = many0(Object::parse_body)(input)?;
        let (input, xref_table) = many0(CrossReferenceTable::parse)(input)?;
        let (input, trailer) = Trailer::parse(input)?;

        Ok((
            input,
            Self {
                header,
                body: objects,
                cross_reference_tables: xref_table,
                trailer,
            },
        ))
    }
}

impl Header {
    pub fn parse(input: &[u8]) -> ParseResult<Header> {
        let (input, (_, major, _, minor, _)) = tuple((
            tag(b"%PDF-"),
            digit1_u32,
            char('.'),
            digit1_u32,
            take_while_separator,
        ))(input)?;

        Ok((input, Header { major, minor }))
    }
}

impl<'a> Object<'a> {
    pub fn parse_null(input: &'a [u8]) -> ParseResult<'a, Object<'a>> {
        let (input, _) = tuple((tag(b"null"), take_while_separator))(input)?;
        Ok((input, Object::Null))
    }

    pub fn parse_bool(input: &'a [u8]) -> ParseResult<'a, Object<'a>> {
        let (input, (result, _)) = tuple((crate::utils::bool, take_while_separator))(input)?;
        Ok((input, Object::Boolean(result)))
    }

    pub fn parse_integer(input: &'a [u8]) -> ParseResult<'a, Object<'a>> {
        let (input, (result, _)) = tuple((crate::utils::digit1_i32, take_while_separator))(input)?;
        Ok((input, Object::Integer(result)))
    }

    pub fn parse_real(input: &'a [u8]) -> ParseResult<'a, Object<'a>> {
        let (input, (result, _)) = tuple((crate::utils::float_f32, take_while_separator))(input)?;
        Ok((input, Object::Real(result)))
    }

    pub fn parse_numeric(input: &'a [u8]) -> ParseResult<Object<'a>> {
        let (input, result) = alt((Object::parse_integer, Object::parse_real))(input)?;

        Ok((input, result))
    }

    pub fn parse_literal_string(input: &'a [u8]) -> ParseResult<Object<'a>> {
        let (input, (value, _)) = tuple((
            delimited(char('('), take_bracketed(b'(', b')'), char(')')),
            take_while_separator,
        ))(input)?;
        let result = std::str::from_utf8(value).map_err(crate::error::ParseError::UTF8Error)?;

        Ok((input, Object::LiteralString(result)))
    }

    pub fn parse_hexadecimal_string(input: &'a [u8]) -> ParseResult<Object<'a>> {
        let (input, (value, _)) = tuple((
            delimited(char('<'), hex_digit1, char('>')),
            take_while_separator,
        ))(input)?;
        let result = std::str::from_utf8(value).map_err(crate::error::ParseError::UTF8Error)?;

        Ok((input, Object::HexadecimalString(result)))
    }

    pub fn parse_name(input: &'a [u8]) -> ParseResult<Object<'a>> {
        let (input, (_, value, _)) =
            tuple((char('/'), take_till_whitespace, take_while_separator))(input)?;
        let result = std::str::from_utf8(value).map_err(crate::error::ParseError::UTF8Error)?;

        Ok((input, Object::Name(NameObject(result))))
    }

    pub fn parse_array(input: &'a [u8]) -> ParseResult<Object<'a>> {
        let (outer_input, value) =
            delimited(char('['), take_bracketed(b'[', b']'), char(']'))(input)?;

        let mut elements = Vec::new();
        let (mut inner_input, _) = take_while_whitespace(value)?;

        loop {
            let (input, element) = Object::parse(inner_input)?;

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

        if !inner_input.is_empty() {
            loop {
                let (input, key_object) = Object::parse_name(inner_input)?;
                let key_name_object = {
                    if let Object::Name(name_object) = key_object {
                        name_object
                    } else {
                        unreachable!()
                    }
                };

                let (input, value_object) = Object::parse(input)?;

                elements.insert(key_name_object, value_object);

                let (input, _) = take_while_whitespace(input)?;
                if input.is_empty() {
                    break;
                }

                inner_input = input;
            }
        }

        let (outer_input, _) = take_while_whitespace(outer_input)?;
        let (outer_input, stream) = many0(Object::parse_stream)(outer_input)?;
        assert!(stream.len() <= 1);
        let stream = *stream.first().unwrap_or(&"");

        Ok((outer_input, Object::Dictionary(elements, stream)))
    }

    pub fn parse_stream(input: &'a [u8]) -> ParseResult<&'a str> {
        let (input, (_, _, _, stream, _, _)) = tuple((
            tag("stream"),
            take_till_whitespace,
            newline,
            take_until("endstream"),
            tag("endstream"),
            take_while_separator,
        ))(input)?;
        let stream = std::str::from_utf8(stream).map_err(crate::error::ParseError::UTF8Error)?;

        Ok((input, stream))
    }

    pub fn parse_indirect_reference(input: &'a [u8]) -> ParseResult<Object<'a>> {
        let (input, (id, _, generation, _, _)) = tuple((
            digit1_u32,
            char(' '),
            digit1_u32,
            tag(" R"),
            take_while_separator,
        ))(input)?;

        Ok((input, Object::IndirectReference { id, generation }))
    }

    pub fn parse_indirect_object(input: &'a [u8]) -> ParseResult<Object<'a>> {
        let (input, (id, _, generation, _, _, dictionary, _, _)) = tuple((
            digit1_u32,
            char(' '),
            digit1_u32,
            tag(" obj"),
            take_while_separator,
            Object::parse_dictionary,
            tag("endobj"),
            take_while_separator,
        ))(input)?;

        Ok((
            input,
            Object::IndirectObject {
                id,
                generation,
                dictionary: Box::new(dictionary),
            },
        ))
    }

    pub fn parse_comment(input: &'a [u8]) -> ParseResult<Object<'a>> {
        let (input, (_, comment, _)) =
            tuple((char('%'), take_till_newline, take_while_separator))(input)?;
        let comment = std::str::from_utf8(comment).map_err(crate::error::ParseError::UTF8Error)?;

        Ok((input, Object::Comment(comment)))
    }

    pub fn parse(input: &'a [u8]) -> ParseResult<Object<'a>> {
        let (input, value_object) = alt((
            Object::parse_indirect_object,
            Object::parse_comment,
            Object::parse_dictionary,
            Object::parse_array,
            Object::parse_indirect_reference,
            Object::parse_name,
            Object::parse_literal_string,
            Object::parse_hexadecimal_string,
            Object::parse_numeric,
            Object::parse_bool,
            Object::parse_null,
        ))(input)?;

        Ok((input, value_object))
    }

    // Parse PDF indirect object.
    // This function ignores all comments.
    pub fn parse_body(input: &'a [u8]) -> ParseResult<Object<'a>> {
        let (input, _) = many0(Object::parse_comment)(input)?;
        let (input, value_object) = Object::parse_indirect_object(input)?;

        Ok((input, value_object))
    }

    // pub fn parse(input: &'a [u8]) -> ParseResult<Vec<Object<'a>>> {
    //     let mut result = Vec::new();
    //     let mut remaining = input;

    //     loop {
    //         match Object::parse_one(remaining) {
    //             Ok((input, value_object)) => {
    //                 remaining = input;
    //                 result.push(value_object);
    //             }
    //             Err(_) => break,
    //         }
    //     }

    //     if result.is_empty() {
    //         Err(nom::Err::Error(crate::error::ParseError::InvalidPDFObject))
    //     } else {
    //         Ok((remaining, result))
    //     }
    // }
}

// TODO: implement CrossReferenceTable::parse
impl CrossReferenceTable {
    pub fn parse(input: &[u8]) -> ParseResult<CrossReferenceTable> {
        let (input, (_, _, id, _, count, _, entries)) = tuple((
            tag("xref"),
            newline,
            digit1_u32,
            char(' '),
            digit1_u32,
            newline,
            CrossReferenceEntry::parse_entries,
        ))(input)?;

        if count != entries.len() as u32 {
            Err(nom::Err::Error(
                crate::error::ParseError::InvalidPDFXrefTable,
            ))
        } else {
            Ok((input, CrossReferenceTable { id, count, entries }))
        }
    }
}

impl CrossReferenceEntry {
    pub fn parse(input: &[u8]) -> ParseResult<CrossReferenceEntry> {
        let (input, offset) = digit1_u32_validate_length(input, 10)?;
        let (input, _) = take_while_separator(input)?;
        let (input, generation) = digit1_u32_validate_length(input, 5)?;
        let (input, _) = take_while_separator(input)?;
        let (input, in_use) = alt((char('n'), char('f')))(input)?;
        let (input, _) = take_till_newline(input)?;
        let (input, _) = many_m_n(0, 1, newline)(input)?;

        let free = match in_use {
            'n' => false,
            'f' => true,
            _ => unreachable!(),
        };

        Ok((
            input,
            CrossReferenceEntry {
                offset,
                generation,
                free,
            },
        ))
    }

    pub fn parse_entries(input: &[u8]) -> ParseResult<Vec<CrossReferenceEntry>> {
        let mut entries = Vec::new();
        let mut remaining: &[u8] = input;
        loop {
            // println!(
            //     "before input: {:?}",
            //     std::str::from_utf8(remaining).unwrap()
            // );
            let result = CrossReferenceEntry::parse(remaining);
            if result.is_err() {
                // println!("breaking: {:?}", entries.len());
                break;
            } else {
                let (input, entry) = result.unwrap();
                entries.push(entry);
                remaining = input;
            }
        }

        Ok((remaining, entries))
    }
}

// TODO: implement Trailer::parse
impl<'a> Trailer<'a> {
    pub fn parse(input: &'a [u8]) -> ParseResult<Trailer<'a>> {
        let (input, (_, _, dictionary, _, _, startxref, _, _, _)) = tuple((
            tag("trailer"),
            take_while_separator,
            crate::object::Object::parse_dictionary,
            tag("startxref"),
            take_while_separator,
            digit1_u32,
            take_while_separator,
            tag("%%EOF"),
            take_while_separator,
        ))(input)?;

        debug_assert!(input.is_empty()); // %%EOF should be the last thing in the file

        Ok((
            input,
            Trailer {
                dictionary,
                startxref,
            },
        ))
    }
}
