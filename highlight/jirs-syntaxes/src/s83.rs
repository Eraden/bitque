
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Textile".to_string(),
  file_extensions: vec!["textile".to_string()],
  scope: Scope { a: 281496456724480, b: 0 },
  first_line_match: Some("textile".to_string()),
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("main".to_string(), ContextId { index: 5769 });
    v.insert("#anon_main_3".to_string(), ContextId { index: 5764 });
    v.insert("#anon_main_2".to_string(), ContextId { index: 5763 });
    v.insert("#anon_main_4".to_string(), ContextId { index: 5765 });
    v.insert("__main".to_string(), ContextId { index: 5766 });
    v.insert("inline".to_string(), ContextId { index: 5768 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 5761 });
    v.insert("__start".to_string(), ContextId { index: 5767 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 5762 });
    v
  }
} }