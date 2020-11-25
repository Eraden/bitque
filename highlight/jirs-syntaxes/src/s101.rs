
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Dart Analysis Output".to_string(),
  file_extensions: vec![],
  scope: Scope { a: 281882998603776, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("main".to_string(), ContextId { index: 6687 });
    v.insert("__main".to_string(), ContextId { index: 6685 });
    v.insert("__start".to_string(), ContextId { index: 6686 });
    v
  }
} }