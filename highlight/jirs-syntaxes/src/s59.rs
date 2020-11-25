
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Pascal".to_string(),
  file_extensions: vec!["pas".to_string(),"p".to_string(),"dpr".to_string()],
  scope: Scope { a: 844682628169728, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("__start".to_string(), ContextId { index: 4181 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 4175 });
    v.insert("#anon_main_5".to_string(), ContextId { index: 4179 });
    v.insert("__main".to_string(), ContextId { index: 4180 });
    v.insert("main".to_string(), ContextId { index: 4182 });
    v.insert("#anon_main_4".to_string(), ContextId { index: 4178 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 4174 });
    v.insert("#anon_main_2".to_string(), ContextId { index: 4176 });
    v.insert("#anon_main_3".to_string(), ContextId { index: 4177 });
    v
  }
} }