
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Fortran Namelist".to_string(),
  file_extensions: vec!["namelist".to_string()],
  scope: Scope { a: 844893081567232, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("ident".to_string(), "[A-Za-z_][A-Za-z_0-9]*".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("__start".to_string(), ContextId { index: 7390 });
    v.insert("__main".to_string(), ContextId { index: 7389 });
    v.insert("singlestring".to_string(), ContextId { index: 7394 });
    v.insert("comment".to_string(), ContextId { index: 7391 });
    v.insert("doublestring".to_string(), ContextId { index: 7392 });
    v.insert("main".to_string(), ContextId { index: 7393 });
    v
  }
} }