
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "MemInfo".to_string(),
  file_extensions: vec!["meminfo".to_string()],
  scope: Scope { a: 844961801043968, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("__main".to_string(), ContextId { index: 8338 });
    v.insert("main".to_string(), ContextId { index: 8340 });
    v.insert("__start".to_string(), ContextId { index: 8339 });
    v
  }
} }