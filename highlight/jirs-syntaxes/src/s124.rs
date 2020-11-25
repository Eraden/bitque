
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "hosts".to_string(),
  file_extensions: vec!["hosts".to_string()],
  scope: Scope { a: 844927441305600, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("main".to_string(), ContextId { index: 7729 });
    v.insert("__main".to_string(), ContextId { index: 7727 });
    v.insert("__start".to_string(), ContextId { index: 7728 });
    v
  }
} }