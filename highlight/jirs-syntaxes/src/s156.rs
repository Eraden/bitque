
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "syslog".to_string(),
  file_extensions: vec!["log".to_string()],
  scope: Scope { a: 845056290324480, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("main".to_string(), ContextId { index: 9303 });
    v.insert("__start".to_string(), ContextId { index: 9302 });
    v.insert("__main".to_string(), ContextId { index: 9301 });
    v
  }
} }