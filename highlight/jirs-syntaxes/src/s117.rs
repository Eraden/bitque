
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "GFortran Build Results".to_string(),
  file_extensions: vec![],
  scope: Scope { a: 844897376534528, b: 0 },
  first_line_match: None,
  hidden: true,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("__start".to_string(), ContextId { index: 7396 });
    v.insert("main".to_string(), ContextId { index: 7397 });
    v.insert("__main".to_string(), ContextId { index: 7395 });
    v
  }
} }