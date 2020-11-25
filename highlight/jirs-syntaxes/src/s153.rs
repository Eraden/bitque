
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Strace".to_string(),
  file_extensions: vec!["strace".to_string()],
  scope: Scope { a: 845043405422592, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("__main".to_string(), ContextId { index: 9168 });
    v.insert("main".to_string(), ContextId { index: 9170 });
    v.insert("__start".to_string(), ContextId { index: 9169 });
    v.insert("string".to_string(), ContextId { index: 9171 });
    v
  }
} }