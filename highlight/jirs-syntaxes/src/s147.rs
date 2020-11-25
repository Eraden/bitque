
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "resolv".to_string(),
  file_extensions: vec!["resolv.conf".to_string()],
  scope: Scope { a: 845017635618816, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("__main".to_string(), ContextId { index: 8831 });
    v.insert("__start".to_string(), ContextId { index: 8832 });
    v.insert("main".to_string(), ContextId { index: 8833 });
    v
  }
} }