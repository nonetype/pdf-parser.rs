use std::collections::HashMap;

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
    pub entries: Vec<CrossReferenceEntry>,
}

/// Represents a cross reference entry.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct CrossReferenceEntry {
    /// 10-digit byte offset in the decoded stream.
    pub offset: u64,
    /// 5-digit generation number.
    pub generation: u32,
    /// Whether the entry is free.
    pub free: bool,
}

/// The PDF trailer.
#[derive(Debug, Hash, Clone)]
pub struct Trailer<'a> {
    pub size: i64,
    pub prev: Option<IndirectReference>,
    pub root: IndirectReference,
    pub encrypt: Option<IndirectReference>,
    pub info: Option<IndirectReference>,
    pub id: Option<Either<IndirectReference, Vec<HexString<'a>>>>,
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
    Dictionary(DictionaryObject<'a>),
    Stream(DictionaryObject<'a>, &'a [u8]),
    Null,
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
