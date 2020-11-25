
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "group".to_string(),
  file_extensions: vec!["group".to_string()],
  scope: Scope { a: 844918851371008, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("password".to_string(), ContextId { index: 7657 });
    v.insert("users".to_string(), ContextId { index: 7658 });
    v.insert("__main".to_string(), ContextId { index: 7653 });
    v.insert("gid".to_string(), ContextId { index: 7655 });
    v.insert("__start".to_string(), ContextId { index: 7654 });
    v.insert("main".to_string(), ContextId { index: 7656 });
    v
  }
} }