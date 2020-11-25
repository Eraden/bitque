
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "orgmode".to_string(),
  file_extensions: vec!["org".to_string()],
  scope: Scope { a: 282029027491840, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_main_9".to_string(), ContextId { index: 8550 });
    v.insert("main".to_string(), ContextId { index: 8556 });
    v.insert("#anon_main_4".to_string(), ContextId { index: 8545 });
    v.insert("#anon_main_7".to_string(), ContextId { index: 8548 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 8536 });
    v.insert("__main".to_string(), ContextId { index: 8551 });
    v.insert("#anon_main_3".to_string(), ContextId { index: 8544 });
    v.insert("heading-inline".to_string(), ContextId { index: 8553 });
    v.insert("#anon_main_14".to_string(), ContextId { index: 8542 });
    v.insert("#anon_main_5".to_string(), ContextId { index: 8546 });
    v.insert("#anon_main_8".to_string(), ContextId { index: 8549 });
    v.insert("#anon_main_12".to_string(), ContextId { index: 8540 });
    v.insert("#anon_main_2".to_string(), ContextId { index: 8543 });
    v.insert("__start".to_string(), ContextId { index: 8552 });
    v.insert("list-inline".to_string(), ContextId { index: 8555 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 8537 });
    v.insert("properties".to_string(), ContextId { index: 8557 });
    v.insert("#anon_main_13".to_string(), ContextId { index: 8541 });
    v.insert("#anon_main_10".to_string(), ContextId { index: 8538 });
    v.insert("#anon_main_6".to_string(), ContextId { index: 8547 });
    v.insert("inline".to_string(), ContextId { index: 8554 });
    v.insert("#anon_main_11".to_string(), ContextId { index: 8539 });
    v
  }
} }