
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "R Console".to_string(),
  file_extensions: vec![],
  scope: Scope { a: 844695513071616, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("__main".to_string(), ContextId { index: 4642 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 4641 });
    v.insert("main".to_string(), ContextId { index: 4644 });
    v.insert("__start".to_string(), ContextId { index: 4643 });
    v
  }
} }