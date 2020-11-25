
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "CMake C++ Header".to_string(),
  file_extensions: vec!["hh.in".to_string(),"hpp.in".to_string(),"hxx.in".to_string(),"h++.in".to_string()],
  scope: Scope { a: 844802888892428, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("identifier".to_string(), "\\b[[:alpha:]_][[:alnum:]_]*\\b".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_main_2".to_string(), ContextId { index: 6145 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 6144 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 6143 });
    v.insert("#anon_main_3".to_string(), ContextId { index: 6146 });
    v.insert("__main".to_string(), ContextId { index: 6147 });
    v.insert("__start".to_string(), ContextId { index: 6148 });
    v.insert("main".to_string(), ContextId { index: 6149 });
    v
  }
} }