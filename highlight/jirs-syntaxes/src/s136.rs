
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Nim".to_string(),
  file_extensions: vec!["nim".to_string(),"nims".to_string()],
  scope: Scope { a: 844970390978560, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_main_2".to_string(), ContextId { index: 8401 });
    v.insert("#anon_main_12".to_string(), ContextId { index: 8399 });
    v.insert("#anon_main_9".to_string(), ContextId { index: 8408 });
    v.insert("__start".to_string(), ContextId { index: 8410 });
    v.insert("__main".to_string(), ContextId { index: 8409 });
    v.insert("#anon_main_6".to_string(), ContextId { index: 8405 });
    v.insert("escaped_char".to_string(), ContextId { index: 8411 });
    v.insert("main".to_string(), ContextId { index: 8412 });
    v.insert("#anon_main_5".to_string(), ContextId { index: 8404 });
    v.insert("#anon_main_13".to_string(), ContextId { index: 8400 });
    v.insert("#anon_main_11".to_string(), ContextId { index: 8398 });
    v.insert("#anon_main_3".to_string(), ContextId { index: 8402 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 8396 });
    v.insert("#anon_main_8".to_string(), ContextId { index: 8407 });
    v.insert("#anon_main_7".to_string(), ContextId { index: 8406 });
    v.insert("#anon_main_10".to_string(), ContextId { index: 8397 });
    v.insert("#anon_main_4".to_string(), ContextId { index: 8403 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 8395 });
    v
  }
} }