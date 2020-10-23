use bincode::serialize_into;
use bincode::Result;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use syntect::dumps::from_binary;
use syntect::parsing::SyntaxSet;

pub fn dump_to_writer<T: Serialize, W: Write>(to_dump: &T, output: W) -> Result<()> {
    let mut encoder = ZlibEncoder::new(output, Compression::none());
    serialize_into(&mut encoder, to_dump)
}

pub fn dump_to_file<T: Serialize, P: AsRef<Path>>(to_dump: &T, path: P) -> Result<()> {
    let output = BufWriter::new(File::create(path)?);
    dump_to_writer(to_dump, output)
}

fn main() -> Result<()> {
    let set: SyntaxSet = from_binary(include_bytes!("../../jirs-client/src/hi/syntaxes.bin"));
    dump_to_file(&set, "./tmp/syntaxes.fast.bin")?;
    Ok(())
}
