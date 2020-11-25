
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "SML".to_string(),
  file_extensions: vec!["sml".to_string(),"cm".to_string(),"sig".to_string()],
  scope: Scope { a: 845039110455296, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_main_0".to_string(), ContextId { index: 9164 });
    v.insert("main".to_string(), ContextId { index: 9167 });
    v.insert("__main".to_string(), ContextId { index: 9165 });
    v.insert("__start".to_string(), ContextId { index: 9166 });
    v
  }
} }