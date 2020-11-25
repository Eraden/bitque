
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Cargo Build Results".to_string(),
  file_extensions: vec![],
  scope: Scope { a: 844729872809984, b: 0 },
  first_line_match: None,
  hidden: true,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("__start".to_string(), ContextId { index: 4960 });
    v.insert("main".to_string(), ContextId { index: 4961 });
    v.insert("__main".to_string(), ContextId { index: 4959 });
    v
  }
} }