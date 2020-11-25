
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Salt State (SLS)".to_string(),
  file_extensions: vec!["sls".to_string()],
  scope: Scope { a: 845034815488000, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("__start".to_string(), ContextId { index: 9162 });
    v.insert("__main".to_string(), ContextId { index: 9161 });
    v.insert("main".to_string(), ContextId { index: 9163 });
    v
  }
} }