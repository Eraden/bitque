use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{BufWriter, Write};
use std::path::Path;

use bincode::deserialize_from;
use bincode::serialize_into;
use bincode::Result;
use flate2::bufread::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub fn dump_to_writer<T: Serialize, W: Write>(to_dump: &T, output: W) -> Result<()> {
    let mut encoder = ZlibEncoder::new(output, Compression::best());
    serialize_into(&mut encoder, to_dump)
}

pub fn dump_binary<T: Serialize>(o: &T) -> Vec<u8> {
    let mut v = Vec::new();
    dump_to_writer(o, &mut v).unwrap();
    v
}

pub fn dump_to_file<T: Serialize, P: AsRef<Path>>(o: &T, path: P) -> Result<()> {
    let out = BufWriter::new(File::create(path)?);
    dump_to_writer(o, out)
}

pub fn from_reader<T: DeserializeOwned, R: BufRead>(input: R) -> Result<T> {
    let mut decoder = ZlibDecoder::new(input);
    deserialize_from(&mut decoder)
}

pub fn from_binary<T: DeserializeOwned>(v: &[u8]) -> T {
    from_reader(v).unwrap()
}

pub fn from_dump_file<T: DeserializeOwned, P: AsRef<Path>>(path: P) -> Result<T> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    from_reader(reader)
}
