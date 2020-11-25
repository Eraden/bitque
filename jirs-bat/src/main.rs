#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

mod syntect;

use crate::syntect::{dumps::from_binary, highlighting::ThemeSet, parsing::SyntaxSet};
use std::io::Write;

fn main() {
    let set: SyntaxSet = from_binary(include_bytes!(
        "../../highlight/jirs-highlight/src/syntaxes.bin"
    ));
    std::fs::create_dir_all("./highlight/jirs-syntaxes/src").unwrap();
    std::fs::create_dir_all("./highlight/jirs-contexts/src").unwrap();
    std::fs::create_dir_all("./highlight/jirs-themes/src").unwrap();

    create_syntaxes_loader(&set);
    create_contexts_loader(&set);
    create_themes_loader()
}

fn create_syntaxes_loader(set: &SyntaxSet) {
    let lib_path = "./highlight/jirs-syntaxes/src/lib.rs";
    let _ = std::fs::remove_file(lib_path);
    let mut lib = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(lib_path)
        .unwrap();

    lib.write(b"use jirs_syntect::*;\n").unwrap();
    for (idx, syntax) in set.syntaxes.iter().enumerate() {
        println!("  \"{}\",\n", syntax.name);

        lib.write(format!("#[cfg(feature = \"{}\")]\n", syntax.name).as_bytes())
            .unwrap();
        lib.write(format!("pub mod s{};\n", idx).as_bytes())
            .unwrap();
    }
    lib.write(b"pub fn load() -> Vec<SyntaxReference> {\n")
        .unwrap();
    lib.write(b"  vec![\n").unwrap();
    for (idx, syntax) in set.syntaxes.iter().enumerate() {
        lib.write(format!("    #[cfg(feature = \"{}\")]\n", syntax.name).as_bytes())
            .unwrap();
        lib.write(format!("    s{}::load(),\n", idx).as_bytes())
            .unwrap();
    }
    lib.write(b"  ]\n").unwrap();
    lib.write(b"}\n").unwrap();
}

fn create_contexts_loader(set: &SyntaxSet) {
    let context_lib_path = "./highlight/jirs-contexts/src/lib.rs";
    let _ = std::fs::remove_file(context_lib_path);
    let mut contexts_lib = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(context_lib_path)
        .unwrap();

    contexts_lib
        .write(format!("use jirs_syntect::*;\n",).as_bytes())
        .unwrap();

    for (i, context_ref) in set.contexts.iter().enumerate() {
        let path = format!("./highlight/jirs-contexts/src/c{}.rs", i);
        let _ = std::fs::remove_file(path.as_str());
        let mut context_file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .unwrap();
        context_file
            .write(
                r#"
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        "#
                .as_bytes(),
            )
            .unwrap();

        context_file
            .write(format!("{:#?} }}", context_ref).as_bytes())
            .unwrap();
        context_file.flush().unwrap();
        contexts_lib
            .write(format!("pub mod c{};\n", i).as_bytes())
            .unwrap();
    }
    contexts_lib
        .write(b"#[inline(always)]\npub fn load() -> Vec<Context> { vec![\n")
        .unwrap();
    for (i, _context_ref) in set.contexts.iter().enumerate() {
        if i > 0 {
            contexts_lib.write(b",\n").unwrap();
        }
        contexts_lib
            .write(format!("c{}::load()", i).as_bytes())
            .unwrap();
    }
    contexts_lib.write(b"] }").unwrap();
    contexts_lib.flush().unwrap();
}

fn create_themes_loader() {
    let theme_set: ThemeSet = from_binary(include_bytes!(
        "../../highlight/jirs-highlight/src/themes.bin"
    ));
    let _ = std::fs::remove_file("./highlight/jirs-themes/src/lib.rs");
    let mut theme_set_mod = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open("./highlight/jirs-themes/src/lib.rs")
        .unwrap();
    theme_set_mod
        .write(b"use jirs_syntect::highlighting::*;\n")
        .unwrap();
    for (idx, (name, theme)) in theme_set.themes.iter().enumerate() {
        theme_set_mod
            .write(format!("#[cfg(feature = \"{}\")]\n", theme.name.as_ref().unwrap()).as_bytes())
            .unwrap();
        theme_set_mod
            .write(format!("pub mod t{};\n", idx).as_bytes())
            .unwrap();
    }
    theme_set_mod
        .write(b"\npub fn load() -> ThemeSet {\n  let mut ts = ThemeSet::new();\n")
        .unwrap();

    for (idx, (name, theme)) in theme_set.themes.iter().enumerate() {
        let _ = std::fs::remove_file(format!("./highlight/jirs-themes/src/t{}.rs", idx));
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .append(false)
            .open(format!("./highlight/jirs-themes/src/t{}.rs", idx))
            .unwrap();
        f.write(b"use jirs_syntect::*;\n").unwrap();
        f.write(b"use jirs_syntect::highlighting::*;\n").unwrap();
        f.write(b"pub fn load() -> Theme {\n").unwrap();
        f.write(format!("{:#?}", theme).as_bytes()).unwrap();
        f.write(b"}\n").unwrap();
        theme_set_mod
            .write(format!("  #[cfg(feature = \"{}\")]\n", theme.name.as_ref().unwrap()).as_bytes())
            .unwrap();
        theme_set_mod
            .write(
                format!(
                    "  ts.themes.insert(\"{}\".to_string(), t{}::load());\n",
                    name, idx
                )
                .as_bytes(),
            )
            .unwrap();
    }
    theme_set_mod.write(b"  ts\n}\n").unwrap();
}
