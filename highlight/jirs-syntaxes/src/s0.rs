
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Plain Text".to_string(),
  file_extensions: vec!["txt".to_string()],
  scope: Scope { a: 281483566645248, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("__start".to_string(), ContextId { index: 1 });
    v.insert("main".to_string(), ContextId { index: 2 });
    v.insert("__main".to_string(), ContextId { index: 0 });
    v
  }
} }