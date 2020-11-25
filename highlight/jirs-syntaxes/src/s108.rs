
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Elm Compile Messages".to_string(),
  file_extensions: vec![],
  scope: Scope { a: 281496458100837, b: 0 },
  first_line_match: None,
  hidden: true,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("main".to_string(), ContextId { index: 6939 });
    v.insert("__start".to_string(), ContextId { index: 6938 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 6933 });
    v.insert("#anon_main_2".to_string(), ContextId { index: 6934 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 6932 });
    v.insert("__main".to_string(), ContextId { index: 6937 });
    v.insert("#anon_main_4".to_string(), ContextId { index: 6936 });
    v.insert("#anon_main_3".to_string(), ContextId { index: 6935 });
    v
  }
} }