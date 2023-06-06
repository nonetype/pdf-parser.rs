use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

/// The parsed PDF file.
#[derive(Debug, Clone)]
pub struct PDF<'a> {
    pub header: Header,
    pub body: Vec<Object<'a>>,
    pub cross_reference_tables: Vec<CrossReferenceTable>,
    pub trailer: Trailer<'a>,
}

/// The PDF header.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Header {
    /// The major version, usually 1.
    pub major: u32,
    /// The minor version, from 0 to 7.
    pub minor: u32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct CrossReferenceTable {
    pub id: u32,
    pub count: u32,
    pub entries: Vec<CrossReferenceEntry>,
}

/// Represents a cross reference entry.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct CrossReferenceEntry {
    /// 10-digit byte offset in the decoded stream.
    pub offset: u32,
    /// 5-digit generation number.
    pub generation: u32,
    /// Whether the entry is free.
    pub free: bool,
}

/// The PDF trailer.
#[derive(Debug, Hash, Clone)]
pub struct Trailer<'a> {
    pub dictionary: Object<'a>,
    pub startxref: u32,
}

/// An indirect object reference.
/// Represented in PDFs like "12 0 R"
/// TODO: Merge with Object::IndirectReference
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct IndirectReference {
    pub id: u32,
    pub generation: u32,
}

/// A name object.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct NameObject<'a>(pub &'a str);
/// A PDF dictionary object.
pub type DictionaryObject<'a> = HashMap<NameObject<'a>, Object<'a>>;
pub type HexString<'a> = &'a str;

#[derive(Debug, Clone)]
pub enum Object<'a> {
    Boolean(bool),
    Integer(i32),
    Real(f32),
    LiteralString(&'a str),
    HexadecimalString(HexString<'a>),
    Name(NameObject<'a>),
    Array(Vec<Object<'a>>),
    Dictionary(DictionaryObject<'a>, &'a str),
    Stream(DictionaryObject<'a>, &'a [u8]),
    Null,
    Comment(&'a str),
    IndirectReference {
        id: u32,
        generation: u32,
    },
    /// An indirect object definition.
    IndirectObject {
        id: u32,
        generation: u32,
        dictionary: Box<Object<'a>>,
    },
}

impl<'a> Hash for Object<'a> {
    #[allow(unused_must_use)] // TODO: Fix this
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Object::Boolean(b) => b.hash(state),
            Object::Integer(i) => i.hash(state),
            Object::Real(f) => f.to_bits().hash(state),
            Object::LiteralString(s) => s.hash(state),
            Object::HexadecimalString(s) => s.hash(state),
            Object::Name(n) => n.hash(state),
            Object::Array(a) => a.hash(state),
            Object::Dictionary(d, s) => {
                d.iter().map(|(k, v)| {
                    k.hash(state);
                    v.hash(state);
                });
                s.hash(state);
            }
            Object::Stream(d, s) => {
                d.iter().map(|(k, v)| {
                    k.hash(state);
                    v.hash(state);
                });
                s.hash(state);
            }
            Object::Null => state.write_u8(0),
            Object::Comment(s) => s.hash(state),
            Object::IndirectReference { id, generation } => {
                id.hash(state);
                generation.hash(state);
            }
            Object::IndirectObject {
                id,
                generation,
                dictionary,
            } => {
                id.hash(state);
                generation.hash(state);
                dictionary.hash(state);
            }
        }
    }
}
