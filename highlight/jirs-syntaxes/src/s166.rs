
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Highlight non-printables".to_string(),
  file_extensions: vec!["show-nonprintable".to_string()],
  scope: Scope { a: 43910096366862336, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("__main".to_string(), ContextId { index: 10201 });
    v.insert("__start".to_string(), ContextId { index: 10202 });
    v.insert("main".to_string(), ContextId { index: 10203 });
    v
  }
} }