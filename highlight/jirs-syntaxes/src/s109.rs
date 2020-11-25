
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Elm Documentation".to_string(),
  file_extensions: vec![],
  scope: Scope { a: 281496458100838, b: 0 },
  first_line_match: None,
  hidden: true,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("__main".to_string(), ContextId { index: 6941 });
    v.insert("__start".to_string(), ContextId { index: 6942 });
    v.insert("main".to_string(), ContextId { index: 6943 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 6940 });
    v
  }
} }