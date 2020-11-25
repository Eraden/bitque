
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "NAnt Build File".to_string(),
  file_extensions: vec!["build".to_string()],
  scope: Scope { a: 844467879804928, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("main".to_string(), ContextId { index: 226 });
    v.insert("__main".to_string(), ContextId { index: 224 });
    v.insert("#anon_main_3".to_string(), ContextId { index: 223 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 220 });
    v.insert("__start".to_string(), ContextId { index: 225 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 221 });
    v.insert("#anon_main_2".to_string(), ContextId { index: 222 });
    v
  }
} }