
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "requirements.txt".to_string(),
  file_extensions: vec!["requirements.txt".to_string()],
  scope: Scope { a: 845090650062848, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("__main".to_string(), ContextId { index: 10198 });
    v.insert("__start".to_string(), ContextId { index: 10199 });
    v.insert("main".to_string(), ContextId { index: 10200 });
    v
  }
} }