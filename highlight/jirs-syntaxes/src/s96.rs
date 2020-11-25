
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Advanced CSV".to_string(),
  file_extensions: vec!["csv".to_string(),"tsv".to_string()],
  scope: Scope { a: 281861523767296, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_main_0".to_string(), ContextId { index: 6570 });
    v.insert("__start".to_string(), ContextId { index: 6573 });
    v.insert("main".to_string(), ContextId { index: 6574 });
    v.insert("__main".to_string(), ContextId { index: 6572 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 6571 });
    v
  }
} }