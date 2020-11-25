
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "HTML (Tcl)".to_string(),
  file_extensions: vec!["adp".to_string()],
  scope: Scope { a: 281496456658944, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("__start".to_string(), ContextId { index: 5701 });
    v.insert("main".to_string(), ContextId { index: 5702 });
    v.insert("__main".to_string(), ContextId { index: 5700 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 5699 });
    v
  }
} }