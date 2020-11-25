
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "CMake C Header".to_string(),
  file_extensions: vec!["h.in".to_string()],
  scope: Scope { a: 844802888892429, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("identifier".to_string(), "\\b[[:alpha:]_][[:alnum:]_]*\\b".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("main".to_string(), ContextId { index: 6142 });
    v.insert("#anon_main_2".to_string(), ContextId { index: 6138 });
    v.insert("#anon_main_3".to_string(), ContextId { index: 6139 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 6136 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 6137 });
    v.insert("__main".to_string(), ContextId { index: 6140 });
    v.insert("__start".to_string(), ContextId { index: 6141 });
    v
  }
} }