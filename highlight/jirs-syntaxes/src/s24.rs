
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Git Mailmap".to_string(),
  file_extensions: vec![".mailmap".to_string(),"mailmap".to_string()],
  scope: Scope { a: 281565172924416, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("__main".to_string(), ContextId { index: 1823 });
    v.insert("__start".to_string(), ContextId { index: 1824 });
    v.insert("main".to_string(), ContextId { index: 1825 });
    v
  }
} }