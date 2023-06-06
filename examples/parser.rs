use std::{fs::File, io::Read};

use pdf_parser::object::PDF;


fn main() {
    // open and read as &[u8] file from argument
    let args: Vec<String> = std::env::args().collect();
    let path = std::path::Path::new(&args[1]);
    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let contents = buffer.as_slice();

    let pdf = PDF::parse(contents).expect("Failed to parse PDF file");
    println!("PDF version: {}.{}", pdf.header.major, pdf.header.minor);
    println!("PDF object count: {:?}", pdf.body.len());
    println!("PDF cross reference table count: {:?}", pdf.cross_reference_tables.len());
    println!("PDF trailer: {:?}", pdf.trailer.dictionary);
    println!("PDF startxref: {:?}", pdf.trailer.startxref);
}