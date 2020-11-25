
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Cabal".to_string(),
  file_extensions: vec!["cabal".to_string()],
  scope: Scope { a: 844815772155904, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("main".to_string(), ContextId { index: 6579 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 6576 });
    v.insert("module_name".to_string(), ContextId { index: 6580 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 6575 });
    v.insert("__main".to_string(), ContextId { index: 6577 });
    v.insert("__start".to_string(), ContextId { index: 6578 });
    v
  }
} }