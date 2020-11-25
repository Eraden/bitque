
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "JavaScript (Rails)".to_string(),
  file_extensions: vec!["js.erb".to_string()],
  scope: Scope { a: 844609618116608, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("__main".to_string(), ContextId { index: 4706 });
    v.insert("main".to_string(), ContextId { index: 4708 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 4704 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 4705 });
    v.insert("__start".to_string(), ContextId { index: 4707 });
    v
  }
} }