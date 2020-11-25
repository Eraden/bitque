#![allow(unused_imports)]
#![allow(dead_code)]

use bincode::{deserialize_from, Result};
use flate2::bufread::ZlibDecoder;
// use jirs_contexts as contexts;
// use jirs_syntaxes as syntaxes;
use jirs_syntect::*;
use jirs_syntect::{highlighting::ThemeSet, parsing::SyntaxSet};
use lazy_static::lazy_static;
use lazycell::AtomicLazyCell;
use serde::de::DeserializeOwned;
use std::io::BufRead;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

lazy_static! {
    pub static ref THEME_SET: ThemeSet = integrated_themeset();
    pub static ref SYNTAX_SET: SyntaxSet = integrated_syntaxset();
}

#[inline(always)]
pub fn load() -> SyntaxSet {
    from_binary(include_bytes!("./syntaxes.bin"))
    // SyntaxSet {
    //     syntaxes: syntaxes::load(),
    //     contexts: contexts::load(),
    // }
}

fn from_reader<T: DeserializeOwned, R: BufRead>(input: R) -> Result<T> {
    let mut decoder = ZlibDecoder::new(input);
    deserialize_from(&mut decoder)
}

fn from_binary<T: DeserializeOwned>(v: &[u8]) -> T {
    from_reader(v).unwrap()
}

#[inline(always)]
fn integrated_syntaxset() -> SyntaxSet {
    load()
}

fn integrated_themeset() -> ThemeSet {
    from_binary(include_bytes!("./themes.bin"))
    // ThemeSet::default()
}

#[wasm_bindgen(start)]
pub fn render() {
    use std::rc::Rc;

    let _ = &THEME_SET;
    let _ = &SYNTAX_SET;

    let window = match web_sys::window() {
        Some(w) => w,
        _ => return,
    };
    let set = Rc::new(load());
    let on_msg = Closure::wrap(Box::new(move |_body| {
        let set_rc = set.clone();
        let _set = set_rc.as_ref();
    }) as Box<dyn Fn(String)>);
    window.set_onmessage(Some(on_msg.as_ref().unchecked_ref()));
}
