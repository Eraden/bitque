
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Regular Expressions (PHP)".to_string(),
  file_extensions: vec![],
  scope: Scope { a: 844613912494080, b: 0 },
  first_line_match: None,
  hidden: true,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_main_3".to_string(), ContextId { index: 4169 });
    v.insert("#anon_main_2".to_string(), ContextId { index: 4168 });
    v.insert("__start".to_string(), ContextId { index: 4171 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 4167 });
    v.insert("#anon_character-class_0".to_string(), ContextId { index: 4165 });
    v.insert("character-class".to_string(), ContextId { index: 4172 });
    v.insert("__main".to_string(), ContextId { index: 4170 });
    v.insert("main".to_string(), ContextId { index: 4173 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 4166 });
    v
  }
} }