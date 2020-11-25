
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Regular Expressions (Python)".to_string(),
  file_extensions: vec![],
  scope: Scope { a: 844613912756224, b: 0 },
  first_line_match: None,
  hidden: true,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_character-class_0".to_string(), ContextId { index: 4632 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 4634 });
    v.insert("#anon_main_3".to_string(), ContextId { index: 4636 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 4633 });
    v.insert("character-class".to_string(), ContextId { index: 4639 });
    v.insert("#anon_main_2".to_string(), ContextId { index: 4635 });
    v.insert("__start".to_string(), ContextId { index: 4638 });
    v.insert("__main".to_string(), ContextId { index: 4637 });
    v.insert("main".to_string(), ContextId { index: 4640 });
    v
  }
} }