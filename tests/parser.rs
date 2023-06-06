#[cfg(test)]
mod tests {
    use pdf_parser::object::{NameObject, PDF};
    use std::{fs::File, io::Read, path::PathBuf};

    fn read_testcase(filename: &str) -> Vec<u8> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("assets");
        path.push(filename);
        let mut file = File::open(path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        buffer
    }

    #[test]
    fn test_parse_null() {
        // Test parsing null
        let input_null = b"null";
        let result_null = pdf_parser::object::Object::parse_null(input_null);
        assert!(result_null.is_ok());
        let (input, obj) = result_null.unwrap();
        assert_eq!(input, b""); // should consume input
        match obj {
            pdf_parser::object::Object::Null => (),
            _ => panic!("Expected Object::Null"),
        }
    }

    #[test]
    fn test_parse_nill_remains() {
        // Test parsing null
        let input_null = b"null THIS_STRING_MUST_REMAINED";
        let result_null = pdf_parser::object::Object::parse_null(input_null);
        assert!(result_null.is_ok());
        let (input, obj) = result_null.unwrap();
        assert_eq!(input, b"THIS_STRING_MUST_REMAINED"); // should consume input
        match obj {
            pdf_parser::object::Object::Null => (),
            _ => panic!("Expected Object::Null"),
        }
    }

    #[test]
    fn test_parse_header() {
        let input = b"%PDF-1.7";
        // let result = crate::object::Header::parse(input);
        let result = pdf_parser::object::Header::parse(input);
        assert!(result.is_ok());
        let (input, header) = result.unwrap();
        assert_eq!(input, b""); // should consume input
        assert_eq!(header.major, 1);
        assert_eq!(header.minor, 7);
    }

    #[test]
    fn test_parse_header_remains() {
        let input = b"%PDF-1.7\nTHIS_STRING_MUST_REMAINED";
        // let result = crate::object::Header::parse(input);
        let result = pdf_parser::object::Header::parse(input);
        assert!(result.is_ok());
        let (input, header) = result.unwrap();
        assert_eq!(input, b"THIS_STRING_MUST_REMAINED"); // should consume input
        assert_eq!(header.major, 1);
        assert_eq!(header.minor, 7);
    }

    #[test]
    fn test_parse_bool_true() {
        // Test parsing true
        let input_true = b"true";
        let result_true = pdf_parser::object::Object::parse_bool(input_true);
        assert!(result_true.is_ok());
        let (input, obj) = result_true.unwrap();
        assert_eq!(input, b""); // should consume input
        match obj {
            pdf_parser::object::Object::Boolean(b) => assert_eq!(b, true),
            _ => panic!("Expected Object::Boolean"),
        }
    }

    #[test]
    fn test_parse_bool_false() {
        // Test parsing false
        let input_false = b"false";
        let result_false = pdf_parser::object::Object::parse_bool(input_false);
        assert!(result_false.is_ok());
        let (input, obj) = result_false.unwrap();
        assert_eq!(input, b""); // should consume input
        match obj {
            pdf_parser::object::Object::Boolean(b) => assert_eq!(b, false),
            _ => panic!("Expected Object::Boolean"),
        }
    }

    #[test]
    fn test_parse_bool_invalid() {
        // Test parsing invalid bool
        let input_invalid = b"ffalse";
        let result_invalid = pdf_parser::object::Object::parse_bool(input_invalid);
        assert!(result_invalid.is_err());
        assert_eq!(input_invalid, b"ffalse"); // should not consume input

        let input_invalid = b"truee";
        let result_invalid = pdf_parser::object::Object::parse_bool(input_invalid);
        assert!(result_invalid.is_err());
        assert_eq!(input_invalid, b"truee"); // should not consume input
    }

    #[test]
    fn test_parse_bool_remains() {
        // Test parsing bool with remains
        let input = b"true THIS_STRING_MUST_REMAINED";
        let result = pdf_parser::object::Object::parse_bool(input);
        assert!(result.is_ok());
        let (input, obj) = result.unwrap();
        assert_eq!(input, b"THIS_STRING_MUST_REMAINED"); // should consume input
        match obj {
            pdf_parser::object::Object::Boolean(b) => assert_eq!(b, true),
            _ => panic!("Expected Object::Boolean"),
        }
    }

    #[test]
    fn test_parse_integer_unsigned() {
        // Test parsing integer
        let input = b"123";
        let result = pdf_parser::object::Object::parse_integer(input);
        assert!(result.is_ok());
        let (input, obj) = result.unwrap();
        assert_eq!(input, b""); // should consume input
        match obj {
            pdf_parser::object::Object::Integer(i) => assert_eq!(i, 123),
            _ => panic!("Expected Object::Integer"),
        }
    }

    #[test]
    fn test_parse_integer_negative() {
        // Test parsing negative integer
        let input = b"-123";
        let result = pdf_parser::object::Object::parse_integer(input);
        assert!(result.is_ok());
        let (input, obj) = result.unwrap();
        assert_eq!(input, b""); // should consume input
        match obj {
            pdf_parser::object::Object::Integer(i) => assert_eq!(i, -123),
            _ => panic!("Expected Object::Integer"),
        }
    }

    #[test]
    fn test_parse_integer_positive() {
        // Test parsing positive integer
        let input = b"+123";
        let result = pdf_parser::object::Object::parse_integer(input);
        assert!(result.is_ok());
        let (input, obj) = result.unwrap();
        assert_eq!(input, b""); // should consume input
        match obj {
            pdf_parser::object::Object::Integer(i) => assert_eq!(i, 123),
            _ => panic!("Expected Object::Integer"),
        }
    }

    #[test]
    fn test_parse_integer_invalid() {
        // Test parsing invalid integer
        let input = b"123a";
        let result = pdf_parser::object::Object::parse_integer(input);
        assert!(result.is_err());
        assert_eq!(input, b"123a"); // should not consume input
    }

    #[test]
    fn test_parse_integer_remains() {
        // Test parsing integer with remains
        let input = b"123 THIS_STRING_MUST_REMAINED";
        let result = pdf_parser::object::Object::parse_integer(input);
        assert!(result.is_ok());
        let (input, obj) = result.unwrap();
        assert_eq!(input, b"THIS_STRING_MUST_REMAINED"); // should consume input
        match obj {
            pdf_parser::object::Object::Integer(i) => assert_eq!(i, 123),
            _ => panic!("Expected Object::Integer"),
        }
    }

    #[test]
    fn test_parse_real() {
        // Test parsing real
        let input = b"123.456";
        let result = pdf_parser::object::Object::parse_real(input);
        assert!(result.is_ok());
        let (input, obj) = result.unwrap();
        assert_eq!(input, b""); // should consume input
        match obj {
            pdf_parser::object::Object::Real(r) => assert_eq!(r, 123.456),
            _ => panic!("Expected Object::Real"),
        }
    }

    #[test]
    fn test_parse_real_negative() {
        // Test parsing negative real
        let input = b"-123.456";
        let result = pdf_parser::object::Object::parse_real(input);
        assert!(result.is_ok());
        let (input, obj) = result.unwrap();
        assert_eq!(input, b""); // should consume input
        match obj {
            pdf_parser::object::Object::Real(r) => assert_eq!(r, -123.456),
            _ => panic!("Expected Object::Real"),
        }
    }

    #[test]
    fn test_parse_real_positive() {
        // Test parsing positive real
        let input = b"+123.456";
        let result = pdf_parser::object::Object::parse_real(input);
        assert!(result.is_ok());
        let (input, obj) = result.unwrap();
        assert_eq!(input, b""); // should consume input
        match obj {
            pdf_parser::object::Object::Real(r) => assert_eq!(r, 123.456),
            _ => panic!("Expected Object::Real"),
        }
    }

    #[test]
    fn test_parse_real_invalid() {
        // Test parsing invalid real
        let input = b"123.456a";
        let result = pdf_parser::object::Object::parse_real(input);
        assert!(result.is_err());
        assert_eq!(input, b"123.456a"); // should not consume input

        let input = b"123.456.";
        let result = pdf_parser::object::Object::parse_real(input);
        assert!(result.is_err());
        assert_eq!(input, b"123.456."); // should not consume input
    }

    #[test]
    fn test_parse_real_remains() {
        // Test parsing real with remains
        let input = b"123.456 THIS_STRING_MUST_REMAINED";
        let result = pdf_parser::object::Object::parse_real(input);
        assert!(result.is_ok());
        let (input, obj) = result.unwrap();
        assert_eq!(input, b"THIS_STRING_MUST_REMAINED"); // should consume input
        match obj {
            pdf_parser::object::Object::Real(r) => assert_eq!(r, 123.456),
            _ => panic!("Expected Object::Real"),
        }
    }

    #[test]
    fn test_parse_numeric_integer() {
        // Test parsing integer
        let input = b"123";
        let result = pdf_parser::object::Object::parse_numeric(input);
        assert!(result.is_ok());
        let (input, obj) = result.unwrap();
        assert_eq!(input, b""); // should consume input
        match obj {
            pdf_parser::object::Object::Integer(n) => assert_eq!(n, 123),
            _ => panic!("Expected Object::Numeric"),
        }
    }

    #[test]
    fn test_parse_numeric_real() {
        // Test parsing real
        let input = b"123.456";
        let result = pdf_parser::object::Object::parse_numeric(input);
        assert!(result.is_ok());
        let (input, obj) = result.unwrap();
        assert_eq!(input, b""); // should consume input
        match obj {
            pdf_parser::object::Object::Real(n) => assert_eq!(n, 123.456),
            _ => panic!("Expected Object::Numeric"),
        }
    }

    #[test]
    fn test_parse_literal_string() {
        // Test parsing literal string
        let input = b"(This is a literal string)";
        let result = pdf_parser::object::Object::parse_literal_string(input);
        assert!(result.is_ok());
        let (input, obj) = result.unwrap();
        assert_eq!(input, b""); // should consume input
        match obj {
            pdf_parser::object::Object::LiteralString(s) => {
                assert_eq!(s, "This is a literal string")
            }
            _ => panic!("Expected Object::LiteralString"),
        }
    }

    #[test]
    fn test_parse_literal_string_with_parentheses() {
        // Test parsing literal string with parentheses
        let input = b"(This is a literal string with (parentheses))";
        let result = pdf_parser::object::Object::parse_literal_string(input);
        assert!(result.is_ok());
        let (input, obj) = result.unwrap();
        assert_eq!(input, b""); // should consume input
        match obj {
            pdf_parser::object::Object::LiteralString(s) => {
                assert_eq!(s, "This is a literal string with (parentheses)")
            }
            _ => panic!("Expected Object::LiteralString"),
        }
    }

    // TODO: Check if this result is correct
    #[test]
    fn test_parse_literal_string_with_newline() {
        // Test parsing literal string with newline
        let input = b"(This is a literal string with \nnewline)";
        let result = pdf_parser::object::Object::parse_literal_string(input);
        assert!(result.is_ok());
        let (input, obj) = result.unwrap();
        assert_eq!(input, b""); // should consume input
        match obj {
            pdf_parser::object::Object::LiteralString(s) => {
                assert_eq!(s, "This is a literal string with \nnewline")
            }
            _ => panic!("Expected Object::LiteralString"),
        }
    }

    #[test]
    fn test_parse_literal_string_invalid() {
        // Test parsing invalid literal string
        let input = b"(This is a literal string with (parentheses";
        let result = pdf_parser::object::Object::parse_literal_string(input);
        assert!(result.is_err());
        assert_eq!(input, b"(This is a literal string with (parentheses"); // should not consume input
    }

    #[test]
    fn test_parse_literal_string_remains() {
        // Test parsing literal string with remains
        let input = b"(This is a literal string) THIS_STRING_MUST_REMAINED";
        let result = pdf_parser::object::Object::parse_literal_string(input);
        assert!(result.is_ok());
        let (input, obj) = result.unwrap();
        assert_eq!(input, b"THIS_STRING_MUST_REMAINED"); // should consume input
        match obj {
            pdf_parser::object::Object::LiteralString(s) => {
                assert_eq!(s, "This is a literal string")
            }
            _ => panic!("Expected Object::LiteralString"),
        }
    }

    // TODO: Support literal string with escape
    #[ignore]
    #[test]
    fn test_parse_literal_string_with_escape() {
        // Test parsing literal string with escape
        let input = b"(This is a literal string with \\(escape\\))";
        let result = pdf_parser::object::Object::parse_literal_string(input);
        assert!(result.is_ok());
        let (input, obj) = result.unwrap();
        assert_eq!(input, b""); // should consume input
        match obj {
            pdf_parser::object::Object::LiteralString(s) => {
                assert_eq!(s, "This is a literal string with (escape)")
            }
            _ => panic!("Expected Object::LiteralString"),
        }
    }

    #[test]
    fn test_parse_hexadecimal_string() {
        // Test parsing hexadecimal string
        let input = b"<48656C6C6F>";
        let result = pdf_parser::object::Object::parse_hexadecimal_string(input);
        assert!(result.is_ok());
        let (input, obj) = result.unwrap();
        assert_eq!(input, b""); // should consume input
        match obj {
            pdf_parser::object::Object::HexadecimalString(s) => assert_eq!(s, "48656C6C6F"),
            _ => panic!("Expected Object::HexadecimalString"),
        }
    }

    #[test]
    fn test_parse_hexadecimal_string_with_space() {
        // Test parsing hexadecimal string with space
        let input = b"<48 65 6C 6C 6F>";
        let result = pdf_parser::object::Object::parse_hexadecimal_string(input);
        assert!(result.is_err());
        assert_eq!(input, b"<48 65 6C 6C 6F>"); // should not consume input
    }

    #[test]
    fn test_parse_hexadecimal_string_remains() {
        // Test parsing hexadecimal string with remains
        let input = b"<48656C6C6F> THIS_STRING_MUST_REMAINED";
        let result = pdf_parser::object::Object::parse_hexadecimal_string(input);
        assert!(result.is_ok());
        let (input, obj) = result.unwrap();
        assert_eq!(input, b"THIS_STRING_MUST_REMAINED"); // should consume input
        match obj {
            pdf_parser::object::Object::HexadecimalString(s) => assert_eq!(s, "48656C6C6F"),
            _ => panic!("Expected Object::HexadecimalString"),
        }
    }

    #[test]
    fn test_parse_name() {
        // Test parsing name
        let input = b"/Name";
        let result = pdf_parser::object::Object::parse_name(input);
        assert!(result.is_ok());
        let (input, obj) = result.unwrap();
        assert_eq!(input, b""); // should consume input
        match obj {
            pdf_parser::object::Object::Name(s) => {
                let NameObject(name) = s;
                assert_eq!(name, "Name");
            }
            _ => panic!("Expected Object::Name"),
        }
    }

    #[test]
    fn test_parse_name_with_space() {
        // Test parsing name with space
        let input = b"/Name with space";
        let result = pdf_parser::object::Object::parse_name(input);
        assert!(result.is_ok());
        let (input, obj) = result.unwrap();

        assert_eq!(input, b"with space"); // should not consume input
        match obj {
            pdf_parser::object::Object::Name(s) => {
                let NameObject(name) = s;
                assert_eq!(name, "Name");
            }
            _ => panic!("Expected Object::Name"),
        }
    }

    #[test]
    fn test_parse_name_remains() {
        // Test parsing name with remains
        let input = b"/Name THIS_STRING_MUST_REMAINED";
        let result = pdf_parser::object::Object::parse_name(input);
        assert!(result.is_ok());
        let (input, obj) = result.unwrap();
        assert_eq!(input, b"THIS_STRING_MUST_REMAINED"); // should consume input
        match obj {
            pdf_parser::object::Object::Name(s) => {
                let NameObject(name) = s;
                assert_eq!(name, "Name");
            }
            _ => panic!("Expected Object::Name"),
        }
    }

    #[test]
    fn test_parse_array() {
        // Test parsing array
        let input = b"[1 2 3]";
        let result = pdf_parser::object::Object::parse_array(input);
        assert!(result.is_ok());
        let (input, obj) = result.unwrap();
        assert_eq!(input, b""); // should consume input
        match obj {
            pdf_parser::object::Object::Array(a) => {
                assert_eq!(a.len(), 3);
                if let pdf_parser::object::Object::Integer(i) = a[0] {
                    assert_eq!(i, 1);
                } else {
                    panic!("Expected Object::Integer");
                }

                if let pdf_parser::object::Object::Integer(i) = a[1] {
                    assert_eq!(i, 2);
                } else {
                    panic!("Expected Object::Integer");
                }

                if let pdf_parser::object::Object::Integer(i) = a[2] {
                    assert_eq!(i, 3);
                } else {
                    panic!("Expected Object::Integer");
                }
            }
            _ => panic!("Expected Object::Array"),
        }
    }

    #[test]
    fn test_parse_array_with_space() {
        // Test parsing array with space
        let input = b"[ 1 2 3 ]";
        let result = pdf_parser::object::Object::parse_array(input);
        assert!(result.is_ok());
        let (input, obj) = result.unwrap();
        assert_eq!(input, b""); // should consume input
        match obj {
            pdf_parser::object::Object::Array(a) => {
                assert_eq!(a.len(), 3);
                if let pdf_parser::object::Object::Integer(i) = a[0] {
                    assert_eq!(i, 1);
                } else {
                    panic!("Expected Object::Integer");
                }

                if let pdf_parser::object::Object::Integer(i) = a[1] {
                    assert_eq!(i, 2);
                } else {
                    panic!("Expected Object::Integer");
                }

                if let pdf_parser::object::Object::Integer(i) = a[2] {
                    assert_eq!(i, 3);
                } else {
                    panic!("Expected Object::Integer");
                }
            }
            _ => panic!("Expected Object::Array"),
        }
    }

    #[test]
    fn test_parse_array_with_multiple_types() {
        // Test parsing array
        let input = b"[0 3.14 false (Ralph) /SomeName]";
        let result = pdf_parser::object::Object::parse_array(input);
        assert!(result.is_ok());
        let (input, obj) = result.unwrap();
        assert_eq!(input, b""); // should consume input
        match obj {
            pdf_parser::object::Object::Array(a) => {
                assert_eq!(a.len(), 5);
                if let pdf_parser::object::Object::Integer(i) = a[0] {
                    assert_eq!(i, 0);
                } else {
                    panic!("Expected Object::Integer");
                }

                if let pdf_parser::object::Object::Real(f) = a[1] {
                    assert_eq!(f, 3.14);
                } else {
                    panic!("Expected Object::Real");
                }

                if let pdf_parser::object::Object::Boolean(b) = a[2] {
                    assert_eq!(b, false);
                } else {
                    panic!("Expected Object::Boolean");
                }

                if let pdf_parser::object::Object::LiteralString(s) = a[3] {
                    assert_eq!(s, "Ralph");
                } else {
                    panic!("Expected Object::LiteralString");
                }

                if let pdf_parser::object::Object::Name(no) = a[4] {
                    let NameObject(n) = no;
                    assert_eq!(n, "SomeName");
                } else {
                    panic!("Expected Object::Name");
                }
            }
            _ => panic!("Expected Object::Array"),
        }
    }

    #[test]
    fn test_parse_array_with_nested() {
        // Test parsing array
        let input = b"[0 [1 2 [3 4] 5] 6]";
        let result = pdf_parser::object::Object::parse_array(input);
        assert!(result.is_ok());
        let (input, obj) = result.unwrap();
        assert_eq!(input, b""); // should consume input
        match obj {
            pdf_parser::object::Object::Array(a) => {
                assert_eq!(a.len(), 3);
                if let pdf_parser::object::Object::Integer(i) = a[0] {
                    assert_eq!(i, 0);
                } else {
                    panic!("Expected Object::Integer");
                }

                if let pdf_parser::object::Object::Array(a) = &a[1] {
                    assert_eq!(a.len(), 4);
                    if let pdf_parser::object::Object::Integer(i) = a[0] {
                        assert_eq!(i, 1);
                    } else {
                        panic!("Expected Object::Integer");
                    }

                    if let pdf_parser::object::Object::Integer(i) = a[1] {
                        assert_eq!(i, 2);
                    } else {
                        panic!("Expected Object::Integer");
                    }

                    if let pdf_parser::object::Object::Array(a) = &a[2] {
                        assert_eq!(a.len(), 2);
                        if let pdf_parser::object::Object::Integer(i) = a[0] {
                            assert_eq!(i, 3);
                        } else {
                            panic!("Expected Object::Integer");
                        }

                        if let pdf_parser::object::Object::Integer(i) = a[1] {
                            assert_eq!(i, 4);
                        } else {
                            panic!("Expected Object::Integer");
                        }
                    } else {
                        panic!("Expected Object::Array");
                    }

                    if let pdf_parser::object::Object::Integer(i) = a[3] {
                        assert_eq!(i, 5);
                    } else {
                        panic!("Expected Object::Integer");
                    }
                } else {
                    panic!("Expected Object::Array");
                }

                if let pdf_parser::object::Object::Integer(i) = a[2] {
                    assert_eq!(i, 6);
                } else {
                    panic!("Expected Object::Integer");
                }
            }
            _ => panic!("Expected Object::Array"),
        }
    }

    #[test]
    fn test_parse_dictionary() {
        // Test parsing dictionary
        let input = b"<< /Type /Catalog /S /JavaScript /Subtype /Widget >>";
        let result = pdf_parser::object::Object::parse_dictionary(input);
        assert!(result.is_ok());
        let (input, obj) = result.unwrap();
        assert_eq!(input, b""); // should consume input
        match obj {
            pdf_parser::object::Object::Dictionary(d, s) => {
                assert_eq!(d.len(), 3);
                assert_eq!(s.len(), 0);
                // TODO: test keys and values
            }
            _ => panic!("Expected Object::Dictionary"),
        }
    }

    #[test]
    fn test_parse_indirect_reference() {
        // Test parsing indirect reference
        let input = b"123 0 R";
        let result = pdf_parser::object::Object::parse_indirect_reference(input);
        assert!(result.is_ok());
        let (input, obj) = result.unwrap();
        assert_eq!(input, b""); // should consume input
        match obj {
            pdf_parser::object::Object::IndirectReference { id, generation } => {
                assert_eq!(id, 123);
                assert_eq!(generation, 0);
            }
            _ => panic!("Expected Object::IndirectReference"),
        }
    }

    #[test]
    fn test_parse_indirect_reference_array() {
        // Test parsing indirect reference
        let input = b"[123 0 R]";
        let result = pdf_parser::object::Object::parse_array(input);
        assert!(result.is_ok());
        let (input, obj) = result.unwrap();
        assert_eq!(input, b""); // should consume input
        match obj {
            pdf_parser::object::Object::Array(a) => {
                assert_eq!(a.len(), 1);
                if let pdf_parser::object::Object::IndirectReference { id, generation } = &a[0] {
                    assert_eq!(id, &123);
                    assert_eq!(generation, &0);
                } else {
                    panic!("Expected Object::IndirectReference");
                }
            }
            _ => panic!("Expected Object::Array"),
        }
    }

    #[test]
    fn test_parse_indirect_object() {
        // Test parsing indirect reference
        let input = b"8 0 obj <<
            /Type /Annot
            /Subtype /Widget
            /Parent 6 0 R
            /Rect [400 400 600 600]
        >>
        endobj
        ";
        let result = pdf_parser::object::Object::parse_indirect_object(input);
        assert!(result.is_ok());
        let (input, obj) = result.unwrap();
        assert_eq!(input, b""); // should consume input
        match obj {
            pdf_parser::object::Object::IndirectObject {
                id,
                generation,
                dictionary,
            } => {
                assert_eq!(id, 8);
                assert_eq!(generation, 0);
                let unboxed = *dictionary;
                if let pdf_parser::object::Object::Dictionary(d, s) = unboxed {
                    assert_eq!(d.len(), 4);
                    assert_eq!(s.len(), 0);
                } else {
                    panic!("Expected Object::Dictionary");
                }
            }
            _ => panic!("Expected Object::IndirectReference"),
        }
    }

    #[test]
    fn test_parse_comment() {
        // Test parsing comment
        let input = b"% this is a comment\n";
        let result = pdf_parser::object::Object::parse_comment(input);
        assert!(result.is_ok());
        let (input, obj) = result.unwrap();
        assert_eq!(input, b""); // should consume input
        match obj {
            pdf_parser::object::Object::Comment(s) => {
                assert_eq!(s, " this is a comment");
            }
            _ => panic!("Expected Object::Comment"),
        }
    }

    #[ignore]
    #[test]
    fn test_parse_one() {}

    #[ignore]
    #[test]
    fn test_parse() {}

    #[test]
    fn test_parse_xref_entry_1() {
        // Test parsing xref entry
        let input = b"0000000000 65535 f";
        let result = pdf_parser::object::CrossReferenceEntry::parse(input);
        assert!(result.is_ok());
        let (input, xref_entry) = result.unwrap();
        assert_eq!(input, b""); // should consume input
        assert_eq!(xref_entry.offset, 0);
        assert_eq!(xref_entry.generation, 65535);
        assert_eq!(xref_entry.free, true);
    }

    #[test]
    fn test_parse_xref_entry_2() {
        // Test parsing xref entry
        let input = b"0000000000 65535 f\n";
        let result = pdf_parser::object::CrossReferenceEntry::parse(input);
        assert!(result.is_ok());
        let (input, xref_entry) = result.unwrap();
        assert_eq!(input, b""); // should consume input
        assert_eq!(xref_entry.offset, 0);
        assert_eq!(xref_entry.generation, 65535);
        assert_eq!(xref_entry.free, true);
    }

    #[test]
    fn test_parse_xref_table_1() {
        // Test parsing xref table
        let input = b"xref\n0 1\n0000000000 65535 f\n";
        let result = pdf_parser::object::CrossReferenceTable::parse(input);
        println!("{:?}", result);
        assert!(result.is_ok());
        let (input, xref_table) = result.unwrap();
        assert_eq!(input, b""); // should consume input
        assert_eq!(xref_table.id, 0);
        assert_eq!(xref_table.count, 1);
        assert_eq!(xref_table.entries.len(), 1);
        assert_eq!(xref_table.entries[0].offset, 0);
        assert_eq!(xref_table.entries[0].generation, 65535);
        assert_eq!(xref_table.entries[0].free, true);
    }
    #[test]
    fn test_parse_xref_table_3() {
        // Test parsing xref table
        let input = b"xref\n0 51\n0000000000 65535 f \n0000000015 00000 n \n0000000107 00000 n \n0000000000 65535 f \n0000000414 00000 n \n0000000548 00000 n \n0000000664 00000 n \n0000000744 00000 n \n0000000850 00000 n \n0000000000 65535 f \n0000000959 00000 n \n0000001051 00000 n \n0000001130 00000 n \n0000001222 00000 n \n0000001640 00000 n \n0000001732 00000 n \n0000001784 00000 n \n0000001876 00000 n \n0000000000 65535 f \n0000000000 65535 f \n0000000000 65535 f \n0000000000 65535 f \n0000000000 65535 f \n0000000000 65535 f \n0000000000 65535 f \n0000000000 65535 f \n0000000000 65535 f \n0000000000 65535 f \n0000000000 65535 f \n0000000000 65535 f \n0000000207 00000 n \n0000000326 00000 n \n0000000000 65535 f \n0000000000 65535 f \n0000000000 65535 f \n0000000000 65535 f \n0000000000 65535 f \n0000000000 65535 f \n0000000000 65535 f \n0000000000 65535 f \n0000000000 65535 f \n0000000000 65535 f \n0000000000 65535 f \n0000001916 00000 n \n0000000000 65535 f \n0000000000 65535 f \n0000000000 65535 f \n0000000000 65535 f \n0000000000 65535 f \n0000002040 00000 n \n0000002118 00000 n \ntrailer";
        let result = pdf_parser::object::CrossReferenceTable::parse(input);
        println!("{:?}", result);
        assert!(result.is_ok());
        let (input, xref_table) = result.unwrap();
        assert_eq!(input, b"trailer"); // should consume input
        assert_eq!(xref_table.id, 0);
        assert_eq!(xref_table.count, 51);
        assert_eq!(xref_table.entries.len(), 51);
        assert_eq!(xref_table.entries[0].offset, 0);
        assert_eq!(xref_table.entries[0].generation, 65535);
        assert_eq!(xref_table.entries[0].free, true);
    }

    #[test]
    fn test_parse_xref_table_2() {
        // Test parsing xref table
        let bytes = read_testcase("test_xref");
        // case bytes to &[u8]
        let input = bytes.as_slice();
        let result = pdf_parser::object::CrossReferenceTable::parse(input);
        assert!(result.is_ok());
        let (input, xref_table) = result.unwrap();
        assert_eq!(input, b""); // should consume input
        assert_eq!(xref_table.id, 0);
        assert_eq!(xref_table.count, 51);
        assert_eq!(xref_table.entries.len(), 51);
        assert_eq!(xref_table.entries[0].offset, 0);
        assert_eq!(xref_table.entries[0].generation, 65535);
        assert_eq!(xref_table.entries[0].free, true);
    }

    #[test]
    fn test_parse_trailer() {
        let bytes = read_testcase("test_trailer");
        let input = bytes.as_slice();
        let result = pdf_parser::object::Trailer::parse(input);
        println!("{:?}", result);
        assert!(result.is_ok());
        let (input, trailer) = result.unwrap();
        assert_eq!(input, b""); // should consume input
        assert_eq!(trailer.startxref, 2167);
        match trailer.dictionary {
            pdf_parser::object::Object::Dictionary(d, s) => {
                assert_eq!(d.len(), 1);
                assert_eq!(s.len(), 0);
            }
            _ => panic!("Expected Object::Dictionary"),
        }
    }

    #[test]
    fn test_parse_stream() {
        let bytes = read_testcase("test_stream");
        let input = bytes.as_slice();
        let result = pdf_parser::object::Object::parse_stream(input);
        assert!(result.is_ok());
        let (input, stream) = result.unwrap();
        assert_eq!(input, b""); // should consume input
        let expected = read_testcase("test_stream.expected");
        // expected to &str
        let expected = std::str::from_utf8(expected.as_slice()).unwrap();
        assert_eq!(stream, expected);
    }

    #[test]
    fn test_parse_object_with_stream() {
        let bytes = read_testcase("test_object_with_stream");
        let input = bytes.as_slice();
        let result = pdf_parser::object::Object::parse_indirect_object(input);
        println!("{:?}", result);
        assert!(result.is_ok());
        let (input, obj) = result.unwrap();
        assert_eq!(input, b""); // should consume input
        let expected = read_testcase("test_stream.expected");
        let expected = std::str::from_utf8(expected.as_slice()).unwrap();
        match obj {
            pdf_parser::object::Object::IndirectObject {
                id,
                generation,
                dictionary,
            } => {
                assert_eq!(id, 13);
                assert_eq!(generation, 0);
                let unboxed = *dictionary;
                if let pdf_parser::object::Object::Dictionary(d, s) = unboxed {
                    assert_eq!(d.len(), 0);
                    assert_eq!(s, expected);
                } else {
                    panic!("Expected Object::Dictionary");
                }
            }
            _ => panic!("Expected Object::IndirectReference"),
        }
    }

    #[test]
    fn test_parse_full() {
        let bytes = read_testcase("test.pdf");
        let input = bytes.as_slice();
        let result = PDF::parse(input);
        println!("{:?}", result);
        assert!(result.is_ok());
        let pdf = result.unwrap();
        // Check header
        assert_eq!(pdf.header.major, 1);
        assert_eq!(pdf.header.minor, 7);

        // Check objects
        assert_eq!(pdf.body.len(), 20);

        // Check xref
        let first_xref = pdf.cross_reference_tables.first();
        assert!(first_xref.is_some());
        let first_xref = first_xref.unwrap();
        assert_eq!(first_xref.id, 0);
        assert_eq!(first_xref.count, 51);
        assert_eq!(first_xref.entries.len(), 51);
        // Check first entry
        assert_eq!(first_xref.entries[0].offset, 0);
        assert_eq!(first_xref.entries[0].generation, 65535);
        assert_eq!(first_xref.entries[0].free, true);
        // Check last entry
        assert_eq!(first_xref.entries[50].offset, 2118);
        assert_eq!(first_xref.entries[50].generation, 0);
        assert_eq!(first_xref.entries[50].free, false);

        // Check trailer
        let trailer = pdf.trailer;
        assert_eq!(trailer.startxref, 2167);
        match trailer.dictionary {
            pdf_parser::object::Object::Dictionary(d, s) => {
                assert_eq!(d.len(), 1);
                assert_eq!(s.len(), 0);
            }
            _ => panic!("Expected Object::Dictionary"),
        }
    }
}
