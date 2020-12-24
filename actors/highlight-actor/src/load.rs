#![allow(unused_imports)]
#![allow(dead_code)]

use std::io::BufRead;
use std::{rc::Rc, sync::Arc};

use bincode::{deserialize_from, Result};
use flate2::bufread::ZlibDecoder;
use serde::de::DeserializeOwned;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

fn from_reader<T: DeserializeOwned, R: BufRead>(input: R) -> Result<T> {
    let mut decoder = ZlibDecoder::new(input);
    deserialize_from(&mut decoder)
}

fn from_binary<T: DeserializeOwned>(v: &[u8]) -> T {
    from_reader(v).unwrap()
}

#[inline(always)]
pub fn integrated_syntaxset() -> syntect::parsing::SyntaxSet {
    from_binary(include_bytes!("./syntaxes.bin"))
}

#[inline(always)]
pub fn integrated_themeset() -> syntect::highlighting::ThemeSet {
    from_binary(include_bytes!("./themes.bin"))
}

pub fn load() -> (Rc<ThemeSet>, Rc<SyntaxSet>) {
    let theme_set = Rc::new(integrated_themeset());
    let syntax_set = Rc::new(integrated_syntaxset());
    (theme_set, syntax_set)
}

pub fn arc() -> (Arc<ThemeSet>, Arc<SyntaxSet>) {
    use std::sync::Arc;

    let theme_set = Arc::new(integrated_themeset());
    let syntax_set = Arc::new(integrated_syntaxset());
    (theme_set, syntax_set)
}
