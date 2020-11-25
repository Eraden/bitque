
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "ActionScript".to_string(),
  file_extensions: vec!["as".to_string()],
  scope: Scope { a: 844450700394496, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_main_2".to_string(), ContextId { index: 96 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 95 });
    v.insert("#anon_main_3".to_string(), ContextId { index: 97 });
    v.insert("main".to_string(), ContextId { index: 100 });
    v.insert("__main".to_string(), ContextId { index: 98 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 94 });
    v.insert("__start".to_string(), ContextId { index: 99 });
    v
  }
} }