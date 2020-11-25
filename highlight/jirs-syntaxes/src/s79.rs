
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Shell-Unix-Generic".to_string(),
  file_extensions: vec![],
  scope: Scope { a: 844742757711872, b: 0 },
  first_line_match: None,
  hidden: true,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("__main".to_string(), ContextId { index: 5462 });
    v.insert("prototype".to_string(), ContextId { index: 5465 });
    v.insert("__start".to_string(), ContextId { index: 5463 });
    v.insert("main".to_string(), ContextId { index: 5464 });
    v
  }
} }